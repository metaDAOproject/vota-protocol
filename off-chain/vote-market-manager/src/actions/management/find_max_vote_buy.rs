use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::actions::management::data::{EpochData, VoteWeight};

pub(crate) fn find_max_vote_buy(data: &mut EpochData, vote_weights: Vec<VoteWeight>) -> Result<(), Box<dyn std::error::Error>> {
    println!("find_max_vote_buy {:#?}", data);
   // data.usd_per_vote;
   // data.gauges
    Ok(())
}