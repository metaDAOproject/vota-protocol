use crate::accounts::resolve::{get_escrow_address_for_owner, resolve_vote_keys, VoteCreateStep};
use crate::actions::create_epoch_gauge::create_epoch_gauge;
use crate::GAUGEMEISTER;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use crate::actions::retry_logic::retry_logic;

pub fn prepare_vote(client: &RpcClient, owner: Pubkey, gauge: Pubkey, payer: &Keypair, epoch: u32) {
    let escrow_address = get_escrow_address_for_owner(&owner);
    println!("Prepare vote for escrow: {:?}", escrow_address);
    let vote_keys = resolve_vote_keys(&escrow_address, &gauge, epoch);
    let steps = vote_keys.get_missing_prepare_vote_accounts(client);
    for step in steps {
        match step {
            VoteCreateStep::GaugeVoter(key) => {
                println!("Creating gauge voter {}", key);
                let data: Vec<u8> = solana_program::hash::hash(b"global:create_gauge_voter_v2")
                    .to_bytes()[..8]
                    .to_vec();
                let create_gauge_voter_ix = solana_program::instruction::Instruction {
                    program_id: gauge_state::id(),
                    accounts: vec![
                        //Gauge voter account
                        AccountMeta::new(key, false),
                        AccountMeta::new_readonly(GAUGEMEISTER, false),
                        AccountMeta::new_readonly(escrow_address, false),
                        AccountMeta::new(payer.pubkey(), true),
                        AccountMeta::new_readonly(solana_program::system_program::id(), false),
                    ],
                    data,
                };
                let mut ixs = vec![create_gauge_voter_ix];
                let result = retry_logic(client, payer, &mut ixs);
                match result {
                    Ok(sig) => {
                        log::info!(target: "vote",
                        sig=sig.to_string(),
                        user=owner.to_string(),
                        epoch=epoch;
                        "gauge voter created"
                        );
                        println!("Gauge voter created")
                    }
                    Err(e) => {
                        log::error!(target: "vote",
                        error=e.to_string(),
                        user=owner.to_string(),
                        epoch=epoch;
                        "failed to create gauge voter");
                        println!("Error creating gauge voter: {:?}", e)
                    }
                }
            }
            VoteCreateStep::GaugeVote(key) => {
                println!("Creating gauge vote {}", key);
                let data: Vec<u8> = solana_program::hash::hash(b"global:create_gauge_vote_v2")
                    .to_bytes()[..8]
                    .to_vec();
                let create_gauge_vote_ix = solana_program::instruction::Instruction {
                    program_id: gauge_state::id(),
                    accounts: vec![
                        //Gauge vote account
                        AccountMeta::new(key, false),
                        AccountMeta::new_readonly(vote_keys.gauge_voter, false),
                        AccountMeta::new_readonly(gauge, false),
                        AccountMeta::new(payer.pubkey(), true),
                        AccountMeta::new_readonly(solana_program::system_program::id(), false),
                    ],
                    data,
                };
                let mut ixs = vec![create_gauge_vote_ix];
                let result = retry_logic(client, payer, &mut ixs);
                match result {
                    Ok(sig) => {
                        log::info!(target: "vote",
                        sig=sig.to_string(),
                        user=owner.to_string(),
                        epoch=epoch;
                        "gauge vote created"
                        );
                        println!("Gauge vote created")
                    }
                    Err(e) => {
                        log::error!(target: "vote",
                        error=e.to_string(),
                        user=owner.to_string(),
                        epoch=epoch;
                        "failed to create gauge vote");
                        println!("Error creating gauge vote: {:?}", e)
                    }
                }
            }
            VoteCreateStep::EpochGauge(_key) => {
                create_epoch_gauge(client, payer, gauge, epoch);
            }
            VoteCreateStep::EpochGaugeVoter(_key) => {}
        }
    }
}
