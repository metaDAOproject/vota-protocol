use crate::accounts::resolve::{get_delegate, get_gauge_vote, get_gauge_voter, resolve_vote_keys};
use crate::{GAUGEMEISTER, LOCKER};
use anchor_client::Client;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use std::fs;
use std::str::FromStr;

pub(crate) fn clear_votes(
    anchor_client: &Client<&Keypair>,
    client: &RpcClient,
    script_authority: &Keypair,
    config: Pubkey,
    owner: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let program = anchor_client.program(vote_market::id())?;
    let gauges_file = fs::File::open("off-chain/vote-market-manager/info/gauges.json")?;
    let gauges: Vec<String> = serde_json::from_reader(gauges_file)?;
    let gauges = gauges.iter().map(|g| Pubkey::from_str(g).unwrap()).collect::<Vec<Pubkey>>();
    let vote_delegate = get_delegate(&config);
    let (escrow, _) = Pubkey::find_program_address(
        &[
            b"Escrow",
            LOCKER.to_bytes().as_ref(),
            owner.to_bytes().as_ref(),
        ],
        &locked_voter_state::id(),
    );
    let gauge_voter = get_gauge_voter(&escrow);
    let gauge_votes = gauges
        .iter()
        .map(|g| get_gauge_vote(&get_gauge_voter(&escrow), g))
        .collect::<Vec<Pubkey>>();
    let gauge_vote_accounts = client.get_multiple_accounts(&gauge_votes)?;
    let mut instructions: Vec<Instruction> = Vec::new();


    for (i, gauge) in gauges.iter().enumerate() {
        // Can only clear initialized gauge_votes
        if gauge_vote_accounts[i].is_none() {
            continue;
        }
        let gauge_vote = gauge_votes[i];
        let vote_ixs = program
            .request()
            .signer(script_authority)
            .args(vote_market::instruction::Vote { weight: 0 })
            .accounts(vote_market::accounts::Vote {
                config,
                script_authority: script_authority.pubkey(),
                gaugemeister: GAUGEMEISTER,
                gauge: *gauge,
                gauge_voter: gauge_voter,
                gauge_vote: gauge_vote,
                escrow,
                vote_delegate,
                gauge_program: gauge_state::id(),
            })
            .instructions()
            .unwrap();
        for ix in vote_ixs {
            instructions.push(ix);
        }
    }
    let mut tx = Transaction::new_with_payer(&instructions, Some(&script_authority.pubkey()));
    tx.sign(&[script_authority], client.get_latest_blockhash()?);
    let result = client.send_transaction_with_config(
        &tx,
        RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Processed),
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        },
    );
    match result {
        Ok(sig) => {
            log::info!(target: "vote",
                sig=sig.to_string(),
                user=owner.to_string(),
                config=config.to_string();
                "cleared votes");
            println!("Cleared votes for {:?}: {:?}", escrow, sig);
        }
        Err(e) => {
            log::error!(target: "vote",
                error=e.to_string(),
                user=owner.to_string(),
                config=config.to_string();
                "failed to clear votes");
            println!("Error clearing votes for {:?}: {:?}", escrow, e);
        }
    }
    Ok(())
}
