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
        program_id: saber_locker::id(),
        accounts: vec![
            AccountMeta::new(*escrow, false),
            AccountMeta::new_readonly(owner.pubkey(), true),
        ],
        data,
    };
    let mut transaction =
        solana_sdk::transaction::Transaction::new_with_payer(&[close_ix], Some(&owner.pubkey()));
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[owner], latest_blockhash);
    let result = client.send_and_confirm_transaction(&transaction);
    match result {
        Ok(sig) => {
            log::info!(target: "vote",
            sig=sig.to_string(),
            user=owner.pubkey().to_string(),
            delegate=delegate.to_string();
            "vote delegate set"
            );
            println!("delegate: {:?}", delegate);
        }
        Err(e) => {
            log::error!(target: "vote",
            error=e.to_string(),
            user=owner.pubkey().to_string(),
            delegate=delegate.to_string();
            "failed to set vote delegate"
            );
            println!("Error setting vote delegate: {:?}", e);
        }
    }
}
