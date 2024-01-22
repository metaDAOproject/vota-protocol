use crate::actions::escrows;
use clap::{value_parser};
use dotenv::dotenv;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

mod actions;
mod accounts;

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
const GAUGEMEISTER: Pubkey = pubkey!("28ZDtf6d2wsYhBvabTxUHTRT6MDxqjmqR7RMCp348tyU");
const LOCKER: Pubkey = pubkey!("8erad8kmNrLJDJPe9UkmTHomrMV3EW48sjGeECyVjbYX");

fn main() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap().to_string();
    println!("rpc_url: {:?}", rpc_url);
    let client = solana_client::rpc_client::RpcClient::new(rpc_url);
    // let args = Args::parse();
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
                    clap::arg!(-k --keypair <FILE> "The escrow owner keypair")
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
                .arg(
                    clap::arg!(-e --escrow <PUBKEY> "The escrow account to delegate")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    clap::Arg::new("delegate")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("The delegate to delegate the escrow to"),
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
                )
        );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("get-escrows", _)) => {
            escrows::get_delegated_escrows(client);
        }
        Some(("delegate", matches)) => {
            let escrow = Pubkey::from_str(matches.get_one::<String>("escrow").unwrap()).unwrap();
            // print!("escrow: {:?}", escrow_string);
            let delegate =
                Pubkey::from_str(matches.get_one::<String>("delegate").unwrap()).unwrap();
            let keypair_path = matches.get_one::<PathBuf>("keypair").unwrap();
            let keypair = solana_sdk::signature::read_keypair_file(keypair_path).unwrap();
            actions::delegate::delegate(client, &escrow, &delegate, &keypair);
        }
        Some(("get-escrow", matches)) => {
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap()).unwrap();
            let escrow = accounts::resolve::get_escrow_address_for_owner(&owner);
            println!("{}", escrow);
        }
        Some(("get-vote-buys", matches)) => {
            let config = Pubkey::from_str(matches.get_one::<String>("config").unwrap()).unwrap();
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            let vote_buys = actions::vote_buys::get_all_vote_buys(*epoch, config);
            println!("vote buys: {:?}", vote_buys);
        }
        Some(("prepare-vote", matches)) => {
            println!("prepare-vote");
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap()).unwrap();
            let gauge = Pubkey::from_str(matches.get_one::<String>("gauge").unwrap()).unwrap();
            let epoch = matches.get_one::<u32>("epoch").unwrap();
            actions::prepare_vote::prepare_vote(&client, owner, gauge, *epoch);
        }
        _ => {
            println!("no subcommand matched")
        }
    };
}
