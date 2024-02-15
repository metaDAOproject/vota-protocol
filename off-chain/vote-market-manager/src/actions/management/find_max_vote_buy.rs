use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::actions::management::data::EpochData;

pub(crate) fn find_max_vote_buy(data: &mut EpochData) -> Result<(), Box<dyn std::error::Error>> {
    println!("find_max_vote_buy {:#?}", data);
    Ok(())
}