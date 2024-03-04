use crate::actions::management::data::{EpochData, VoteInfo};

use crate::actions::vote_market::vote::vote;
use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use crate::accounts::resolve::{get_epoch_gauge_voter, get_escrow_address_for_owner};
use crate::actions::reset_epoch_gauge_voter::reset_epoch_gauge_voter;
use crate::actions::vote_market::clear_votes::clear_votes;

pub(crate) fn execute_votes(
    client: &RpcClient,
    anchor_client: &Client<&Keypair>,
    script_authority: &Keypair,
    data: EpochData,
    vote_weights: Vec<VoteInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Executing votes");
    println!("Data: {:?}", data);
    println!("Vote weights: {:?}", vote_weights);
    for (i, escrow_owner) in data.escrow_owners.iter().enumerate() {
        println!(
            "Voting on behalf of escrow owner {:?}\n Escrow {} out of {}",
            escrow_owner,
            i,
            data.escrow_owners.len()
        );
        clear_votes(anchor_client, client, script_authority, data.config, *escrow_owner)?;
        // Check for epoch_gauge_voter
        let escrow = get_escrow_address_for_owner(&escrow_owner);
        let epoch_gauge_voter = get_epoch_gauge_voter(&escrow, data.epoch);
        let epoch_gauge_voter_account = client.get_account(&epoch_gauge_voter);
        if epoch_gauge_voter_account.is_ok() {
            println!("Epoch gauge voter not found, resetting");
            reset_epoch_gauge_voter(client, script_authority, *escrow_owner, data.epoch);
        }

        let result = vote(
            anchor_client,
            client,
            script_authority,
            data.config,
            *escrow_owner,
            data.epoch,
            vote_weights.clone(),
        );
        match result {
            Ok(_) => println!("Escrow owner: {:?} voted", escrow_owner),
            Err(e) => println!("Error voting for escrow owner: {:?} {:?}", escrow_owner, e),
        }
    }
    Ok(())
}
