use std::{env, fs};
use anchor_lang::AccountDeserialize;
use serde_json::Value;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use gauge_state::Gauge;
use quarry_state::Quarry;
use crate::{ANCHOR_DISCRIMINATOR_SIZE, GAUGEMEISTER};

pub fn get_gauges(client: RpcClient) -> () {
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
                    // Check if the gauge is enabled
                    RpcFilterType::Memcmp(Memcmp::new(
                        ANCHOR_DISCRIMINATOR_SIZE + 32 * 2,
                        MemcmpEncodedBytes::Bytes((false as u8).to_le_bytes().to_vec()),
                    )),
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
    let quary_info = fs::read_to_string("quarry_info.json").unwrap();
    let quarry_info: Value = serde_json::from_str(&quary_info).unwrap();
    for (key, _) in &accounts {
        let gauge_account = client.get_account(&key).unwrap();
        let gauge_data = Gauge::try_deserialize(&mut &gauge_account.data[..]).unwrap();
        let quarry_account = client.get_account(&gauge_data.quarry).unwrap();
        let quarry_data = Quarry::try_deserialize(&mut &quarry_account.data[..]).unwrap();
        //find index where quarry_data.token_mint_key matches quarry_info["tokens"]["address"]
        let mut index = 0;
        for (i, token) in quarry_info["tokens"].as_array().unwrap().iter().enumerate() {
            if quarry_data.token_mint_key.to_string() == token["address"].as_str().unwrap() {
                index = i;
                break;
            }
        }
        if let Some(quarry) = quarry_info["tokens"][index]["symbol"].as_str()
        {
            println!("{:?} {:?}", key, quarry);
        }
    }
}
