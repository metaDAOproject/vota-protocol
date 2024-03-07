use crate::accounts::resolve::{
    get_delegate, get_epoch_gauge_voter, get_escrow_address_for_owner, get_gauge_voter,
    resolve_vote_keys,
};
use crate::{GAUGEMEISTER, LOCKER};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

pub(crate) fn withdraw_votes(
    client: &RpcClient,
    script_authority: &Keypair,
    owner: Pubkey,
    gauge: Pubkey,
    config: Pubkey,
    epoch: u32,
) {
    let escrow = get_escrow_address_for_owner(&owner);
    let data: Vec<u8> =
        solana_program::hash::hash(b"global:gauge_revert_vote").to_bytes()[..8].to_vec();
    let vote_keys = resolve_vote_keys(&escrow, &gauge, epoch);
    let delegate = get_delegate(&config);
    let create_epoch_gauge_voter_ix = solana_program::instruction::Instruction {
        program_id: gauge_state::id(),
        accounts: vec![
            //Gauge vote account
            AccountMeta::new_readonly(GAUGEMEISTER, false),
            AccountMeta::new_readonly(gauge, false),
            AccountMeta::new_readonly(vote_keys.gauge_voter, false),
            AccountMeta::new_readonly(vote_keys.gauge_vote, false),
            AccountMeta::new(vote_keys.epoch_gauge, false),
            AccountMeta::new(vote_keys.epoch_gauge_voter, false),
            AccountMeta::new(escrow, false),
            AccountMeta::new_readonly(delegate, true),
            AccountMeta::new(vote_keys.epoch_gauge_vote, true),
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
            println!("Epoch gauge vote withdraw")
        }
        Err(e) => {
            log::error!(target: "vote",
            error=e.to_string(),
            user=owner.to_string(),
            epoch=epoch;
            "failed to reset epoch gauge voter");
            println!("Error withdrawing epoch gauge vote: {:?}", e)
        }
    }
}
