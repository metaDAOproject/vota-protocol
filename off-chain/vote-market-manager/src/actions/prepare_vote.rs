use crate::accounts::resolve::{get_escrow_address_for_owner, resolve_vote_keys, VoteCreateStep};
use crate::{GAUGEMEISTER, LOCKER};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;

pub fn prepare_vote(
    client: &RpcClient,
    owner: &Pubkey,
    gauge: &Pubkey,
    payer: &Keypair,
    epoch: u32,
) {
    let escrow_address = get_escrow_address_for_owner(&owner);
    println!("Prepare vote for escrow: {:?}", escrow_address);
    let vote_keys = resolve_vote_keys(&escrow_address, gauge, epoch);
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
                        AccountMeta {
                            pubkey: key,
                            is_signer: false,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: GAUGEMEISTER,
                            is_signer: false,
                            is_writable: false,
                        },
                        AccountMeta {
                            pubkey: escrow_address,
                            is_signer: false,
                            is_writable: false,
                        },
                        AccountMeta {
                            pubkey: payer.pubkey(),
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
                    &[create_gauge_voter_ix],
                    Some(&payer.pubkey()),
                );
                let latest_blockhash = client.get_latest_blockhash().unwrap();
                transaction.sign(&[payer], latest_blockhash);
                let result = client.send_and_confirm_transaction(&transaction).unwrap();
                println!("result: {:?}", result);
                println!("transaction: {:?}", transaction.signatures.first().unwrap());
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
                        AccountMeta {
                            pubkey: key,
                            is_signer: false,
                            is_writable: true,
                        },
                        AccountMeta {
                            pubkey: vote_keys.gauge_voter,
                            is_signer: false,
                            is_writable: false,
                        },
                        AccountMeta {
                            pubkey: *gauge,
                            is_signer: false,
                            is_writable: false,
                        },
                        AccountMeta {
                            pubkey: payer.pubkey(),
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
                    &[create_gauge_vote_ix],
                    Some(&payer.pubkey()),
                );
                let latest_blockhash = client.get_latest_blockhash().unwrap();
                transaction.sign(&[payer], latest_blockhash);
                let result = client.send_and_confirm_transaction(&transaction).unwrap();
                println!("result: {:?}", result);
                println!("transaction: {:?}", transaction.signatures.first().unwrap());
            }
            VoteCreateStep::EpochGaugeVoter(key) => {
            }
            _ => {}
        }
    }
}
