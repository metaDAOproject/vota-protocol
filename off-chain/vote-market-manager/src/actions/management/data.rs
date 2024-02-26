use crate::actions::management::oracle::KnownTokens;
use serde::{Deserialize, Serialize};
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
    pub total_votes: u64,
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
    pub usd_per_vote: f64,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteInfo {
    #[serde(
        deserialize_with = "common::deserialize_pubkey",
        serialize_with = "common::serialize_pubkey"
    )]
    pub gauge: Pubkey,
    pub votes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteWeight {
    #[serde(
        deserialize_with = "common::deserialize_pubkey",
        serialize_with = "common::serialize_pubkey"
    )]
    pub gauge: Pubkey,
    pub weight: u32, // Assuming weight is a floating point value for demonstration
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteInfoCollection(pub Vec<VoteInfo>);

impl VoteInfoCollection {
    pub fn new(vote_infos: Vec<VoteInfo>) -> Self {
        VoteInfoCollection(vote_infos)
    }

    pub fn get_infos(&self) -> Vec<VoteInfo> {
        self.0.clone()
    }

    pub fn get_weights(&self) -> Vec<VoteWeight> {
        let min_votes: Vec<u64> = self.0.iter().map(|vote_info| vote_info.votes).collect();

        let multiplier = get_normalize_divider(min_votes);

        // Set the highest vote the highest u32 value possible for best resolution
        self.0
            .iter()
            .map(|vote_info| VoteWeight {
                gauge: vote_info.gauge,
                weight: (vote_info.votes as f64 * multiplier) as u32,
            })
            .collect()
    }
}
pub(crate) fn get_normalize_divider(mut votes: Vec<u64>) -> f64 {
    println!("votes: {:?}", votes);
    let votes_max = *votes.iter().max().unwrap();
    u32::MAX as f64 / votes_max as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(get_normalize_divider(vec![10, 15, 17]), 252645135.0);
        assert_eq!(get_normalize_divider(vec![10]), 429496729.5);
        assert_eq!(get_normalize_divider(vec![8589934590]), 0.5);
    }
}
