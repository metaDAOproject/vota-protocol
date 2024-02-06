use crate::actions::management::oracle::KnownTokens;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct EpochInput {
    pub epoch: u32,
    pub direct_votes: u64,
    pub delegated_votes: u64,
    pub total_vote_buy_value: f64,
    pub gauges: Vec<GaugeInfo>,
    pub prices: HashMap<KnownTokens, f64>,
    #[serde(
        deserialize_with = "common::deserialize_pubkey_vec",
        serialize_with = "common::serialize_pubkey_vec"
    )]
    pub escrows: Vec<Pubkey>,
}

#[derive(Serialize, Debug)]
pub struct GaugeInfo {
    #[serde(
        deserialize_with = "common::deserialize_pubkey",
        serialize_with = "common::serialize_pubkey"
    )]
    pub gauge: Pubkey,
    pub payment: f64,
    pub votes: u64,
}
