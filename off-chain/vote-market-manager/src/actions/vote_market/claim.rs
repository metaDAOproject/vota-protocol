use crate::accounts::resolve::{get_delegate, get_vote_buy, resolve_vote_keys};
use crate::actions::retry_logic;
use crate::GAUGEMEISTER;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use std::error::Error;

pub fn claim(
    anchor_client: &anchor_client::Client<&Keypair>,
    client: &RpcClient,
    payer: &Keypair,
    seller: Pubkey,
    mint: Pubkey,
    escrow: Pubkey,
    config: Pubkey,
    gauge: Pubkey,
    epoch: u32,
) -> Result<(), Box<dyn Error>> {
    let vote_delegate = get_delegate(&config);
    let seller_token_account = get_associated_token_address(&seller, &mint);
    let vote_accounts = resolve_vote_keys(&escrow, &gauge, epoch);
    let [ref seller_token_account_info, ref epoch_gauge_vote_acount_info] =
        client.get_multiple_accounts(&[seller_token_account, vote_accounts.epoch_gauge_vote])?[..]
    else {
        return Err("Failed to get accounts".into());
    };
    if epoch_gauge_vote_acount_info.is_none() {
        println!("No votes for {}. Nothing to do", escrow.to_string());
        return Ok(());
    }
    let mut create_ata_ix = None;
    if seller_token_account_info.is_none() {
        create_ata_ix = Some(create_associated_token_account(
            &payer.pubkey(),
            &seller,
            &mint,
            &spl_token::id(),
        ));
        println!("creating associated token account");
    }
    let vote_buy = get_vote_buy(&config, &gauge, epoch);
    let token_vault = get_associated_token_address(&vote_buy, &mint);
    let treasury = get_associated_token_address(&payer.pubkey(), &mint);
    let program = anchor_client.program(vote_market::id()).unwrap();

    let mut ixs = program
        .request()
        .signer(payer)
        .args(vote_market::instruction::ClaimVotePayment { epoch })
        .accounts(vote_market::accounts::ClaimVotePayment {
            script_authority: payer.pubkey(),
            seller,
            seller_token_account,
            token_vault,
            treasury,
            admin: payer.pubkey(),
            mint,
            config,
            vote_buy,
            vote_delegate,
            escrow,
            gaugemeister: GAUGEMEISTER,
            gauge_voter: vote_accounts.gauge_voter,
            gauge_vote: vote_accounts.gauge_vote,
            epoch_gauge_voter: vote_accounts.epoch_gauge_voter,
            gauge,
            epoch_gauge: vote_accounts.epoch_gauge,
            epoch_gauge_vote: vote_accounts.epoch_gauge_vote,
            gauge_program: gauge_state::id(),
            locked_voter_program: saber_locker::id(),
            token_program: spl_token::id(),
            system_program: solana_program::system_program::id(),
        })
        .instructions()?;

    if let Some(ata_ix) = create_ata_ix {
        ixs.insert(0, ata_ix);
    }

    let result = retry_logic::retry_logic(client, &payer, &mut ixs, Some(150_000));
    //This worked once, but blockage expired will panic
    match result {
        Ok(sig) => {
            log::info!(target: "claim",
            sig=sig.to_string(),
            user=seller.to_string(),
            config=config.to_string(),
            gauge=gauge.to_string(),
            epoch=epoch;
            "claiming vote payment"
            );
            println!("claimed vote payment");
        }
        Err(e) => {
            log::error!(target: "claim",
                error=e.to_string(),
                user=seller.to_string(),
                config=config.to_string(),
                gauge=gauge.to_string(),
                epoch=epoch;
                "failed to claim vote payment");
            println!("failed to claim vote payment");
        }
    }
    Ok(())
}
