use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use crate::actions::retry_logic;

pub(crate) fn update_mints(
    anchor_client: &Client<&Keypair>,
    client: &RpcClient,
    payer: &Keypair,
    config: Pubkey,
    allowed_mints: Vec<Pubkey>,
) {
    let program = anchor_client.program(vote_market::id()).unwrap();
    let (allowed_mints_address, _) =
        Pubkey::find_program_address(&[b"allow-list", config.as_ref()], &vote_market::id());
    let mut ixs = program
        .request()
        .signer(payer)
        .args(vote_market::instruction::UpdateAllowedMints { allowed_mints })
        .accounts(vote_market::accounts::UpdateAllowedMints {
            config,
            admin: payer.pubkey(),
            allowed_mints: allowed_mints_address,
            system_program: solana_program::system_program::id(),
        }).instructions().unwrap();
    let result = retry_logic::retry_logic(client, payer, &mut ixs,None);
    match result {
        Ok(sig) => println!("allowed mints updated: {:?}", sig),
        Err(e) => println!("Error updating allowed mints: {:?}", e),
    }
}
