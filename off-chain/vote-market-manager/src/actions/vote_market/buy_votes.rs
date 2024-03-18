use crate::accounts::resolve::get_vote_buy;
use crate::GAUGEMEISTER;
use anchor_client::Client;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;
use crate::actions::retry_logic;

pub(crate) fn buy_votes(
    anchor_client: &Client<&Keypair>,
    client: &solana_client::rpc_client::RpcClient,
    payer: &Keypair,
    config: &Pubkey,
    gauge: &Pubkey,
    mint: &Pubkey,
    epoch: u32,
    amount: u64,
) {
    let program = anchor_client.program(vote_market::id()).unwrap();
    let buyer_token_account = get_associated_token_address(&payer.pubkey(), mint);
    let vote_buy = get_vote_buy(config, gauge, epoch);
    let token_vault = get_associated_token_address(&vote_buy, mint);
    println!("vote buy {}", vote_buy);
    println!("token vault {}", token_vault);
    let (allowed_mints, _) = Pubkey::find_program_address(
        &[b"allow-list".as_ref(), config.as_ref()],
        &vote_market::id(),
    );
    let mut ixs = program
        .request()
        .signer(payer)
        .accounts(vote_market::accounts::IncreaseVoteBuy {
            buyer: (*payer).pubkey(),
            buyer_token_account,
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
        .instructions().unwrap();
    let result = retry_logic::retry_logic(client, payer, &mut ixs, None);

    match result {
        Ok(sig) => println!("Vote buy increased: {:?}", sig),
        Err(e) => println!("Error increasing vote buy: {:?}", e),
    }
}
