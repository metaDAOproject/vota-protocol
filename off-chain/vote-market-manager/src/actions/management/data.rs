use crate::actions::management::oracle::KnownTokens;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct VoteInfo {
    pub buys: u64,
    pub delegated_votes: u64,
    pub direct_votes: u64,
}

#[derive(Serialize, Debug)]
pub struct GaugeVoteInfo {
    // #[serde(
    // deserialize_with = "deserialize_pubkey",
    // serialize_with = "serialize_pubkey"
    // )]
    pub gauge: Pubkey,
    pub info: VoteInfo,
}

#[derive(Serialize, Debug)]
pub struct EpochInput {
    pub epoch: u32,
    pub totals: VoteInfo,
    pub gauges: Vec<GaugeVoteInfo>,
    pub prices: HashMap<KnownTokens, f64>,
}

#[derive(Serialize, Debug)]
pub struct EpochStats {
    pub epoch: u32,
    pub vote_payment: u64,
    pub votes: u64,
    pub expected_emissions: u64,
    pub emission_token_price: f64,
    pub gauges: Vec<GaugeStats>,
}

#[derive(Serialize, Debug)]
pub struct GaugeStats {
    pub gauge: Pubkey,
    pub payment: u64,
    pub votes: u64,
}