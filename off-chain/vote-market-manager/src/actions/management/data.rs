use crate::actions::management::oracle::KnownTokens;
use serde::{Serialize, Deserialize};
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochData {
    #[serde(
    deserialize_with = "common::deserialize_pubkey",
    serialize_with = "common::serialize_pubkey"
    )]
    pub config: Pubkey,
    pub epoch: u32,
    pub direct_votes: u64,
    pub delegated_votes: u64,
    pub total_vote_buy_value: f64,
    pub gauges: Vec<GaugeInfo>,
    pub prices: HashMap<KnownTokens, f64>,
    pub sbr_per_epoch: u64,
    #[serde(
        deserialize_with = "common::deserialize_pubkey_vec",
        serialize_with = "common::serialize_pubkey_vec"
    )]
    pub escrows: Vec<Pubkey>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GaugeInfo {
    #[serde(
        deserialize_with = "common::deserialize_pubkey",
        serialize_with = "common::serialize_pubkey"
    )]
    pub gauge: Pubkey,
    pub payment: f64,
    pub votes: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VoteWeight {
    #[serde(
    deserialize_with = "common::deserialize_pubkey",
    serialize_with = "common::serialize_pubkey"
    )]
    pub gauge: Pubkey,
    pub votes: u64,
}


