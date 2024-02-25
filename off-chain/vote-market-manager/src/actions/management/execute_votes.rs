use crate::actions::management::data::{EpochData, VoteWeight};
use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use crate::accounts::resolve::get_delegate;
use crate::actions::queries::escrows::get_delegated_escrows;
use crate::actions::vote_market::vote::vote;

pub(crate) fn execute_votes(
    client: &RpcClient,
    anchor_client: &Client<&Keypair>,
    script_authority: &Keypair,
    data: EpochData,
    vote_weights: Vec<VoteWeight>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Executing votes");
    println!("Data: {:?}", data);
    println!("Vote weights: {:?}", vote_weights);
    let delegate = get_delegate(&data.config);
    let escrows = get_delegated_escrows(client, &delegate);
    for (escrow, _) in escrows {
        println!("Voting on behalf of escrow {:?}", escrow);
        let result = vote(anchor_client, client, script_authority, data.config, escrow, data.epoch, vote_weights.clone());
        match result {
            Ok(_) => println!("Escrow: {:?} voted", escrow),
            Err(e) => println!("Error voting for escrow: {:?} {:?}", escrow, e),
        }
    }

    Ok(())
}
