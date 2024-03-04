use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_program::system_instruction::create_account;
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;

pub fn create_token(client: &RpcClient, payer: &Keypair) {
    let mint = Keypair::new();
    let lamports = client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .unwrap();
    let init_mint_account_ix = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        lamports,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        0,
    )
    .unwrap();

    let ata = get_associated_token_address(&payer.pubkey(), &mint.pubkey());

    let ata_ix = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::id(),
    );

    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint.pubkey(),
        &ata,
        &payer.pubkey(),
        &[],
        1000000000,
    )
    .unwrap();

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[init_mint_account_ix, init_mint_ix, ata_ix, mint_to_ix],
        Some(&payer.pubkey()),
        &[payer, &mint],
        latest_blockhash,
    );
    let result = client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &tx,
            solana_sdk::commitment_config::CommitmentConfig::confirmed(),
        );
    match result {
        Ok(sig) => {
            println!("mint: {}", mint.pubkey());
        }
        Err(e) => {
            println!("Error minting: {:?}", e);
        }
    }
}
