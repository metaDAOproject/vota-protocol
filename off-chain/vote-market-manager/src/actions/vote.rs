use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use crate::accounts::resolve::{get_delegate, resolve_vote_keys};
use crate::GAUGEMEISTER;


pub struct WeightInfo {
    pub gauge: Pubkey,
    pub weight: u32,
}


// TODO: payer is only needed until this is wrapped and payed with a PDA
pub fn vote(client: &RpcClient, payer: &Keypair, config: &Pubkey, escrow: &Pubkey, epoch: u32, weights: Vec<WeightInfo>) {
    // Set weights
    for weight in weights {
        let vote_accounts = resolve_vote_keys(&escrow, &weight.gauge, epoch);
        let vote_delegate = get_delegate(config);
        let mut data: Vec<u8> =
            solana_program::hash::hash(b"global:gauge_set_vote").to_bytes()[..8].to_vec();
        data.extend_from_slice(&weight.weight.to_le_bytes());
        let set_weight_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                AccountMeta {
                    pubkey: GAUGEMEISTER,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: weight.gauge,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: vote_accounts.gauge_voter,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: vote_accounts.gauge_vote,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *escrow,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: payer.pubkey(),
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data,
        };
        let mut transaction =
            solana_sdk::transaction::Transaction::new_with_payer(&[set_weight_ix], Some(&payer.pubkey()));
        let latest_blockhash = client.get_latest_blockhash().unwrap();
        transaction.sign(&[payer], latest_blockhash);
        let result = client.send_and_confirm_transaction(&transaction).unwrap();
        println!("result: {:?}", result);
        println!("transaction: {:?}", transaction.signatures.first().unwrap());
    }


    // Commit vote

}