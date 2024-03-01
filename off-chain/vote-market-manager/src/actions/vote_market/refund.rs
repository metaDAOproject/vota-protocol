use anchor_client::Client;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;
use vote_market::state::VoteBuy;
use crate::accounts::resolve::get_vote_buy;
use crate::GAUGEMEISTER;

pub(crate) fn get_refund(anchor_client: &Client<&Keypair>, payer: &Keypair, config: Pubkey, gauge: Pubkey, epoch: u32) {
    let program = anchor_client.program(vote_market::id()).unwrap();
    let vote_buy = get_vote_buy(&config, &gauge, epoch);
    let vote_buy_data : VoteBuy = program.account(vote_buy).unwrap();
    let buyer_token_account = get_associated_token_address(&payer.pubkey(), &vote_buy_data.mint);
    let token_vault = get_associated_token_address(&vote_buy, &vote_buy_data.mint);
    println!("Buyer token account: {:?}", buyer_token_account);
    println!("Token vault: {:?}", token_vault);
    let result = program
        .request()
        .signer(payer)
        .args(vote_market::instruction::VoteBuyRefund{
            epoch
        })
        .accounts(vote_market::accounts::VoteBuyRefund {
            buyer: payer.pubkey(),
            buyer_token_account,
            vote_buy,
            token_vault,
            mint: vote_buy_data.mint,
            config,
            gauge,
            gaugemeister: GAUGEMEISTER,
            token_program: spl_token::id(),
        }).send();
        // .send_with_spinner_and_config(
        //     RpcSendTransactionConfig {
        //         ..RpcSendTransactionConfig::default()
        //     }
        // );
    match result {
        Ok(sig) => {
            log::info!(target: "refund",
            sig=sig.to_string(),
            user=payer.pubkey().to_string(),
            config=config.to_string(),
            gauge=gauge.to_string(),
            epoch=epoch;
            "claiming refund"
            );
            println!("Refund signature: {:?}", sig)
        },
        Err(e) => {
            log::error!(target: "refund",
            error=e.to_string(),
            user=payer.pubkey().to_string(),
            config=config.to_string(),
            gauge=gauge.to_string(),
            epoch=epoch;
            "failed to claim refund");
            println!("Error claiming refund: {:?}", e)
        }
    }

}