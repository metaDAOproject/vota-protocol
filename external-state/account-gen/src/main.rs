use crate::account::{Account, Root};
use anchor_lang::error::ComparedValues::Pubkeys;
use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use dotenv::dotenv;
use gauge_state::{EpochGauge, Gaugemeister};
use locked_voter_state::Escrow;
use serde::Deserialize;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;
use std::{env, fs};

mod account;

fn main() {
    dotenv().ok();
    let cwd = std::env::current_dir().unwrap();
    // Make sure this is run from the project workspace directory
    if let Some(dirname) = cwd.iter().last().and_then(|osstr| osstr.to_str()) {
        if dirname == "account-gen" {
            println!(
                "Run this script from the workspace directory using `cargo run -p account-gen`"
            );
            return ();
        }
    }
    let path = env::var("KEY_PATH").expect("KEY_PATH must be set in .env");
    let payer = read_keypair_file(path).expect("Failed to read payer file");
    println!("Using payer pubkey: {:?}", payer.pubkey());
    let gaugemeister_file =
        std::fs::read_to_string("./external-state/account-gen/test-accounts/gaugemeister.json")
            .expect("Failed to read gaugemeister account file");
    let gaugemeister_account =
        Root::from_string(&gaugemeister_file).expect("Failed to parse gaugemeister account");
    let mut gaugemeister_data = gaugemeister_account
        .get_account_data::<Gaugemeister>()
        .expect("Failed to get gaugemeister account data");

    let escrow_account_file =
        std::fs::read_to_string("./external-state/account-gen/test-accounts/escrow.json")
            .expect("Failed to read escrow account file");
    let escrow_account =
        Root::from_string(&escrow_account_file).expect("Failed to parse escrow account");
    let mut escrow_data = escrow_account
        .get_account_data::<Escrow>()
        .expect("Failed to get escrow account data");
    escrow_data.owner = payer.pubkey();
    println!("program id: {:?}", &locked_voter_state::id());
    let (escrow_address, _) = Pubkey::find_program_address(
        &[
            b"Escrow",
            gaugemeister_data.locker.to_bytes().as_ref(),
            payer.pubkey().to_bytes().as_ref(),
        ],
        &locked_voter_state::id(),
    );
    let updated_escrow_account = escrow_account
        .update_account_data(&escrow_data)
        .expect("Failed to update escrow account data")
        .update_pubkey(&escrow_address)
        .expect("Failed to update escrow pubkey");
    //write escrow account to file
    let updated_data = updated_escrow_account.get_account_data::<Escrow>().unwrap();
    let updated_escrow_account_json = serde_json::to_string(&updated_escrow_account)
        .expect("Failed to serialize updated escrow account");
    fs::write(
        "./test-accounts/updated-escrow.json",
        updated_escrow_account_json,
    )
    .expect("Failed to write updated escrow account to file");

    //Old gauge 6eVnGtqt4YvVbFn8a1KqyHdAsxhKMk1uz16Z6eCWNYgU,

    println!("done");
}
