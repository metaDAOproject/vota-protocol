use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use crate::GAUGEMEISTER;

pub(crate) fn trigger_epoch(client: &RpcClient, payer: &Keypair) {

    let mut data: Vec<u8> =
        solana_program::hash::hash(b"global:trigger_next_epoch").to_bytes()[..8].to_vec();
    let close_ix = solana_program::instruction::Instruction {
        program_id: gauge_state::id(),
        accounts: vec![
            AccountMeta {
                pubkey: GAUGEMEISTER,
                is_signer: false,
                is_writable: true,
            },
        ],
        data,
    };
    let mut transaction =
        solana_sdk::transaction::Transaction::new_with_payer(&[close_ix], Some(&payer.pubkey()));
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[payer], latest_blockhash);
    let result = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("result: {:?}", result);
}