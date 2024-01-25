use anchor_client::Client;
use crate::accounts::resolve::{get_delegate, resolve_vote_keys};
use crate::{GAUGEMEISTER, LOCKER};
use anchor_lang::prelude::{Account, Program, System, SystemAccount};
use gauge_state::GaugeProgram;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use vote_market::state::VoteMarketConfig;

pub struct WeightInfo {
    pub gauge: Pubkey,
    pub weight: u32,
}

pub fn vote(
    anchor_client: &anchor_client::Client<&Keypair>,
    client: &RpcClient,
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
        // Set weight
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



        let mut data: Vec<u8> =
            solana_program::hash::hash(b"global:reset_epoch_gauge_voter").to_bytes()[..8].to_vec();
        data.extend_from_slice(&weight.weight.to_le_bytes());
        let reset_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                AccountMeta {
                    pubkey: GAUGEMEISTER,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: LOCKER,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: *escrow,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: vote_accounts.gauge_voter,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: vote_accounts.epoch_gauge_voter,
                    is_signer: false,
                    is_writable: true,
                },
            ],
            data,
        };
        let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
            &[reset_ix],
            Some(&script_authority.pubkey()),
        );
        let latest_blockhash = client.get_latest_blockhash().unwrap();
        transaction.sign(&[script_authority], latest_blockhash);
        let result = client.send_and_confirm_transaction(&transaction).unwrap();
        println!("reset epoch gauge voter result: {:?}", result);
        // Commit vote

        let mut data: Vec<u8> =
            solana_program::hash::hash(b"global:gauge_commit_vote_v2").to_bytes()[..8].to_vec();
        data.extend_from_slice(&weight.weight.to_le_bytes());
        let commit_vote_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                AccountMeta {
                    pubkey: GAUGEMEISTER,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: weight.gauge,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: vote_accounts.gauge_voter,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: vote_accounts.gauge_vote,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: vote_accounts.epoch_gauge,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: vote_accounts.epoch_gauge_voter,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey:  vote_accounts.epoch_gauge_vote,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: script_authority.pubkey(),
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: solana_program::system_program::id(),
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data,
        };
        let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
            &[commit_vote_ix],
            Some(&script_authority.pubkey()),
        );
        let latest_blockhash = client.get_latest_blockhash().unwrap();
        transaction.sign(&[script_authority], latest_blockhash);
        println!("Trying to send with skipping preflight");
        let result = client.send_and_confirm_transaction_with_spinner_and_config(&transaction,
        CommitmentConfig::confirmed(),
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        }).unwrap();
        println!("result: {:?}", result);
        println!("transaction: {:?}", transaction.signatures.first().unwrap());
    }

}
