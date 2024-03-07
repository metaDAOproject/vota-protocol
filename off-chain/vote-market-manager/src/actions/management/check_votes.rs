use anchor_lang::{AnchorDeserialize, Key};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use crate::accounts::resolve::{get_escrow_address_for_owner, resolve_vote_keys};
use crate::actions::management::utils::get_relevant_gauges;

pub(crate) fn check_votes(client: &RpcClient, owner: &Keypair, epoch: u32) -> Result<(), Box<dyn std::error::Error>> {
    let escrow = get_escrow_address_for_owner(&owner.pubkey());
    let gauges = get_relevant_gauges()?;

    for gauge in gauges {
        let vote_accounts = resolve_vote_keys(&escrow, &gauge, epoch);
        if let Ok(epoch_gauge_vote) = client.get_account(&vote_accounts.epoch_gauge_vote) {
            let epoch_gauge_vote_data = gauge_state::EpochGaugeVote::deserialize(&mut &epoch_gauge_vote.data[8..])?;
            println!("Epoch gauge vote {:?}", epoch_gauge_vote_data);
        }
    }
    Ok(())
}