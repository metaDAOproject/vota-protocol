use anchor_client::Client;
use anchor_lang::AnchorDeserialize;
use reqwest::blocking::ClientBuilder;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use crate::actions::management::data::{EpochData, VoteWeight};
use crate::actions::vote_market::set_maximum::set_maximum;

pub(crate) fn find_max_vote_buy(client: &RpcClient, anchor_client: &Client<&Keypair>, payer: &Keypair, data: &mut EpochData, vote_weights: Vec<VoteWeight>) -> Result<(), Box<dyn std::error::Error>> {
    println!("find_max_vote_buy {:#?}", data);
    for gauge in &data.gauges {
        let vote_weight = vote_weights.iter().find(|x| x.gauge == gauge.gauge);
        let delegated_votes = match vote_weight {
            Some(vote_weight) => {
               vote_weight.votes
            },
            None => {
                0
            }
        };
        let gauge_usd_effect = (gauge.votes + delegated_votes) as f64 * data.usd_per_vote;
        println!("==================================");
        println!("gauge_usd_effect: {}", gauge_usd_effect);
        let gauge_efficiency = gauge_usd_effect / gauge.payment;
        println!("gauge_efficiency: {} {}", gauge.gauge, gauge_efficiency);
        let max_vote_usd =  gauge_usd_effect / 1.2;
        println!("total_gauge_payment: {}", gauge.payment);
        println!("max_vote_usd: {}", max_vote_usd);
        let percentage_to_use = max_vote_usd / gauge.payment;
        println!("percentage_to_use: {}", percentage_to_use);
        let (vote_buy_address, _) = Pubkey::find_program_address(
            &[
                b"vote-buy",
                data.epoch.to_le_bytes().as_ref(),
                data.config.to_bytes().as_ref(),
                gauge.gauge.to_bytes().as_ref(),
            ],
            &vote_market::ID,
        );
        let vote_buy_account = client.get_account(&vote_buy_address).unwrap();
        let vote_buy_data = vote_market::state::VoteBuy::deserialize(&mut vote_buy_account.data[8..].as_ref())?;
        let max_token_amount = vote_buy_data.amount as f64 * percentage_to_use;
        println!("current max_token_amount: {:?}", vote_buy_data.max_amount);
        println!("total_token_amount: {}", vote_buy_data.amount);
        println!("max_token_amount: {}", max_token_amount);
        println!("Setting max token amount for {}", gauge.gauge);
        set_maximum(anchor_client, payer, gauge.gauge, data.config, data.epoch, max_token_amount as u64);
    }
    Ok(())
}