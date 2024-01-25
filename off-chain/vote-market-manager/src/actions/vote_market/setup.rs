use crate::GAUGEMEISTER;
use anchor_lang::Key;
use solana_client::rpc_config::{RpcRequestAirdropConfig, RpcSendTransactionConfig};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};

pub fn setup(anchor_client: anchor_client::Client<&Keypair>, mints: Vec<Pubkey>, payer: &Keypair) {
    let program = anchor_client.program(vote_market::id()).unwrap();
    println!("Creating config");
    let config = Keypair::new();
    let (allowed_mints, _) = Pubkey::find_program_address(
        &[b"allow-list".as_ref(), config.pubkey().as_ref()],
        &vote_market::id(),
    );
    program
        .request()
        .signer(payer)
        .signer(&config)
        .accounts(vote_market::accounts::CreateConfig {
            config: config.pubkey(),
            payer: payer.pubkey(),
            allowed_mints,
            system_program: solana_program::system_program::id(),
        })
        .args(vote_market::instruction::CreateConfig {
            mints,
            gaugemeister: GAUGEMEISTER,
            efficiency_ratio: 0,
            script_authority: payer.pubkey(),
        })
        .send()
        .unwrap();
    println!("Allowed mints: {}", allowed_mints);
    println!("Config: {}", config.pubkey());
}