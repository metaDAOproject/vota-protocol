use crate::{ANCHOR_DISCRIMINATOR_SIZE, GAUGEMEISTER};
use anchor_lang::AccountDeserialize;
use gauge_state::Gauge;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use std::env;
use vote_market::state::VoteBuy;

pub fn get_all_vote_buys(epoch: u32, config: &Pubkey) -> Vec<VoteBuy> {
    let rpc_url = env::var("RPC_URL").unwrap().to_string();
    let client = solana_client::rpc_client::RpcClient::new(rpc_url);
    let accounts = client
        .get_program_accounts_with_config(
            &gauge_state::id(),
            RpcProgramAccountsConfig {
                filters: Some(vec![
                    DataSize((ANCHOR_DISCRIMINATOR_SIZE + Gauge::LEN) as u64),
                    // Check if the gauge is for the gaugemeister
                    RpcFilterType::Memcmp(Memcmp::new(
                        ANCHOR_DISCRIMINATOR_SIZE,
                        MemcmpEncodedBytes::Bytes(GAUGEMEISTER.to_bytes().to_vec()),
                    )),
                    //Check if the gauge is enabled
                    // RpcFilterType::Memcmp(Memcmp::new(
                    //     ANCHOR_DISCRIMINATOR_SIZE + 32 * 2,
                    //     MemcmpEncodedBytes::Bytes((false as u8).to_le_bytes().to_vec()),
                    // )),
                ]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: None,
                    data_slice: None,
                    min_context_slot: None,
                },
                with_context: None,
            },
        )
        .unwrap();
    let mut vote_buy_addresses: Vec<Pubkey> = Vec::new();
    let mut vote_buy_parsed_accounts: Vec<VoteBuy> = Vec::new();
    for (key, _) in &accounts {
        println!("{:?}",key);
        let vote_buy_address = Pubkey::find_program_address(
            &[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                config.to_bytes().as_ref(),
                key.to_bytes().as_ref(),
            ],
            &vote_market::id(),
        )
        .0;
        vote_buy_addresses.push(vote_buy_address);
    }
    let vote_buy_accounts = client.get_multiple_accounts(&vote_buy_addresses).unwrap();
    for vote_buy_account in vote_buy_accounts.into_iter().flatten() {
        let parsed_vote_buy = VoteBuy::try_deserialize(&mut &vote_buy_account.data[..]).unwrap();
        vote_buy_parsed_accounts.push(parsed_vote_buy);
    }
    vote_buy_parsed_accounts
}
