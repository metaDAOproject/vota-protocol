use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::accounts::resolve::get_escrow_address_for_owner;

pub fn prepare_vote(client: &RpcClient, owner: Pubkey) {
    let escrow_address = get_escrow_address_for_owner(&owner);
    println!("Prepare vote for escrow: {:?}", escrow_address);
}