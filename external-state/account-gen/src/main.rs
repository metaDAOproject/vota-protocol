use anchor_lang::prelude::*;
use dotenv::dotenv;
use gauge_state::{EpochGauge, EpochGaugeVote, EpochGaugeVoter, Gauge, Gaugemeister, GaugeVote, GaugeVoter};
use locked_voter_state::Escrow;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;
use std::env;
use crate::account::proccess_account;

mod account;
mod errors;
mod utils;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cwd = env::current_dir().unwrap();
    // Make sure this is run from the project workspace directory
    if let Some(dirname) = cwd.iter().last().and_then(|osstr| osstr.to_str()) {
        if dirname == "account-gen" {
            println!("Must run from project root directory");
            return Err(Box::new(errors::AccountGenError::InvalidCwd));
        }
    }
    let path = match env::var("KEY_PATH") {
        Ok(path) => path,
        Err(e) => {
            println!("No KEY_PATH env variable set");
            return Err(Box::new(e));
        }
    };
    let payer = read_keypair_file(path)?;
    println!("Using payer pubkey: {:?}", payer.pubkey());
    let (gaugemeister_data, gaugemeister_account) =
        account::proccess_account::<Gaugemeister, _>("gaugemeister", None, |x| x)?;

    let (escrow_address, _) = Pubkey::find_program_address(
        &[
            b"Escrow",
            gaugemeister_data.locker.to_bytes().as_ref(),
            payer.pubkey().to_bytes().as_ref(),
        ],
        &locked_voter_state::id(),
    );
    proccess_account::<Escrow, _>("escrow", Some(escrow_address), |mut escrow_data| {
        escrow_data.owner = payer.pubkey();
        escrow_data
    })?;

    let (gauge_voter_address, _) = Pubkey::find_program_address(
        &[
            b"GaugeVoter",
            gaugemeister_account.pubkey.to_bytes().as_ref(),
            escrow_address.to_bytes().as_ref(),
        ],
        &gauge_state::id(),
    );
    account::proccess_account::<GaugeVoter, _>("gauge-voter", Some(gauge_voter_address), |mut data| {
            data.owner = payer.pubkey();
            data.escrow = escrow_address;
            data
        })?;

    let (_, gauge_account) = account::proccess_account::<Gauge,_>("gauge", None, |x| x)?;

    let (gauge_vote_address, _) = Pubkey::find_program_address(
        &[
            b"GaugeVote",
            gauge_voter_address.to_bytes().as_ref(),
            gauge_account.pubkey.to_bytes().as_ref(),
        ],
        &gauge_state::id(),
    );
    account::proccess_account::<GaugeVote,_>("gauge-vote", Some(gauge_vote_address), |mut data| {
        data.gauge_voter = payer.pubkey();
        data
    })?;

    let (epoch_gauge_address, _) = Pubkey::find_program_address(
        &[
            b"EpochGauge",
            gauge_account.pubkey.to_bytes().as_ref(),
            gaugemeister_data
                .voting_epoch()?
                .to_le_bytes()
                .as_ref(),
        ],
        &gauge_state::id(),
    );

    let (epoch_gauge_data, _) = account::proccess_account::<EpochGauge, _>("epoch-gauge", Some(epoch_gauge_address), |x|x )?;

    let (epoch_gauge_voter_address, _) = Pubkey::find_program_address(
        &[
            b"EpochGaugeVoter",
            gauge_voter_address.to_bytes().as_ref(),
            gaugemeister_data.voting_epoch()?.to_le_bytes().as_ref(),
        ],
        &gauge_state::id(),
    );
    let (epoch_gauge_voter_data, _) = proccess_account::<EpochGaugeVoter,_>("epoch-gauge-voter", Some(epoch_gauge_voter_address), |mut data| {
        data.gauge_voter = gauge_voter_address;
        data.voting_epoch = gaugemeister_data.voting_epoch().expect("if it deserializes the epoch should be valid");
        data
    })?;
    proccess_account::<EpochGaugeVoter,_>("epoch-gauge-voter", Some(epoch_gauge_voter_address), |mut data| {
        data.gauge_voter = gauge_voter_address;
        data
    })?;

    let (epoch_gauge_vote_address, _) = Pubkey::find_program_address(
        &[
            b"EpochGaugeVote",
            gauge_vote_address.to_bytes().as_ref(),
            epoch_gauge_voter_data.voting_epoch.to_le_bytes().as_ref(),
        ],
        &gauge_state::id(),
    );
    proccess_account::<EpochGaugeVote,_>("epoch-gauge-vote", Some(epoch_gauge_vote_address), |x| x)?;

    Ok(())
}
