use anchor_lang::AnchorDeserialize;
use crate::accounts::resolve::{
    get_delegate, get_epoch_gauge_voter, get_escrow_address_for_owner, get_gauge_voter,
    resolve_vote_keys,
};
use crate::actions::management::data::VoteInfo;
use crate::actions::prepare_vote::prepare_vote;
use crate::GAUGEMEISTER;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use crate::actions::retry_logic::retry_logic;

pub fn vote(
    anchor_client: &anchor_client::Client<&Keypair>,
    client: &RpcClient,
    script_authority: &Keypair,
    config: Pubkey,
    owner: Pubkey,
    epoch: u32,
    weights: Vec<VoteInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let vote_delegate = get_delegate(&config);
    println!("Vote delegate address is {}", vote_delegate);
    let program = anchor_client.program(vote_market::id())?;
    let escrow = get_escrow_address_for_owner(&owner);

    // Set weights
    let mut vote_instructions: Vec<Instruction> = Vec::new();
    for weight in weights.clone() {
        // Set weight
        let vote_accounts = resolve_vote_keys(&escrow, &weight.gauge, epoch);
        println!("Epoch the votes are for: {}", epoch);
        println!("Epoch gauge voter: {:?}", vote_accounts.epoch_gauge_voter);
        prepare_vote(client, owner, weight.gauge, script_authority, epoch);

        //check if weight needs to change
        let vote_account = client.get_account(&vote_accounts.gauge_vote)?;
        let vote_data = gauge_state::GaugeVote::deserialize(&mut vote_account.data[..].as_ref())?;

        if vote_data.weight == weight.weight {
            println!("Weight is already set to {}", weight.weight);
            continue;
        }

        println!("Prepare voter completed");
        let vote_ixs = program
            .request()
            .signer(script_authority)
            .args(vote_market::instruction::Vote {
                weight: weight.weight,
            })
            .accounts(vote_market::accounts::Vote {
                config,
                script_authority: script_authority.pubkey(),
                gaugemeister: GAUGEMEISTER,
                gauge: weight.gauge,
                gauge_voter: vote_accounts.gauge_voter,
                gauge_vote: vote_accounts.gauge_vote,
                escrow,
                vote_delegate,
                gauge_program: gauge_state::id(),
            })
            .instructions()?;
        for ix in vote_ixs {
            vote_instructions.push(ix);
        }
    }
    if vote_instructions.len() > 0 {
        let max_cus = 100_000;
        let vote_result = retry_logic(client, script_authority, &mut vote_instructions, Some(max_cus));
        match vote_result {
            Ok(sig) => {
                log::info!(target: "vote",
            sig=sig.to_string(),
            user=owner.to_string(),
            config=config.to_string(),
            epoch=epoch;
            "set vote weight"
            );
                println!("Vote succsesful for {:?}: {:?}", escrow, sig);
            }
            Err(e) => {
                log::error!(target: "vote",
                    error=e.to_string(),
                    user=owner.to_string(),
                    config=config.to_string(),
                    epoch=epoch;
                    "failed to set vote weight");
                println!("Error sending vote for {:?}: {:?}", escrow, e);
                return Err(Box::<dyn std::error::Error>::from(anyhow::anyhow!(e.to_string())));
            }
        }
    }

    println!("Creating epoch gauge voter");
    // Create epoch gauge voter when all votes are complete
    let gauge_voter = get_gauge_voter(&escrow);
    let epoch_gauge_voter = get_epoch_gauge_voter(&gauge_voter, epoch);
    let epoch_gauge_voter_account = client.get_account(&epoch_gauge_voter);
    if epoch_gauge_voter_account.is_ok() {
        println!("Epoch gauge voter already exists");
    } else {
        let data: Vec<u8> =
            solana_program::hash::hash(b"global:prepare_epoch_gauge_voter_v2").to_bytes()[..8].to_vec();
        let create_epoch_gauge_voter_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                //Gauge vote account
                AccountMeta::new_readonly(crate::GAUGEMEISTER, false),
                AccountMeta::new_readonly(crate::LOCKER, false),
                AccountMeta::new_readonly(escrow, false),
                AccountMeta::new_readonly(gauge_voter, false),
                AccountMeta::new(epoch_gauge_voter, false),
                AccountMeta::new(script_authority.pubkey(), true),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data,
        };
        let mut ixs = vec![create_epoch_gauge_voter_ix];
        let max_cus = 25_000;
        let result = retry_logic(client, script_authority, &mut ixs, Some(max_cus));
        match result {
            Ok(sig) => {
                log::info!(target: "vote",
                sig=sig.to_string(),
                user=owner.to_string(),
                config=config.to_string(),
                epoch=epoch;
                "epoch gauge vote prepared"
                );
                println!("Epoch gauge vote prepared for {:?}: {:?}", escrow, result);
            }
            Err(e) => {
                log::error!(target: "vote",
                error=e.to_string(),
                user=owner.to_string(),
                config=config.to_string(),
                epoch=epoch;
                "failed to prepare epoch gauge vote");
                println!("Error preparing epoch gauge vote for {:?}: {:?}", escrow, e);
                return Err(Box::<dyn std::error::Error>::from(anyhow::anyhow!(e.to_string())));
            }
        }
        //add a delay to wait for the epoch gauge voter to be created
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    let mut commit_instructions: Vec<Instruction> = Vec::new();
    for weight in weights {
        let vote_accounts = resolve_vote_keys(&escrow, &weight.gauge, epoch);

        // Commit vote
        let commit_ixs = program
            .request()
            .signer(script_authority)
            .args(vote_market::instruction::CommitVote { epoch })
            .accounts(vote_market::accounts::CommitVote {
                config,
                script_authority: script_authority.pubkey(),
                gaugemeister: GAUGEMEISTER,
                gauge: weight.gauge,
                gauge_voter: vote_accounts.gauge_voter,
                gauge_vote: vote_accounts.gauge_vote,
                epoch_gauge: vote_accounts.epoch_gauge,
                epoch_gauge_voter: vote_accounts.epoch_gauge_voter,
                epoch_gauge_vote: vote_accounts.epoch_gauge_vote,
                vote_delegate,
                gauge_program: gauge_state::id(),
                system_program: solana_program::system_program::id(),
            })
            .instructions()?;
        for ix in commit_ixs {
            commit_instructions.push(ix);
        }
    }
    let max_cus = 75_000;
    let commit_result = retry_logic(client, script_authority, &mut commit_instructions, Some(max_cus));
    match commit_result {
        Ok(sig) => {
            log::info!(target: "vote",
            sig=sig.to_string(),
            user=owner.to_string(),
            config=config.to_string(),
            epoch=epoch;
            "vote committed"
            );
            client.confirm_transaction_with_spinner(
                &sig,
                &client.get_latest_blockhash()?,
                CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                },
            )?;
            println!("Vote committed for {:?}: {:?}", escrow, sig);
        }
        Err(e) => {
            log::error!(target: "vote",
                error=e.to_string(),
                user=owner.to_string(),
                config=config.to_string(),
                epoch=epoch;
                "failed to commit vote");
            println!("Error committing vote for {:?}: {:?}", escrow, e);
            return Err(Box::<dyn std::error::Error>::from(anyhow::anyhow!(e.to_string())));
        }
    }
    Ok(())
}
