use std::{env, fs};
use crate::account::{Account, Root};
use anchor_lang::AnchorDeserialize;
use anchor_lang::error::ComparedValues::Pubkeys;
use gauge_state::{EpochGauge, Gaugemeister};
use serde::Deserialize;
use dotenv::dotenv;
use anchor_lang::prelude::*;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::signature::Signer;
use locked_voter_state::Escrow;


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
    let path =env::var("KEY_PATH").expect("KEY_PATH must be set in .env");
    let payer = read_keypair_file(path).expect("Failed to read payer file");
    println!("Using payer pubkey: {:?}", payer.pubkey());
    // Set the escrow account to be owned by the payer
    let gaugemeister_file = std::fs::read_to_string("./external-state/account-gen/test-accounts/gaugemeister.json").expect("Failed to read gaugemeister account file");
    let gaugemeister_account = Root::from_string(&gaugemeister_file).expect("Failed to parse gaugemeister account");
    let mut gaugemeister_data = gaugemeister_account.get_account_data::<Gaugemeister>().expect("Failed to get gaugemeister account data");

    let escrow_accunt_file = std::fs::read_to_string("./external-state/account-gen/test-accounts/escrow.json").expect("Failed to read escrow account file");
    let escrow_account = Root::from_string(&escrow_accunt_file).expect("Failed to parse escrow account");
    let mut escrow_data = escrow_account.get_account_data::<Escrow>().expect("Failed to get escrow account data");
    escrow_data.owner = payer.pubkey();
    println!("program id: {:?}", &locked_voter_state::id());
    println!("gaugemeister: {:?}", gaugemeister_data);
    let (escrow_address, _) = Pubkey::find_program_address(
        &[
            b"Escrow",
            gaugemeister_data.locker.to_bytes().as_ref(),
            payer.pubkey().to_bytes().as_ref(),
        ],
        &locked_voter_state::id(),
    );
    let updated_escrow_account = escrow_account.update_account_data(&escrow_data)
        .expect("Failed to update escrow account data")
        .update_pubkey(&payer.pubkey())
        .expect("Failed to update escrow pubkey");
    //write escrow account to file
    let updated_escrow_account_json = serde_json::to_string(&updated_escrow_account).expect("Failed to serialize updated escrow account");
    fs::write("./test-accounts/updated-escrow.json", updated_escrow_account_json).expect("Failed to write updated escrow account to file");

    //Old gauge 6eVnGtqt4YvVbFn8a1KqyHdAsxhKMk1uz16Z6eCWNYgU,

    // let account = Root::from_string(&source_account).expect("Failed to parse account");
    // match account.get_account_data::<EpochGauge>() {
    //     Ok(epoch_gauge) => {
    //         print!("epoch_gauge: {:?}", epoch_gauge);
    //         let updated_address = account.update_pubkey(&Pubkey::default()).unwrap();
    //         println!("updated_address: {:?}", updated_address);
    //         let mut acct_data = updated_address.get_account_data::<EpochGauge>().unwrap();
    //         println!("acct_data: {:?}", acct_data);
    //         acct_data.gauge = Pubkey::default();
    //         let updated_account = updated_address.update_account_data(&acct_data).unwrap();
    //         println!("updated_account: {:?}", updated_account);
    //         let mut updated_acct_data = updated_account.get_account_data::<EpochGauge>().unwrap();
    //         println!("updated_acct_data: {:?}", updated_acct_data);
    //     }
    //     Err(e) => println!("{:?}", e),
    // }
    println!("done");
}
