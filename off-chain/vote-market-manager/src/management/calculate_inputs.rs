use std::fs;
use solana_client::rpc_client::RpcClient;
use chrono::{Utc};
use solana_program::pubkey::Pubkey;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct EpochStats {
    epoch: u32,
    vote_payment: u64,
    votes: u64,
    expected_emissions: u64,
    emission_token_price: f64,
    gauges: Vec<GaugeStats>,

}

#[derive(Serialize, Debug)]
struct GaugeStats {
    gauge: Pubkey,
    payment: u64,
    votes: u64,
}

pub(crate) fn calculate_inputs(client: &RpcClient) {
    println!("calculate_inputs");
    // mock epoch stats
    let epoch_stats = EpochStats {
        epoch: 1,
        vote_payment: 100,
        votes: 1000,
        expected_emissions: 1000,
        emission_token_price: 1.0,
        gauges: vec![GaugeStats {
            gauge: Pubkey::new_unique(),
            payment: 100,
            votes: 1000,
        }],
    };
    let epoch_stats_json = serde_json::to_string(&epoch_stats).unwrap();
    fs::write(format!("./epoch_stats{}.json", Utc::now().format("%Y-%m-%d-%H_%M" )), epoch_stats_json);
}