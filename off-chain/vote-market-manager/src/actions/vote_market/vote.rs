use crate::accounts::resolve::{get_delegate, resolve_vote_keys};
use crate::GAUGEMEISTER;
use anchor_lang::prelude::{Account, Program, SystemAccount};
use gauge_state::GaugeProgram;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use vote_market::state::VoteMarketConfig;

pub struct WeightInfo {
    pub gauge: Pubkey,
    pub weight: u32,
}

pub fn vote(
    anchor_client: &anchor_client::Client<&Keypair>,
    script_authority: &Keypair,
    config: &Pubkey,
    escrow: &Pubkey,
    epoch: u32,
    weights: Vec<WeightInfo>,
) {
    let vote_delegate = get_delegate(config);
    let program = anchor_client.program(vote_market::id()).unwrap();
    // Set weights
    for weight in weights {
        let vote_accounts = resolve_vote_keys(&escrow, &weight.gauge, epoch);
        let sig = program
            .request()
            .signer(script_authority)
            .args(vote_market::instruction::Vote {
                weight: weight.weight,
            })
            .accounts(vote_market::accounts::Vote {
                config: *config,
                script_authority: script_authority.pubkey(),
                gaugemeister: GAUGEMEISTER,
                gauge: weight.gauge,
                gauge_voter: vote_accounts.gauge_voter,
                gauge_vote: vote_accounts.gauge_vote,
                escrow: *escrow,
                vote_delegate,
                gauge_program: gauge_state::id(),
            })
            .send()
            .unwrap();
        println!("vote: {}", sig)
    }

    // Commit vote
}
