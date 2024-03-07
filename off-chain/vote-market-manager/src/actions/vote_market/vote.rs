use crate::accounts::resolve::{
    get_delegate, get_epoch_gauge_voter, get_escrow_address_for_owner, get_gauge_voter,
    resolve_vote_keys,
};
use crate::actions::management::data::VoteInfo;
use crate::actions::prepare_vote::prepare_vote;
use crate::{GAUGEMEISTER, LOCKER};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_error::ProgramError::MaxInstructionTraceLengthExceeded;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;

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
    let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
        &vote_instructions,
        Some(&script_authority.pubkey()),
    );
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[script_authority], latest_blockhash);
    let vote_result = client.send_and_confirm_transaction(&transaction);

    match vote_result {
        Ok(sig) => {
            log::info!(target: "vote",
            sig=sig.to_string(),
            user=owner.to_string(),
            config=config.to_string(),
            epoch=epoch;
            "set vote weight"
            );

            // client.confirm_transaction_with_spinner(
            //     &sig,
            //     &client.get_latest_blockhash()?,
            //     CommitmentConfig {
            //         commitment: CommitmentLevel::Confirmed,
            //     },
            // )?;
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
            return Err(Box::new(e));
        }
    }

    // Create epoch gauge voter when all votes are complete
    let gauge_voter = get_gauge_voter(&escrow);
    let epoch_gauge_voter = get_epoch_gauge_voter(&gauge_voter, epoch);
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
    let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
        &[create_epoch_gauge_voter_ix],
        Some(&script_authority.pubkey()),
    );
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[script_authority], latest_blockhash);
    let result = client.send_and_confirm_transaction(&transaction);
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
            return Err(Box::new(e));
        }
    }
    println!("transaction: {:?}", transaction.signatures.first().unwrap());
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
    let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
        &commit_instructions,
        Some(&script_authority.pubkey()),
    );
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[script_authority], latest_blockhash);
    let commit_result = client.send_and_confirm_transaction(&transaction);

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
            return Err(Box::new(e));
        }
    }
    Ok(())
}
