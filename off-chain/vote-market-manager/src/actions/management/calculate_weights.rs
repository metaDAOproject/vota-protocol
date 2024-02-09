use std::collections::HashMap;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use gauge_state::EpochGauge;
use crate::actions::management::data::{EpochData, GaugeInfo, VoteWeight};

pub(crate) fn calculate_weights(data: &EpochData)
    -> Result<Vec<VoteWeight>, Box<dyn std::error::Error>> {
    println!("calculate_weights {:?}", data);
    let mut vote_weights: Vec<VoteWeight> = vec![];
    for gauge_data in data.gauges.iter() {
        if gauge_data.payment == 0.0 {
            continue;
        }
        let algorithmic_votes = (gauge_data.payment * (data.direct_votes as f64 + data.delegated_votes as f64) / data.total_vote_buy_value) - gauge_data.votes as f64;

        vote_weights.push(VoteWeight {
            gauge: gauge_data.gauge,
            votes: algorithmic_votes.round() as u64,
        });
        println!("Algorithmic votes for {}: {}", gauge_data.gauge, algorithmic_votes);
    }
    Ok(vote_weights)
}

#[cfg(test)]
mod test_calculate_weight {
    use crate::actions::management::oracle::KnownTokens;
    use super::*;
    #[test]
    fn test_even_distribution() {
        let gauge1 = Pubkey::new_unique();
        let gauge2 = Pubkey::new_unique();
        let gauge3 = Pubkey::new_unique();
        let data = EpochData {
            config: Pubkey::new_unique(),
            epoch: 1,
            direct_votes: 0,
            delegated_votes: 1_000_000,
            total_vote_buy_value: 300.0,
            gauges: vec![
                GaugeInfo {
                    gauge: gauge1,
                    payment: 100.0,
                    votes: 0,
                },
                GaugeInfo {
                    gauge: gauge2,
                    payment: 100.0,
                    votes: 0,
                },
                GaugeInfo {
                    gauge: gauge3,
                    payment: 100.0,
                    votes: 0,
                }
            ],
            prices: HashMap::from([
                  (KnownTokens::mSOL, 120.56),
                  (KnownTokens::UXD, 0.991553),
                  (KnownTokens::SBR, 0.00286583),
                  (KnownTokens::BLZE, 0.00311461),
            ]),
            escrows: vec![],
        };
        let weights = calculate_weights(&data).unwrap();
        assert_eq!(weights.len(), 3);
        assert_eq!(weights[0].votes, (((data.direct_votes + data.delegated_votes) as f64)/3.0).round() as u64);
    }
    #[test]
    fn test_uneven_distribution() {
        let gauge1 = Pubkey::new_unique();
        let gauge2 = Pubkey::new_unique();
        let gauge3 = Pubkey::new_unique();
        let data = EpochData {
            config: Pubkey::new_unique(),
            epoch: 1,
            direct_votes: 0,
            delegated_votes: 600_000,
            total_vote_buy_value: 60.0,
            gauges: vec![
                GaugeInfo {
                    gauge: gauge1,
                    payment: 10.0,
                    votes: 0,
                },
                GaugeInfo {
                    gauge: gauge2,
                    payment: 20.0,
                    votes: 0,
                },
                GaugeInfo {
                    gauge: gauge3,
                    payment: 30.0,
                    votes: 0,
                }
            ],
            prices: HashMap::from([
                (KnownTokens::mSOL, 120.56),
                (KnownTokens::UXD, 0.991553),
                (KnownTokens::SBR, 0.00286583),
                (KnownTokens::BLZE, 0.00311461),
            ]),
            escrows: vec![],
        };
        let weights = calculate_weights(&data).unwrap();
        assert_eq!(weights[0].votes, 100_000);
        assert_eq!(weights[1].votes, 200_000);
        assert_eq!(weights[2].votes, 300_000);
    }

    #[test]
    fn test_with_direct_votes() {
        let gauge1 = Pubkey::new_unique();
        let gauge2 = Pubkey::new_unique();
        let gauge3 = Pubkey::new_unique();
        let data = EpochData {
            config: Pubkey::new_unique(),
            epoch: 1,
            direct_votes: 500_000,
            delegated_votes: 1_000_000,
            total_vote_buy_value: 300.0,
            gauges: vec![
                GaugeInfo {
                    gauge: gauge1,
                    payment: 100.0,
                    votes: 500_000,
                },
                GaugeInfo {
                    gauge: gauge2,
                    payment: 100.0,
                    votes: 0,
                },
                GaugeInfo {
                    gauge: gauge3,
                    payment: 100.0,
                    votes: 0,
                }
            ],
            prices: HashMap::from([
                (KnownTokens::mSOL, 120.56),
                (KnownTokens::UXD, 0.991553),
                (KnownTokens::SBR, 0.00286583),
                (KnownTokens::BLZE, 0.00311461),
            ]),
            escrows: vec![],
        };
        let weights = calculate_weights(&data).unwrap();
        assert_eq!(weights.len(), 3);
        assert_eq!(weights[0].votes, 0);
        assert_eq!(weights[1].votes, 500_000);
        assert_eq!(weights[2].votes, 500_000);
    }
    #[test]
    fn test_with_multiple_direct_votes() {
        let gauge1 = Pubkey::new_unique();
        let gauge2 = Pubkey::new_unique();
        let gauge3 = Pubkey::new_unique();
        let data = EpochData {
            config: Pubkey::new_unique(),
            epoch: 1,
            //
            direct_votes: 60,
            delegated_votes: 300_000 - 60,
            total_vote_buy_value: 300.0,
            gauges: vec![
                GaugeInfo {
                    gauge: gauge1,
                    payment: 100.0,
                    votes: 10,
                },
                GaugeInfo {
                    gauge: gauge2,
                    payment: 100.0,
                    votes: 20,
                },
                GaugeInfo {
                    gauge: gauge3,
                    payment: 100.0,
                    votes: 30,
                }
            ],
            prices: HashMap::from([
                (KnownTokens::mSOL, 120.56),
                (KnownTokens::UXD, 0.991553),
                (KnownTokens::SBR, 0.00286583),
                (KnownTokens::BLZE, 0.00311461),
            ]),
            escrows: vec![],
        };
        let weights = calculate_weights(&data).unwrap();
        assert_eq!(weights.len(), 3);
        // Should make them even when the direct votes are included
        assert_eq!(weights[0].votes, 100_000 - 10);
        assert_eq!(weights[1].votes, 100_000 - 20);
        assert_eq!(weights[2].votes, 100_000 - 30);
    }
}