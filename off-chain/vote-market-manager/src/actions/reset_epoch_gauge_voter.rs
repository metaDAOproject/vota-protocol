
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use crate::{GAUGEMEISTER, LOCKER};
use crate::accounts::resolve::{get_epoch_gauge_voter, get_escrow_address_for_owner, get_gauge_voter};

pub(crate) fn reset_epoch_gauge_voter(client: &RpcClient, script_authority: &Keypair, owner: Pubkey, epoch: u32) {

    let escrow = get_escrow_address_for_owner(&owner);
    let gauge_voter = get_gauge_voter(&escrow);
    let epoch_gauge_voter = get_epoch_gauge_voter(&escrow, epoch);
    let data: Vec<u8> =
        solana_program::hash::hash(b"global:reset_epoch_gauge_voter").to_bytes()[..8].to_vec();
    let create_epoch_gauge_voter_ix = solana_program::instruction::Instruction {

    program_id: gauge_state::id(),
    accounts:
    vec![
        //Gauge vote account
        AccountMeta::new_readonly(GAUGEMEISTER, false),
        AccountMeta::new_readonly(LOCKER, false),
        AccountMeta::new_readonly(escrow, false),
        AccountMeta::new_readonly(gauge_voter, false),
        AccountMeta::new(epoch_gauge_voter, false),
    ],
    data,
    };
    let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
        &[create_epoch_gauge_voter_ix],
        Some(&script_authority.pubkey()),
    );
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[script_authority], latest_blockhash);
    let result = client.send_and_confirm_transaction(&transaction);
    match result {
        Ok(sig) => {
            log::info!(target: "vote",
            sig=sig.to_string(),
            user=owner.to_string(),
            epoch=epoch;
            "epoch gauge voter reset"
            );
            println!("Epoch gauge voter reset")
        }
        Err(e) => {
            log::error!(target: "vote",
            error=e.to_string(),
            user=owner.to_string(),
            epoch=epoch;
            "failed to reset epoch gauge voter");
            println!("Error resetting epoch gauge voter: {:?}", e)
        }
    }

}