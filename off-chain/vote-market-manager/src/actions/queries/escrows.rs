use crate::ANCHOR_DISCRIMINATOR_SIZE;
use anchor_lang::AccountDeserialize;
use locked_voter_state::Escrow;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};

pub fn get_delegated_escrows(client: RpcClient) {
    let accounts = client
        .get_program_accounts_with_config(
            &locked_voter_state::id(),
            RpcProgramAccountsConfig {
                filters: Some(vec![
                    DataSize((ANCHOR_DISCRIMINATOR_SIZE + Escrow::LEN) as u64),
                    RpcFilterType::Memcmp(Memcmp::new(
                        129,
                        MemcmpEncodedBytes::Base58(
                            "5GhPyownvAAbnxt3qt3JmaBeGNM9DdmR6Xv8y729SK94".to_string(),
                        ),
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
    println!("account len: {:?}", accounts.len());
    for (_, account) in accounts {
        println!("account: {:?}", account);
        //parse escrow data
        let parsed_account = Escrow::try_deserialize(&mut account.data.as_slice()).unwrap();
        println!("parsed account: {:?}", parsed_account);
    }

    println!("get gauges");
}
