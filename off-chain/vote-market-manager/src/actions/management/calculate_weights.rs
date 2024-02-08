use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use gauge_state::EpochGauge;
use crate::actions::management::data::EpochData;

pub(crate) fn calculate_weights(data: EpochData)
    -> Result<(), Box<dyn std::error::Error>> {
    println!("calculate_weights {:?}", data);
    Ok(())
}