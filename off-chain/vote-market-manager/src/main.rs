use crate::actions::queries::escrows;

use clap::value_parser;
use dotenv::dotenv;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

mod accounts;
mod actions;

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
const GAUGEMEISTER: Pubkey = pubkey!("28ZDtf6d2wsYhBvabTxUHTRT6MDxqjmqR7RMCp348tyU");
const LOCKER: Pubkey = pubkey!("8erad8kmNrLJDJPe9UkmTHomrMV3EW48sjGeECyVjbYX");

fn main() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap().to_string();
    let keypair_path = env::var("KEY_PATH").unwrap().to_string();
    println!("rpc_url: {:?}", rpc_url);
    let client = solana_client::rpc_client::RpcClient::new(rpc_url);
    let payer = solana_sdk::signature::read_keypair_file(keypair_path).unwrap();
    let anchor_client = anchor_client::Client::new_with_options(
        anchor_client::Cluster::Localnet,
        &payer,
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );
    let program = anchor_client.program(vote_market::id()).unwrap();

    // TODO: Can't do this if mainnet
    // Make sure we have some funds
    let amount = program.rpc().get_balance(&payer.pubkey()).unwrap();
    if amount == 0 {
        println!("Airdropping 100 SOL");
        let sig = program
            .rpc()
            .request_airdrop(&payer.pubkey(), 100_000_000_000)
            .unwrap();
        let blockhash = program.rpc().get_latest_blockhash().unwrap();
        program
            .rpc()
            .confirm_transaction_with_spinner(
                &sig,
                &blockhash,
                CommitmentConfig {
                    commitment: solana_sdk::commitment_config::CommitmentLevel::Finalized,
                },
            )
            .unwrap();
    }

    let cmd = clap::Command::new("vote-market-manager")
        .bin_name("vote-market-manager")
        .subcommand(clap::command!("get-escrows"))
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
            clap::command!("delegate")
                .arg(
                    clap::Arg::new("escrow")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The escrow to set the delegate for"),
                )
                .arg(
                    clap::Arg::new("config")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The config to generate the delegate from"),
                ),
        )
        .subcommand(
            clap::command!("prepare-vote")
                .arg(
                    clap::arg!(-k --keypair <FILE> "The payer keypair")
                        .value_parser(value_parser!(PathBuf)),
                )
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
            clap::command!("vote")
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
                    clap::Arg::new("amount")
                        .required(true)
                        .value_parser(value_parser!(u64))
                        .help("The amount of tokens to buy votes for"),
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
        );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("get-escrows", _)) => {
            escrows::get_delegated_escrows(client);
        }
        Some(("delegate", matches)) => {
            let escrow = Pubkey::from_str(matches.get_one::<String>("escrow").unwrap()).unwrap();
            // print!("escrow: {:?}", escrow_string);
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap()).unwrap();
            let delegate = accounts::resolve::get_delegate(&config);
            print!("delegate: {:?}", delegate);
            actions::delegate::delegate(client, &escrow, &delegate, &payer);
        }
        Some(("get-escrow", matches)) => {
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap()).unwrap();
            let escrow = accounts::resolve::get_escrow_address_for_owner(&owner);
            println!("{}", escrow);
        }
        Some(("get-vote-buys", matches)) => {
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap()).unwrap();
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let vote_buys = actions::queries::vote_buys::get_all_vote_buys(*epoch, config);
            println!("vote buys: {:?}", vote_buys);
        }
        Some(("prepare-vote", matches)) => {
            println!("prepare-vote");
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap()).unwrap();
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap()).unwrap();
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let keypair_path = matches.get_one::<PathBuf>("keypair").unwrap();
            let keypair = solana_sdk::signature::read_keypair_file(keypair_path).unwrap();
            actions::prepare_vote::prepare_vote(&client, &owner, &gauge, &keypair, *epoch);
        }
        Some(("vote", matches)) => {
            println!("vote");
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap()).unwrap();
            let escrow = Pubkey::from_str(matches.get_one::<String>("escrow").unwrap()).unwrap();
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let weights = vec![actions::vote_market::vote::WeightInfo {
                gauge: Pubkey::from_str("3xC4eW6xhW3Gpb4T5sCKFe73ay2K4aUUfxL57XFdguJx").unwrap(),
                weight: 100,
            }];
            actions::vote_market::vote::vote(
                &anchor_client,
                &payer,
                &config,
                &escrow,
                *epoch,
                weights,
            );
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
        Some(("create-token", _)) => {
            println!("create-token");
            actions::create_token::create_token(&client, &payer);
        }
        Some(("buy-votes", matches)) => {
            //TODO: bring out epoch
            println!("buy-votes");
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap()).unwrap();
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap()).unwrap();
            let mint = Pubkey::from_str(matches.get_one::<String>("mint").unwrap()).unwrap();
            let amount = matches.get_one::<u64>("amount").unwrap();
            actions::vote_market::buy_votes::buy_votes(
                &anchor_client,
                &payer,
                &config,
                &gauge,
                &mint,
                96,
                *amount,
            );
        }
        Some(("set-maximum", matches)) => {
            //TODO: bring out epoch
            let maximum = matches.get_one::<u64>("max").unwrap();
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap()).unwrap();
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap()).unwrap();
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
        _ => {
            println!("no subcommand matched")
        }
    };
}
