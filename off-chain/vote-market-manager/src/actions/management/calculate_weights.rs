use crate::actions::management::data::{
    EpochData,
};



pub(crate) fn calculate_weights(_data: &mut EpochData) -> Result<(), Box<dyn std::error::Error>> {
    // let weights = weight_calc(data)?;
    // println!("Weights1: {:?}", weights);
    //
    // let vic = VoteInfoCollection(weights);
    // print!("Weights: {:?}", vic.get_weights());
    // println!("Infos: {:?}", vic.get_infos());
    // let vote_weights_json = serde_json::to_string(&vic).unwrap();
    // fs::write(
    //     format!(
    //         "./epoch_{}_weights{}.json",
    //         data.epoch,
    //         Utc::now().format("%Y-%m-%d-%H_%M")
    //     ),
    //     vote_weights_json,
    // )?;
     Ok(())
}

// pub(crate) fn sort_gauges(gauges: &mut [GaugeInfo]) {
//     gauges.sort_by(|a, b| {
//         //change all zeros to very small numbers to get the correct order and
//         //avoid divide by zero
//         let votes_a = if a.votes == 0 {
//             0.0000000001
//         } else {
//             a.votes as f64
//         };
//         let votes_b = if b.votes == 0 {
//             0.0000000001
//         } else {
//             b.votes as f64
//         };
//         let payment_a = if a.payment == 0.0 {
//             0.0000000001
//         } else {
//             a.payment
//         };
//         let payment_b = if b.payment == 0.0 {
//             0.0000000001
//         } else {
//             b.payment
//         };
//         let cmp_value_a = votes_a / payment_a;
//         let cmp_value_b = votes_b / payment_b;
//         cmp_value_b.partial_cmp(&cmp_value_a).unwrap()
//     });
// }
//
// pub fn weight_calc(data: &EpochData) -> Result<Vec<VoteInfo>, Box<dyn std::error::Error>> {
//     let mut input_data: EpochData = data.clone();
//     let mut pass = weight_calc_pass(&input_data).unwrap();
//     sort_gauges(&mut input_data.gauges);
//     input_data.gauges.retain(|x| x.payment > 0.0);
//     while !pass.is_empty() && pass[0].votes == 0 {
//         input_data.direct_votes -= input_data.gauges[0].votes;
//         input_data.total_vote_buy_value -= input_data.gauges[0].payment;
//         input_data.gauges.remove(0);
//         sort_gauges(&mut input_data.gauges);
//         pass = weight_calc_pass(&input_data).unwrap();
//     }
//     Ok(pass)
// }
//
// pub fn weight_calc_pass(data: &EpochData) -> Result<Vec<VoteInfo>, Box<dyn std::error::Error>> {
//     let mut vote_weights: Vec<VoteInfo> = vec![];
//     for gauge_data in data.gauges.iter() {
//         if gauge_data.payment == 0.0 {
//             continue;
//         }
//         let algorithmic_votes = (gauge_data.payment
//             * (data.direct_votes as f64 + data.delegated_votes as f64)
//             / data.total_vote_buy_value)
//             - gauge_data.votes as f64;
//
//         vote_weights.push(VoteInfo {
//             gauge: gauge_data.gauge,
//             votes: algorithmic_votes.round() as u64,
//             weight: 0,
//         });
//         println!(
//             "Algorithmic votes for {}: {}",
//             gauge_data.gauge, algorithmic_votes
//         );
//     }
//     Ok(vote_weights)
// }

// #[cfg(test)]
// mod test_calculate_weight {
//     use super::*;
//     use crate::actions::management::oracle::KnownTokens;
//     use solana_program::pubkey::Pubkey;
//     use std::collections::HashMap;
//     #[test]
//     fn test_even_distribution() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let data = EpochData {
//             config: Pubkey::new_unique(),
//             epoch: 1,
//             direct_votes: 0,
//             total_votes: 0,
//             delegated_votes: 1_000_000,
//             total_vote_buy_value: 300.0,
//             gauges: vec![
//                 GaugeInfo {
//                     gauge: gauge1,
//                     payment: 100.0,
//                     votes: 0,
//                 },
//                 GaugeInfo {
//                     gauge: gauge2,
//                     payment: 100.0,
//                     votes: 0,
//                 },
//                 GaugeInfo {
//                     gauge: gauge3,
//                     payment: 100.0,
//                     votes: 0,
//                 },
//             ],
//             prices: HashMap::from([
//                 (KnownTokens::Msol, 120.56),
//                 (KnownTokens::Uxd, 0.991553),
//                 (KnownTokens::Sbr, 0.00286583),
//                 (KnownTokens::Blze, 0.00311461),
//             ]),
//             escrows: vec![],
//             sbr_per_epoch: 0,
//             usd_per_vote: 0.0,
//         };
//         let weights = weight_calc_pass(&data).unwrap();
//         assert_eq!(weights.len(), 3);
//         assert_eq!(
//             weights[0].votes,
//             (((data.direct_votes + data.delegated_votes) as f64) / 3.0).round() as u64
//         );
//     }
//     #[test]
//     fn test_uneven_distribution() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let data = EpochData {
//             config: Pubkey::new_unique(),
//             epoch: 1,
//             direct_votes: 0,
//             total_votes: 0,
//             delegated_votes: 600_000,
//             total_vote_buy_value: 60.0,
//             gauges: vec![
//                 GaugeInfo {
//                     gauge: gauge1,
//                     payment: 10.0,
//                     votes: 0,
//                 },
//                 GaugeInfo {
//                     gauge: gauge2,
//                     payment: 20.0,
//                     votes: 0,
//                 },
//                 GaugeInfo {
//                     gauge: gauge3,
//                     payment: 30.0,
//                     votes: 0,
//                 },
//             ],
//             prices: HashMap::from([
//                 (KnownTokens::Msol, 120.56),
//                 (KnownTokens::Uxd, 0.991553),
//                 (KnownTokens::Sbr, 0.00286583),
//                 (KnownTokens::Blze, 0.00311461),
//             ]),
//             escrows: vec![],
//             sbr_per_epoch: 0,
//             usd_per_vote: 0.0,
//         };
//         let weights = weight_calc_pass(&data).unwrap();
//         assert_eq!(weights[0].votes, 100_000);
//         assert_eq!(weights[1].votes, 200_000);
//         assert_eq!(weights[2].votes, 300_000);
//     }
//
//     #[test]
//     fn test_with_direct_votes() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let data = EpochData {
//             config: Pubkey::new_unique(),
//             epoch: 1,
//             direct_votes: 500_000,
//             total_votes: 500_000,
//             delegated_votes: 1_000_000,
//             total_vote_buy_value: 300.0,
//             gauges: vec![
//                 GaugeInfo {
//                     gauge: gauge1,
//                     payment: 100.0,
//                     votes: 500_000,
//                 },
//                 GaugeInfo {
//                     gauge: gauge2,
//                     payment: 100.0,
//                     votes: 0,
//                 },
//                 GaugeInfo {
//                     gauge: gauge3,
//                     payment: 100.0,
//                     votes: 0,
//                 },
//             ],
//             prices: HashMap::from([
//                 (KnownTokens::Msol, 120.56),
//                 (KnownTokens::Uxd, 0.991553),
//                 (KnownTokens::Sbr, 0.00286583),
//                 (KnownTokens::Blze, 0.00311461),
//             ]),
//             escrows: vec![],
//             sbr_per_epoch: 0,
//             usd_per_vote: 0.0,
//         };
//         let weights = weight_calc_pass(&data).unwrap();
//         assert_eq!(weights.len(), 3);
//         assert_eq!(weights[0].votes, 0);
//         assert_eq!(weights[1].votes, 500_000);
//         assert_eq!(weights[2].votes, 500_000);
//     }
//     #[test]
//     fn test_with_multiple_direct_votes() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let data = EpochData {
//             config: Pubkey::new_unique(),
//             epoch: 1,
//             //
//             direct_votes: 60,
//             total_votes: 60,
//             delegated_votes: 300_000 - 60,
//             total_vote_buy_value: 300.0,
//             gauges: vec![
//                 GaugeInfo {
//                     gauge: gauge1,
//                     payment: 100.0,
//                     votes: 10,
//                 },
//                 GaugeInfo {
//                     gauge: gauge2,
//                     payment: 100.0,
//                     votes: 20,
//                 },
//                 GaugeInfo {
//                     gauge: gauge3,
//                     payment: 100.0,
//                     votes: 30,
//                 },
//             ],
//             prices: HashMap::from([
//                 (KnownTokens::Msol, 120.56),
//                 (KnownTokens::Uxd, 0.991553),
//                 (KnownTokens::Sbr, 0.00286583),
//                 (KnownTokens::Blze, 0.00311461),
//             ]),
//             escrows: vec![],
//             sbr_per_epoch: 0,
//             usd_per_vote: 0.0,
//         };
//         let weights = weight_calc_pass(&data).unwrap();
//         assert_eq!(weights.len(), 3);
//         // Should make them even when the direct votes are included
//         assert_eq!(weights[0].votes, 100_000 - 10);
//         assert_eq!(weights[1].votes, 100_000 - 20);
//         assert_eq!(weights[2].votes, 100_000 - 30);
//     }
//     #[test]
//     pub fn test_sort_gauges_zero_votes() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let mut gauges = vec![
//             GaugeInfo {
//                 gauge: gauge1,
//                 payment: 200.0,
//                 votes: 10,
//             },
//             GaugeInfo {
//                 gauge: gauge2,
//                 payment: 500.0,
//                 votes: 20,
//             },
//             GaugeInfo {
//                 gauge: gauge3,
//                 payment: 700.0,
//                 votes: 30,
//             },
//         ];
//         sort_gauges(&mut gauges);
//         // Want to eliminate the lowest vote buys first in this case
//         assert_eq!(gauges[0].payment, 200.0);
//         assert_eq!(gauges[1].payment, 700.0);
//         assert_eq!(gauges[2].payment, 500.0);
//     }
//     #[test]
//     pub fn test_sort_gauges() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let mut gauges = vec![
//             GaugeInfo {
//                 gauge: gauge1,
//                 payment: 200.0,
//                 votes: 0,
//             },
//             GaugeInfo {
//                 gauge: gauge2,
//                 payment: 100.0,
//                 votes: 0,
//             },
//             GaugeInfo {
//                 gauge: gauge3,
//                 payment: 300.0,
//                 votes: 0,
//             },
//         ];
//         sort_gauges(&mut gauges);
//         // Want to eliminate the lowest vote buys first in this case
//         assert_eq!(gauges[0].payment, 100.0);
//     }
//     pub fn test_sort_gauges_divide_by_zero() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let mut gauges = vec![
//             GaugeInfo {
//                 gauge: gauge1,
//                 payment: 0.0,
//                 votes: 0,
//             },
//             GaugeInfo {
//                 gauge: gauge2,
//                 payment: 200.0,
//                 votes: 0,
//             },
//             GaugeInfo {
//                 gauge: gauge3,
//                 payment: 300.0,
//                 votes: 0,
//             },
//         ];
//         sort_gauges(&mut gauges);
//         assert_eq!(gauges[0].payment, 300.0);
//     }
//     #[test]
//     pub fn test_remove_negative() {
//         let gauge1 = Pubkey::new_unique();
//         let gauge2 = Pubkey::new_unique();
//         let gauge3 = Pubkey::new_unique();
//         let data = EpochData {
//             config: Pubkey::new_unique(),
//             epoch: 1,
//             //
//             direct_votes: 100_002_000,
//             total_votes: 100_002_000,
//             delegated_votes: 300_000,
//             total_vote_buy_value: 300.0,
//             gauges: vec![
//                 GaugeInfo {
//                     gauge: gauge1,
//                     payment: 100.0,
//                     votes: 100_000_000,
//                 },
//                 GaugeInfo {
//                     gauge: gauge2,
//                     payment: 100.0,
//                     votes: 1_000,
//                 },
//                 GaugeInfo {
//                     gauge: gauge3,
//                     payment: 100.0,
//                     votes: 1_000,
//                 },
//             ],
//             prices: HashMap::from([
//                 (KnownTokens::Msol, 120.56),
//                 (KnownTokens::Uxd, 0.991553),
//                 (KnownTokens::Sbr, 0.00286583),
//                 (KnownTokens::Blze, 0.00311461),
//             ]),
//             escrows: vec![],
//             sbr_per_epoch: 0,
//             usd_per_vote: 0.0,
//         };
//
//         let weights = weight_calc(&data).unwrap();
//         println!("{:?}", weights);
//         assert_eq!(weights.len(), 2);
//         // Should make them even when the direct votes are included
//         assert_eq!(weights[0].votes, 150_000);
//         assert_eq!(weights[1].votes, 150_000);
//     }
// }
