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
        solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(100000);
    ixs.push(priority_fee_ix);
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
    let retry_strategy = Exponential::from_millis(10).take(5);
    let result = retry::retry(retry_strategy, || {
        if !(client
            .is_blockhash_valid(
                &latest_blockhash,
                CommitmentConfig {
                    commitment: CommitmentLevel::Processed,
                },
            )
            .unwrap())
        {
            return OperationResult::Err("Blockhash Expired");
        }
        let result = client.send_and_confirm_transaction_with_spinner_and_config(
            &tx,
            CommitmentConfig {
                commitment: CommitmentLevel::Processed,
            },
            RpcSendTransactionConfig {
                skip_preflight: true,
                max_retries: Some(0),
                ..RpcSendTransactionConfig::default()
            },
        );
        return match result {
            Ok(sig) => OperationResult::Ok(sig),
            Err(e) => OperationResult::Retry("Failed to send transaction"),
        };
    });
    result
}
