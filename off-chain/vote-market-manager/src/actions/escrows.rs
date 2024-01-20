use crate::{ANCHOR_DISCRIMINATOR_SIZE, LOCKER, LOCKER_PROGRAM};
use anchor_lang::AccountDeserialize;
use locked_voter_state::Escrow;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;

pub fn get_escrow_address_for_owner(client: RpcClient, owner: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"Escrow".as_ref(), LOCKER.as_ref(), owner.as_ref()],
        &LOCKER_PROGRAM,
    )
    .0
}

pub fn get_delegated_escrows(client: RpcClient) {
    let accounts = client
        .get_program_accounts_with_config(
            &LOCKER_PROGRAM,
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
    for (key, account) in accounts {
        println!("account: {:?}", account);
        //parse escrow data
        let parsed_account = Escrow::try_deserialize(&mut account.data.as_slice()).unwrap();
        println!("parsed account: {:?}", parsed_account);
    }

    println!("get gauges");
}
