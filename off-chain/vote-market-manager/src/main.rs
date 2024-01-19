use crate::actions::escrows;
use clap::{value_parser, Parser};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::path::PathBuf;
use std::str::FromStr;
use std::env;
use dotenv::dotenv;

mod actions;

fn main() {
    dotenv().ok();
    let GAUGEMEISTER = pubkey!("28ZDtf6d2wsYhBvabTxUHTRT6MDxqjmqR7RMCp348tyU");
    let MY_ESCROW = pubkey!("C8CMP5RkRQneDtYruTNWbRXihorfXpYh7JdEXjia1DJL");

    let rpc_url = env::var("RPC_URL").unwrap().to_string();
    print!("rpc_url: {:?}", rpc_url);
    let client = solana_client::rpc_client::RpcClient::new(rpc_url);
    // let args = Args::parse();
    let cmd = clap::Command::new("vote-market-manager")
        .bin_name("vote-market-manager")
        .subcommand(clap::command!("get"))
        .subcommand(clap::command!("get-escrow")
            .arg(
                clap::Arg::new("owner")
                    .required(true)
                    .value_parser(value_parser!(String))
                    .help("The owner of the escrow to get"),
            ))
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
        );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("get", matches)) => {
            escrows::get_delegated_escrows(client);
        }
        Some(("delegate", matches)) => {
            let escrow = Pubkey::from_str(matches.get_one::<String>("escrow").unwrap()).unwrap();
            // print!("escrow: {:?}", escrow_string);
            let delegate =
                Pubkey::from_str(matches.get_one::<String>("delegate").unwrap()).unwrap();
            let keypair_path = matches.get_one::<PathBuf>("keypair").unwrap();
            let keypair = solana_sdk::signature::read_keypair_file(&keypair_path).unwrap();
            actions::delegate::delegate(client, &escrow, &delegate, &keypair);
        }
        Some(("get-escrow", matches)) => {
            let owner = Pubkey::from_str(matches.get_one::<String>("owner").unwrap()).unwrap();
            let escrow = escrows::get_escrow_address_for_owner(client, owner);
            println!("{}",escrow);
        }
        _ => {
            println!("no subcommand matched")
        }
    };
}
