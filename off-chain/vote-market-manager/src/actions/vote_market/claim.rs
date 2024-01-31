use crate::accounts::resolve::{get_delegate, get_vote_buy, resolve_vote_keys};
use crate::GAUGEMEISTER;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_associated_token_account::get_associated_token_address;

pub fn claim(
    anchor_client: &anchor_client::Client<&Keypair>,
    payer: &Keypair,
    mint: Pubkey,
    escrow: Pubkey,
    config: Pubkey,
    gauge: Pubkey,
    epoch: u32,
) {
    let program = anchor_client.program(vote_market::id()).unwrap();
    let seller_token_account = get_associated_token_address(&payer.pubkey(), &mint);
    let vote_buy = get_vote_buy(&config, &gauge, epoch);
    let token_vault = get_associated_token_address(&vote_buy, &mint);
    println!("vote buy {}", vote_buy);
    println!("token vault {}", token_vault);
    let vote_delegate = get_delegate(&config);
    let vote_accounts = resolve_vote_keys(&escrow, &gauge, epoch);

    let result = program
        .request()
        .signer(payer)
        .args(vote_market::instruction::ClaimVotePayment { epoch })
        .accounts(vote_market::accounts::ClaimVotePayment {
            script_authority: payer.pubkey(),
            seller: payer.pubkey(),
            seller_token_account,
            token_vault,
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
            locked_voter_program: locked_voter_state::id(),
            token_program: spl_token::id(),
            system_program: solana_program::system_program::id(),
        })
        .send()
        .unwrap();
    println!("result: {:?}", result);
}
