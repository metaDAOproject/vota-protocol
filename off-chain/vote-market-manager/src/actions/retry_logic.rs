use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_program::instruction::Instruction;
use retry::{Error as RetryError, OperationResult};
use solana_sdk::transaction::Transaction;
use retry::delay::Exponential;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_client::rpc_config::RpcSendTransactionConfig;

pub fn retry_logic<'a>(
    client: &'a RpcClient,
    payer: &'a Keypair,
    ixs: &'a mut Vec<Instruction>,
) -> Result<Signature, RetryError<&'a str>> {
    let priority_fee_ix =
        solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(6000);
    // Add the priority fee instruction to the beginning of the transaction
    ixs.insert(0, priority_fee_ix);
    let mut tx = Transaction::new_with_payer(&ixs, Some(&payer.pubkey()));
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer], latest_blockhash);
    // From Helius discord
    //I recommend following these best practices:
    // * using alpha piriorty fee api from Helius to get a more reliable fee
    // * sending transactions with maxRetries=0
    // * polling transactions status with different commitment levels, and keep sending the same signed transaction (with maxRetries=0 and skipPreflight=true) until it gets to confirmed using exponential backoff
    // * aborting if the blockheight goes over the lastValidBlockHeight

    let sim = client.simulate_transaction(&tx).unwrap();
    println!("simulated: {:?}", sim);
    let retry_strategy = Exponential::from_millis(100).take(10);
    let mut signature = Signature::default();
    let mut try_number = 0;
    let result = retry::retry(retry_strategy, || {
        println!("Try number {}", try_number);
        try_number += 1;
        // Check if the blockhash has expired
        if !(client
            .is_blockhash_valid(
                &latest_blockhash,
                CommitmentConfig {
                    commitment: CommitmentLevel::Processed,
                },
            )
            .unwrap())
        {
            println!("Blockhash expired. Checking if it landed");
            let blockhash = client.get_latest_blockhash().unwrap();
            let confirmed_result = client.confirm_transaction_with_spinner(
                &signature,
            &blockhash,
            CommitmentConfig {
                commitment: CommitmentLevel::Finalized,
            });
            match confirmed_result {
                Ok(_confirmed) => {
                    return OperationResult::Ok(signature);
                },
                Err(_e) => {
                    return OperationResult::Err("Failed to try to confirm transaction")
                }
            }
        }
        // Poll to see if processed. First try thruogh it will send but won't check, so always this will
        // need two tries at least.
        if signature != Signature::default() {
            let result = client.get_signature_status_with_commitment(&signature, CommitmentConfig {
                commitment: CommitmentLevel::Processed,
            });
            match result {
                Ok(status) => {
                    if status.is_some() {
                        return OperationResult::Ok(signature);
                    } else {
                        return OperationResult::Retry("Failed to confirm transaction")
                    }
                },
                Err(_e) => {
                    return OperationResult::Retry("Failed to try to confirm transaction")
                }
            }
        }
        let sent = client.send_transaction_with_config(&tx, RpcSendTransactionConfig {
            skip_preflight: true,
            max_retries: Some(0),
            ..RpcSendTransactionConfig::default()
        });
        if let Some(sig) = sent.ok() {
            signature = sig;
        }
        return OperationResult::Retry("Another attempt")
    });
    result
}
