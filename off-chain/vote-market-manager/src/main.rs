use std::str::FromStr;
use std::{env, fs};

use anchor_lang::AnchorDeserialize;
use chrono::Utc;
use clap::value_parser;
use dotenv::dotenv;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use structured_logger::Builder;
use structured_logger::json::new_writer;

use crate::accounts::resolve::{get_delegate, get_escrow_address_for_owner};
use crate::actions::management::data::{VoteInfo};
use crate::actions::queries::escrows;
use crate::utils::short_address;

mod accounts;
mod actions;
mod errors;
mod utils;

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
const GAUGEMEISTER: Pubkey = pubkey!("28ZDtf6d2wsYhBvabTxUHTRT6MDxqjmqR7RMCp348tyU");
const LOCKER: Pubkey = pubkey!("8erad8kmNrLJDJPe9UkmTHomrMV3EW48sjGeECyVjbYX");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    Builder::with_level("info")
        .with_target_writer("*", new_writer(fs::File::create(
            format!(
                "./vote_market_{}.log",
                Utc::now().format("%Y-%m-%d-%H_%M")
            ))?))
        .init();
    let cmd = clap::Command::new("vote-market-manager")
        .bin_name("vote-market-manager")
        .arg(
            clap::Arg::new("keypair")
                .long("keypair")
                .short('k')
                .required(false)
                .value_parser(value_parser!(String))
                .help("The keypair to use for the payer")
                .global(true),
        )
        .subcommand(
            clap::command!("get-escrows").arg(
                clap::Arg::new("config")
                    .required(true)
                    .value_parser(value_parser!(String))
                    .help("The config to calculate the escrow delegate"),
            ),
        )
        .subcommand(
            clap::command!("get-direct-votes").arg(
                clap::Arg::new("epoch")
                    .required(true)
                    .value_parser(value_parser!(u32))
                    .help("The epoch to get the direct votes for"),
            ),
        )
        .subcommand(
            clap::command!("get-vote-buys")
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("the config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("the epoch for the vote buy accounts"),
                ),
        )
        .subcommand(
            clap::command!("get-escrow").arg(
                clap::Arg::new("owner")
                    .required(true)
                    .value_parser(value_parser!(String))
                    .help("The owner of the escrow to get"),
            ),
        )
        .subcommand(
            clap::command!("delegate").arg(
                clap::Arg::new("config")
                    .required(true)
                    .value_parser(value_parser!(String))
                    .help("The config to generate the delegate from"),
            ),
        )
        .subcommand(
            clap::command!("prepare-vote")
                .arg(
                    clap::Arg::new("owner")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The owner of the escrow to vote for"),
                )
                .arg(
                    clap::Arg::new("gauge")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The gauge voting for"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to vote for"),
                ),
        )
        .subcommand(
            clap::command!("create-epoch-gauge")
                .arg(
                    clap::Arg::new("gauge")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The gauge to create the epoch gauge for"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to create the gauge for"),
                ),
        )
        .subcommand(
            clap::command!("vote-test")
                .arg(
                    clap::Arg::new("escrow")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The escrow to vote for"),
                )
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to vote for"),
                ),
        )
        .subcommand(
            clap::command!("setup").arg(
                clap::Arg::new("mints")
                    .long("mints")
                    .short('m')
                    .required(false)
                    .value_delimiter(',')
                    .value_parser(value_parser!(String))
                    .help("The mints to allow for the vote buys"),
            ),
        )
        .subcommand(
            clap::command!("update-mints")
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config to update the mints for"),
                )
                .arg(
                    clap::Arg::new("mints")
                        .long("mints")
                        .short('m')
                        .required(true)
                        .value_delimiter(',')
                        .value_parser(value_parser!(String))
                        .help("The mints to allow for the vote buys"),
                ),
        )
        .subcommand(clap::command!("create-token"))
        .subcommand(
            clap::command!("buy-votes")
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("gauge")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The gauge buy votes for"),
                )
                .arg(
                    clap::Arg::new("mint")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The amount of tokens to buy votes for"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to vote for"),
                )
                .arg(
                    clap::Arg::new("amount")
                        .required(true)
                        .value_parser(value_parser!(u64))
                        .help("The amount of tokens to buy votes for"),
                ),
        )
        .subcommand(
            clap::command!("get-refund")
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("gauge")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The gauge buy votes for"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to vote for"),
                ),

        )
        .subcommand(
            clap::command!("set-maximum")
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("gauge")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The gauge buy votes for"),
                )
                .arg(
                    clap::Arg::new("max")
                        .required(true)
                        .value_parser(value_parser!(u64))
                        .help("The maximum amount of tokens to buy votes for"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to vote for"),
                ),
        )
        .subcommand(clap::command!("trigger-epoch"))
        .subcommand(
            clap::command!("claim")
                .arg(
                    clap::Arg::new("mint")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The mint for the token to claim"),
                )
                .arg(
                    clap::Arg::new("escrow")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The escrow to claim for"),
                )
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("gauge")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The gauge claim vote payments for"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to vote for"),
                ),
        )
        .subcommand(
            clap::command!("calculate-inputs")
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config for the vote buy accounts"),
                )
                .arg(
                    clap::Arg::new("epoch")
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .help("The epoch to calculate inputs for"),
                ),
        )
        .subcommand(
            clap::command!("calculate-weights").arg(
                clap::Arg::new("epoch-data")
                    .required(true)
                    .value_parser(value_parser!(String))
                    .help("The data file output by the calculate-inputs subcommand"),
            ),
        )
        .subcommand(
            clap::command!("find-max-vote-buy")
                .arg(
                    clap::Arg::new("epoch-data")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The data file output by the calculate-inputs subcommand"),
                )
                .arg(
                    clap::Arg::new("vote-weights")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The vote weights file output by the calculate-weights subcommand"),
                ),
        )
        .subcommand(
            clap::command!("execute-votes")
                .arg(
                    clap::Arg::new("epoch-data")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The data file output by the calculate-inputs subcommand"),
                )
                .arg(
                    clap::Arg::new("vote-weights")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The vote weights file output by the calculate-weights subcommand"),
                ),
        )
        .subcommand(
            clap::command!("execute-claim")
                .arg(
                    clap::Arg::new("config").
                    required(true).
                    value_parser(value_parser!(String)).
                    help("The config to claim for"),
                )
                .arg(
                    clap::Arg::new("epoch").
                    required(true).
                    value_parser(value_parser!(u32)).
                    help("The epoch to claim for"),
                )

        );

    let matches = cmd.get_matches();
    let keypair = matches.get_one::<String>("keypair");
    let keypair_path = match keypair {
        Some(keypair) => keypair.to_string(),
        None => env::var("KEY_PATH2")?.to_string(),
    };
    let rpc_url = env::var("RPC_URL").unwrap().to_string();
    let rpc_wss_url = env::var("RPC_WSS_URL").unwrap().to_string();
    println!("rpc_url: {:?}", rpc_url);
    let client = solana_client::rpc_client::RpcClient::new(&rpc_url);
    let payer = solana_sdk::signature::read_keypair_file(keypair_path)?;
    println!("payer: {:?}", payer.pubkey());
    let anchor_client = anchor_client::Client::new_with_options(
        anchor_client::Cluster::Custom(rpc_url.clone(), rpc_wss_url),
        &payer,
        CommitmentConfig::confirmed(),
    );
    let program = anchor_client.program(vote_market::id())?;
    if rpc_url == "http://127.0.0.1:8899" || rpc_url == "http://localhost:8899" {
        // Make sure we have some funds
        let amount = program.rpc().get_balance(&payer.pubkey())?;
        if amount == 0 {
            println!("Airdropping 100 SOL");
            let sig = program
                .rpc()
                .request_airdrop(&payer.pubkey(), 100_000_000_000)?;
            let blockhash = program.rpc().get_latest_blockhash()?;
            program.rpc().confirm_transaction_with_spinner(
                &sig,
                &blockhash,
                CommitmentConfig {
                    commitment: solana_sdk::commitment_config::CommitmentLevel::Finalized,
                },
            )?;
        }
    }
    match matches.subcommand() {
        Some(("get-escrows", matches)) => {
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let delegate = get_delegate(&config);
            let escrows = escrows::get_delegated_escrows(&client, &delegate);
            println!("escrows: {:?}", escrows);
        }
        Some(("delegate", matches)) => {
            let escrow = get_escrow_address_for_owner(&payer.pubkey());
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let delegate = accounts::resolve::get_delegate(&config);
            println!("delegate: {:?}", delegate);
            actions::delegate::delegate(client, &escrow, &delegate, &payer);
        }
        Some(("get-escrow", matches)) => {
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap())?;
            let escrow = accounts::resolve::get_escrow_address_for_owner(&owner);
            println!("{}", escrow);
        }
        Some(("get-direct-votes", matches)) => {
            // Update this with the latest from the file from calculate-inputs
            let usd_per_vote = 9.699848367796068e-12;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let direct_votes = actions::queries::direct_votes::get_direct_votes(&client, *epoch)?;
            let mut total_votes = 0;
            for eg in direct_votes {
                let gauge_account = client.get_account(&eg.gauge)?;
                let gauge_data =
                    gauge_state::Gauge::deserialize(&mut gauge_account.data[8..].as_ref())?;
                let quarry_address = gauge_data.quarry;
                let quarry_account = client.get_account(&quarry_address)?;
                let quarry_data =
                    quarry_state::Quarry::deserialize(&mut quarry_account.data[8..].as_ref())?;
                let quarry_mint = quarry_data.token_mint_key;
                total_votes += eg.total_power;
                println!(
                    "Pool Mint: {:?}, Gauge: {:?}, Power: {:?}, USD value of votes: {:?}",
                    short_address(&quarry_mint),
                    &eg.gauge,
                    eg.total_power,
                    eg.total_power as f64 * usd_per_vote
                );
            }
            println!("Total votes: {:?}", total_votes);
        }
        Some(("get-vote-buys", matches)) => {
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let vote_buys = actions::queries::vote_buys::get_all_vote_buys(*epoch, &config);
            println!("vote buys: {:?}", vote_buys);
        }
        Some(("prepare-vote", matches)) => {
            println!("prepare-vote");
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap())?;
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            actions::prepare_vote::prepare_vote(&client, owner, gauge, &payer, *epoch);
        }
        Some(("create-epoch-gauge", matches)) => {
            println!("create-epoch-gauge");
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            actions::create_epoch_gauge::create_epoch_gauge(&client, &payer, gauge, *epoch);
        }
        Some(("vote-test", matches)) => {
            println!("vote-test");
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let escrow = Pubkey::from_str(matches.get_one::<String>("escrow").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let weights = vec![VoteInfo {
                gauge: Pubkey::from_str("3xC4eW6xhW3Gpb4T5sCKFe73ay2K4aUUfxL57XFdguJx")?,
                weight: 100,
                votes: 100,
            }];
            actions::vote_market::vote::vote(
                &anchor_client,
                &client,
                &payer,
                config,
                escrow,
                *epoch,
                weights,
            )?;
        }
        Some(("setup", matches)) => {
            println!("setup");
            let mut mints = vec![Pubkey::default()];
            if let Some(mint_vaulues) = matches.get_many::<String>("mints") {
                mints = mint_vaulues
                    .map(|mint| Pubkey::from_str(mint).unwrap())
                    .collect();
            }
            actions::vote_market::setup::setup(&anchor_client, mints, &payer);
        }
        Some(("update-mints", matches)) => {
            println!("update-mints");
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let mut mints = vec![Pubkey::default()];
            if let Some(mint_vaulues) = matches.get_many::<String>("mints") {
                mints = mint_vaulues
                    .map(|mint| Pubkey::from_str(mint).unwrap())
                    .collect();
            }
            println!("mints: {:?}", mints);
            actions::vote_market::update_mints::update_mints(&anchor_client, &payer, config, mints);
        }
        Some(("create-token", _)) => {
            println!("create-token");
            actions::create_token::create_token(&client, &payer);
        }
        Some(("buy-votes", matches)) => {
            //TODO: bring out epoch
            println!("buy-votes");
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap())?;
            let mint = Pubkey::from_str(matches.get_one::<String>("mint").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let amount = matches.get_one::<u64>("amount").unwrap();
            actions::vote_market::buy_votes::buy_votes(
                &anchor_client,
                &payer,
                &config,
                &gauge,
                &mint,
                *epoch,
                *amount,
            );
        }
        Some(("get-refund", matches)) => {
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            actions::vote_market::refund::get_refund(
                &anchor_client,
                &payer,
                config,
                gauge,
                *epoch,
            );
        }
        Some(("set-maximum", matches)) => {
            //TODO: bring out epoch
            let maximum = matches.get_one::<u64>("max").unwrap();
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            actions::vote_market::set_maximum::set_maximum(
                &anchor_client,
                &payer,
                gauge,
                config,
                *epoch,
                *maximum,
            );
        }
        Some(("trigger-epoch", _)) => {
            actions::trigger_epoch::trigger_epoch(&client, &payer);
        }
        Some(("claim", matches)) => {
            let mint = Pubkey::from_str(matches.get_one::<String>("mint").unwrap())?;
            let escrow = Pubkey::from_str(matches.get_one::<String>("escrow").unwrap())?;
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();

            actions::vote_market::claim::claim(
                &anchor_client,
                &client,
                &payer,
                payer.pubkey(),
                mint,
                escrow,
                config,
                gauge,
                *epoch,
            );
        }
        Some(("calculate-inputs", matches)) => {
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            actions::management::calculate_inputs::calculate_inputs(&client, &config, *epoch)?;
        }
        Some(("calculate-weights", matches)) => {
            let epoch_data = matches.get_one::<String>("epoch-data").unwrap();
            let epoch_data_string = std::fs::read_to_string(epoch_data)?;
            let mut data: actions::management::data::EpochData =
                serde_json::from_str(&epoch_data_string)?;
            let results =
                actions::management::calculate_weights::calculate_weights(&mut data)?;
            println!("results {:?}", results);
            let vote_weights_json = serde_json::to_string(&results).unwrap();
            fs::write(
                format!(
                    "./epoch_{}_weights{}.json",
                    data.epoch,
                    Utc::now().format("%Y-%m-%d-%H_%M")
                ),
                vote_weights_json,
            )?;
        }
        Some(("find-max-vote-buy", matches)) => {
            let epoch_data = matches.get_one::<String>("epoch-data").unwrap();
            let epoch_data_string = std::fs::read_to_string(epoch_data)?;
            let data: actions::management::data::EpochData =
                serde_json::from_str(&epoch_data_string)?;
            let vote_weights_file = matches.get_one::<String>("vote-weights").unwrap();
            let vote_weights_string = std::fs::read_to_string(vote_weights_file)?;
            let vote_weights: Vec<VoteInfo>=
                serde_json::from_str(&vote_weights_string)?;
            actions::management::find_max_vote_buy::find_max_vote_buy(
                &client,
                &anchor_client,
                &payer,
                data,
                vote_weights,
            )?;
        }
        Some(("execute-votes", matches)) => {
            let epoch_data = matches.get_one::<String>("epoch-data").unwrap();
            let epoch_data_string = std::fs::read_to_string(epoch_data)?;
            let data: actions::management::data::EpochData =
                serde_json::from_str(&epoch_data_string)?;
            let vote_weights_file = matches.get_one::<String>("vote-weights").unwrap();
            let vote_weights_string = std::fs::read_to_string(vote_weights_file)?;
            let vote_infos: Vec<VoteInfo> = serde_json::from_str(&vote_weights_string)?;
            actions::management::execute_votes::execute_votes(
                &client,
                &anchor_client,
                &payer,
                data,
                vote_infos,
            )?;
        }
        Some(("execute-claim", matches)) => {
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap())?;
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            actions::management::execute_claim::execute_claim(
                &client,
                &anchor_client,
                &payer,
                config,
                *epoch,
            )?;
        }
        _ => {
            println!("No subcommand");
        }
    };
    Ok(())
}
