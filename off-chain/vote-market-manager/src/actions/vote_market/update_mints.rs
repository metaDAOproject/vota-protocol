use anchor_client::Client;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

pub(crate) fn update_mints(client: &Client<&Keypair>, payer: &Keypair, config: Pubkey, allowed_mints: Vec<Pubkey>) {
    let program = client.program(vote_market::id()).unwrap();
    let (allowed_mints_address, _) = Pubkey::find_program_address(&[b"allow-list",
    config.as_ref()], &vote_market::id());
    let _sig = program
        .request()
        .signer(payer)
        .args(vote_market::instruction::UpdateAllowedMints{
            allowed_mints,
        })
        .accounts(vote_market::accounts::UpdateAllowedMints {
            config,
            admin: payer.pubkey(),
            allowed_mints: allowed_mints_address,
            system_program: solana_program::system_program::id(),
        }).send().unwrap();
    println!("Sim {:?}", _sig);
        // .send()
        // .unwrap();
}