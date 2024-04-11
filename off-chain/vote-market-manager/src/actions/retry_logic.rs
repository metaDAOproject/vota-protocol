use crate::actions::rpc_retry::retry_rpc;
use retry::delay::Exponential;
use retry::{Error as RetryError, OperationResult};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};
use solana_program::instruction::Instruction;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::transaction::Transaction;

pub fn retry_logic<'a>(
    client: &'a RpcClient,
    payer: &'a Keypair,
    ixs: &'a mut Vec<Instruction>,
    max_cus: Option<u32>,
) -> Result<Signature, RetryError<&'a str>> {
    let jito_client = RpcClient::new("https://mainnet.block-engine.jito.wtf/api/v1/transactions");
    let sim_ixs = ixs.clone();
    let mut sim_tx = Transaction::new_with_payer(&sim_ixs, Some(&payer.pubkey()));
    let (latest_blockhash, _) = retry_rpc(|| {
        client.get_latest_blockhash_with_commitment({
            CommitmentConfig {
                commitment: CommitmentLevel::Confirmed,
            }
        })
    })
    .or_else(|_| {
        Err(RetryError {
            tries: 0,
            total_delay: std::time::Duration::from_millis(0),
            error: "RPC failed to get blockhash",
        })
    })?;
    sim_tx.sign(&[payer], latest_blockhash);
    // From Helius discord
    //I recommend following these best practices:
    // * using alpha piriorty fee api from Helius to get a more reliable fee
    // * sending transactions with maxRetries=0
    // * polling transactions status with different commitment levels, and keep sending the same signed transaction (with maxRetries=0 and skipPreflight=true) until it gets to confirmed using exponential backoff
    // * aborting if the blockheight goes over the lastValidBlockHeight
    // delay for 1 sec to ensure blockhash is found by sim
    std::thread::sleep(std::time::Duration::from_secs(4));
    let sim = retry_rpc(|| {
        client.simulate_transaction_with_config(&sim_tx, {
            RpcSimulateTransactionConfig {
                replace_recent_blockhash: false,
                sig_verify: true,
                commitment: Some(CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                }),
                ..RpcSimulateTransactionConfig::default()
            }
        })
    })
    .or_else(|_| {
        Err(RetryError {
            tries: 0,
            total_delay: std::time::Duration::from_millis(0),
            error: "RPC failed to get sim results",
        })
    })?;
    println!("simulated: {:?}", sim);
    if sim.value.err.is_some() {
        println!("Simulate error: {:?}", sim.value.err.unwrap());
        return Err(RetryError {
            tries: 0,
            total_delay: std::time::Duration::from_millis(0),
            error: "Simulation failed",
        });
    }
    let PRIORITY_FEE = 200_000;
    let priority_fee_ix =
        solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(PRIORITY_FEE);
    // Add the priority fee instruction to the beginning of the transaction
    ixs.insert(0, priority_fee_ix);
    if let Some(cus) = sim.value.units_consumed {
        let max_cus_ix =
            solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(
                (cus as u32) + 1000,
            );
        ixs.insert(0, max_cus_ix);
    }

    let mut tx = Transaction::new_with_payer(&ixs, Some(&payer.pubkey()));
    // let (latest_blockhash, _) = retry_rpc(|| {
    //     client.get_latest_blockhash_with_commitment({
    //         CommitmentConfig {
    //             commitment: CommitmentLevel::Confirmed,
    //         }
    //     })
    // })
    // .or_else(|_| {
    //     Err(RetryError {
    //         tries: 0,
    //         total_delay: std::time::Duration::from_millis(0),
    //         error: "RPC failed to get blockhash",
    //     })
    // })?;
    tx.sign(&[payer], latest_blockhash);
    // Send to jito client
    jito_client.send_transaction_with_config(
        &tx,
        RpcSendTransactionConfig {
            skip_preflight: true,
            max_retries: Some(0),
            ..RpcSendTransactionConfig::default()
        },
    ).unwrap();

    let retry_strategy = Exponential::from_millis(200).take(10);
    let mut signature = Signature::default();
    let mut try_number = 0;
    let result = retry::retry(retry_strategy, || {
        println!("Try number {}", try_number);
        try_number += 1;
        // Check if the blockhash has expired
        let is_valid;
        let is_valid_result = retry_rpc(|| {
            client.is_blockhash_valid(
                &latest_blockhash,
                CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                },
            )
        });
        match is_valid_result {
            Ok(is_valid_value) => {
                is_valid = is_valid_value;
            }
            Err(_) => {
                return OperationResult::Err("RPC Error checking for whether blockhash is valid");
            }
        }
        println!("Is blockhash valid: {:?}", is_valid);
        if !is_valid {
            println!("Blockhash expired. Checking if it landed");
            let confirmed_result = client.confirm_transaction_with_spinner(
                &signature,
                &latest_blockhash,
                CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                },
            );
            match confirmed_result {
                Ok(_confirmed) => {
                    return OperationResult::Ok(signature);
                }
                Err(_e) => return OperationResult::Err("Failed to try to confirm transaction"),
            }
        }
        // Poll to see if processed. First try thruogh it will send but won't check, so always this will
        // need two tries at least.
        if signature != Signature::default() {
            let result = client.get_signature_status_with_commitment(
                &signature,
                CommitmentConfig {
                    commitment: CommitmentLevel::Processed,
                },
            );
            match result {
                Ok(status) => {
                    if status.is_some() {
                        println!("Confirmed. Delaying so next instruction will work");
                        std::thread::sleep(std::time::Duration::from_secs(10));
                        return OperationResult::Ok(signature);
                    }
                }
                Err(_e) => return OperationResult::Retry("Failed to try to confirm transaction"),
            }
        }
        let sent = client.send_transaction_with_config(
            &tx,
            RpcSendTransactionConfig {
                skip_preflight: true,
                max_retries: Some(0),
                ..RpcSendTransactionConfig::default()
            },
        );
        if let Some(sig) = sent.ok() {
            println!("Sent transaction: {:?}", sig);
            signature = sig;
        }
        return OperationResult::Retry("Another attempt");
    });
    result
}
