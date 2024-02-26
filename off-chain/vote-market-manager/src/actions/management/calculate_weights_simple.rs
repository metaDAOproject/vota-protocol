use crate::actions::management::data::{EpochData, VoteInfo};

pub(crate) fn calculate_weights(
    data: &mut EpochData,
) -> Result<Vec<VoteInfo>, Box<dyn std::error::Error>> {
    let total: f64 = data.gauges.iter().map(|g| g.payment).sum();
    let multiplier = u32::MAX as f64 / total;
    let mut vote_infos: Vec<VoteInfo> = Vec::new();
    for gauge in &data.gauges {
        let vote_info = VoteInfo {
            weight: (gauge.payment * multiplier) as u32,
            votes: gauge.votes,
            gauge: gauge.gauge,
        };
        vote_infos.push(vote_info);
    }
    Ok(vote_infos)
}
