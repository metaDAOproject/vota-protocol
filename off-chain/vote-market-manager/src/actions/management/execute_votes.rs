use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use crate::actions::management::data::{EpochData, VoteWeight};

pub(crate) fn execute_votes(client: &RpcClient, anchor_client: &Client<&Keypair>, script_authority: &Keypair, data: EpochData, vote_weights: Vec<VoteWeight>) -> Result<(),
Box<dyn std::error::Error>> {
    todo!()
}