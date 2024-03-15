use crate::ANCHOR_DISCRIMINATOR_SIZE;
use anchor_lang::AnchorDeserialize;
use gauge_state::{EpochGauge, Gauge};
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey;
use quarry_state::Quarry;

pub(crate) fn get_direct_votes(
    client: &RpcClient,
    epoch: u32,
) -> Result<Vec<EpochGauge>, Box<dyn std::error::Error>> {
    let accounts = client.get_program_accounts_with_config(
        &gauge_state::id(),
        RpcProgramAccountsConfig {
            filters: Some(vec![
                DataSize((ANCHOR_DISCRIMINATOR_SIZE + EpochGauge::LEN) as u64),
                RpcFilterType::Memcmp(Memcmp::new(
                    ANCHOR_DISCRIMINATOR_SIZE + 32,
                    MemcmpEncodedBytes::Bytes(epoch.to_le_bytes().to_vec()),
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
    )?;
    let gauge = pubkey!("3xC4eW6xhW3Gpb4T5sCKFe73ay2K4aUUfxL57XFdguJx");
    let gauge_account = client.get_account(&gauge).unwrap();
    let gauge_data = Gauge::deserialize(&mut &gauge_account.data[8..])?;
    println!("epoch_guage: {:?}", gauge_data);
    let quarry_account = client.get_account(&gauge_data.quarry).unwrap();
    let quarry_data = Quarry::deserialize(&mut &quarry_account.data[8..])?;
    println!("quarry: {:?}", quarry_data.token_mint_key);

    accounts
        .iter()
        .map(|(_pubkey, account)| {
            let epoch_guage = EpochGauge::deserialize(&mut &account.data[8..])?;
            Ok(epoch_guage)
        })
        .collect()
}
