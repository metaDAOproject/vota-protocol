use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub fn delegate(client: RpcClient, escrow: &Pubkey, delegate: &Pubkey, owner: &Keypair) {
    let mut data: Vec<u8> =
        solana_program::hash::hash(b"global:set_vote_delegate").to_bytes()[..8].to_vec();
    data.extend_from_slice(&delegate.to_bytes());
    let close_ix = solana_program::instruction::Instruction {
        program_id: locked_voter_state::id(),
        accounts: vec![
            AccountMeta {
                pubkey: *escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: owner.pubkey(),
                is_signer: true,
                is_writable: false,
            },
        ],
        data,
    };
    let mut transaction =
        solana_sdk::transaction::Transaction::new_with_payer(&[close_ix], Some(&owner.pubkey()));
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[owner], latest_blockhash);
    let result = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("result: {:?}", result);
}
