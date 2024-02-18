use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::actions::management::data::{EpochData, VoteWeight};

pub(crate) fn find_max_vote_buy(data: &mut EpochData, vote_weights: Vec<VoteWeight>) -> Result<(), Box<dyn std::error::Error>> {
    println!("find_max_vote_buy {:#?}", data);
    for gauge in &data.gauges {
        let vote_weight = vote_weights.iter().find(|x| x.gauge == gauge.gauge);
        match vote_weight {
            Some(vote_weight) => {
                let gauge_usd_effect = (vote_weight.votes + gauge.votes) as f64 * data.usd_per_vote;
                println!("gauge_usd_effect: {}", gauge_usd_effect);
                let gauge_efficiency = gauge_usd_effect / gauge.payment;
                println!("gauge_efficiency: {} {}", gauge.gauge, gauge_efficiency);
                println!("vote_weight: {:?}", vote_weight);
            },
            None => {
                println!("Nothing for this gauge");
            }
        }
    }
    Ok(())
}