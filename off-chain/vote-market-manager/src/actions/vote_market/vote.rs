use anchor_lang::AnchorDeserialize;
use crate::accounts::resolve::{get_delegate, resolve_vote_keys};
use crate::{GAUGEMEISTER, LOCKER};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use locked_voter_state::Escrow;
use crate::actions::management::data::VoteWeight;
use crate::actions::prepare_vote::prepare_vote;

pub fn vote(
    anchor_client: &anchor_client::Client<&Keypair>,
    client: &RpcClient,
    script_authority: &Keypair,
    config: Pubkey,
    escrow: Pubkey,
    epoch: u32,
    weights: Vec<VoteWeight>,
) -> Result<(), Box<dyn std::error::Error>> {
    let vote_delegate = get_delegate(&config);
    let program = anchor_client.program(vote_market::id())?;

    let escrow_account = client.get_account(&escrow).unwrap();
    let escrow_data = Escrow::deserialize(&mut &escrow_account.data[8..])?;
    let owner = escrow_data.owner;
    // Set weights
    for weight in weights {

        // Set weight
        let vote_accounts = resolve_vote_keys(&escrow, &weight.gauge, epoch);
        println!("Epoch the votes are for: {}", epoch);
        prepare_vote(client, owner, weight.gauge, script_authority, epoch);

        let vote_result = program
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
           .send();
        match vote_result {
            Ok(sig) => {
                println!("Vote succsesful for {:?}: {:?}", escrow, sig);
            }
            Err(e) => {
                println!("Error sending vote for {:?}: {:?}", escrow, e);
            }
        }
        let data: Vec<u8> = solana_program::hash::hash(b"global:prepare_epoch_gauge_voter_v2")
            .to_bytes()[..8]
            .to_vec();
        let create_epoch_gauge_voter_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                //Gauge vote account
                AccountMeta::new_readonly(GAUGEMEISTER, false),
                AccountMeta::new_readonly(LOCKER, false),
                AccountMeta::new_readonly(escrow, false),
                AccountMeta::new_readonly(vote_accounts.gauge_voter, false),
                AccountMeta::new(vote_accounts.epoch_gauge_voter, false),
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
            Ok(_) => {
                println!("Epoch gauge vote prepared for {:?}: {:?}", escrow, result);
            }
            Err(e) => {
                println!("Error preparing epoch gauge vote for {:?}: {:?}", escrow, e);
            }
        }
        println!("transaction: {:?}", transaction.signatures.first().unwrap());
        // Commit vote

        let mut data: Vec<u8> =
            solana_program::hash::hash(b"global:gauge_commit_vote_v2").to_bytes()[..8].to_vec();
        data.extend_from_slice(&weight.weight.to_le_bytes());
        let commit_vote_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                AccountMeta::new_readonly(GAUGEMEISTER, false),
                AccountMeta::new_readonly(weight.gauge, false),
                AccountMeta::new_readonly(vote_accounts.gauge_voter, false),
                AccountMeta::new_readonly(vote_accounts.gauge_vote, false),
                AccountMeta::new(vote_accounts.epoch_gauge, false),
                AccountMeta::new(vote_accounts.epoch_gauge_voter, false),
                AccountMeta::new(vote_accounts.epoch_gauge_vote, false),
                AccountMeta::new(script_authority.pubkey(), true),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data,
        };
        let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
            &[commit_vote_ix],
            Some(&script_authority.pubkey()),
        );
        let latest_blockhash = client.get_latest_blockhash().unwrap();
        transaction.sign(&[script_authority], latest_blockhash);
        let result = client
            .send_and_confirm_transaction_with_spinner_and_config(
                &transaction,
                CommitmentConfig::confirmed(),
                RpcSendTransactionConfig {
                    skip_preflight: true,
                    ..RpcSendTransactionConfig::default()
                },
            );
        match result {
            Ok(_) => {
                println!("Vote committed for {:?}: {:?}", escrow, result);
            }
            Err(e) => {
                println!("Error committing vote for {:?}: {:?}", escrow, e);
            }
        }
    }
    Ok(())
}
