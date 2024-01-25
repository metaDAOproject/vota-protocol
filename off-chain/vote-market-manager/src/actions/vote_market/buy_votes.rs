use crate::GAUGEMEISTER;
use anchor_client::Client;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;

pub(crate) fn buy_votes(
    anchor_client: &Client<&Keypair>,
    payer: &Keypair,
    config: &Pubkey,
    gauge: &Pubkey,
    mint: &Pubkey,
    epoch: u32,
    amount: u64,
) {
    let program = anchor_client.program(vote_market::id()).unwrap();
    let buyer_token_account = get_associated_token_address(&payer.pubkey(), mint);
    let (vote_buy, _) = Pubkey::find_program_address(
        &[
            b"vote-buy".as_ref(),
            epoch.to_le_bytes().as_ref(),
            config.as_ref(),
            gauge.to_bytes().as_ref(),
        ],
        &vote_market::id(),
    );
    let token_vault = get_associated_token_address(&vote_buy, mint);

    let (allowed_mints, _) = Pubkey::find_program_address(
        &[b"allow-list".as_ref(), config.as_ref()],
        &vote_market::id(),
    );
    program
        .request()
        .signer(payer)
        .accounts(vote_market::accounts::IncreaseVoteBuy {
            buyer: (*payer).pubkey(),
            buyer_token_account: buyer_token_account,
            token_vault,
            mint: *mint,
            config: *config,
            gaugemeister: GAUGEMEISTER,
            vote_buy,
            gauge: *gauge,
            allowed_mints,
            token_program: spl_token::id(),
            associated_token_program: spl_associated_token_account::id(),
            system_program: solana_program::system_program::id(),
        })
        .args(vote_market::instruction::IncreaseVoteBuy { amount, epoch })
        .send()
        .unwrap();
    println!("vote buy {}", vote_buy)
}
