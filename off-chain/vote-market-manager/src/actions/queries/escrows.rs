use crate::ANCHOR_DISCRIMINATOR_SIZE;
use anchor_lang::{AccountDeserialize, AnchorDeserialize};
use saber_locker::Escrow;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::account;
use gauge_state::EpochGaugeVote;
use crate::accounts::resolve::resolve_vote_keys;
use crate::utils::get_multiple_accounts;

pub fn get_delegated_escrows(client: &RpcClient, delegate: &Pubkey) -> Vec<(Pubkey, Escrow)> {
    let accounts = client
        .get_program_accounts_with_config(
            &saber_locker::id(),
            RpcProgramAccountsConfig {
                filters: Some(vec![
                    DataSize((ANCHOR_DISCRIMINATOR_SIZE + Escrow::LEN) as u64),
                    RpcFilterType::Memcmp(Memcmp::new(
                        129,
                        MemcmpEncodedBytes::Bytes(delegate.to_bytes().to_vec()),
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
    let mut escrows: Vec<(Pubkey, Escrow)> = Vec::new();
    for (key, account) in accounts {
        if let Ok(parsed_account) = Escrow::try_deserialize(&mut account.data.as_slice()) {
            escrows.push((key, parsed_account));
        }
    }
    escrows
}


pub(crate) fn get_escrow_votes(client: &RpcClient, delegate: &Pubkey, gauge: &Pubkey, epoch: u32 ) -> () {
    let escrows = get_delegated_escrows(client, &delegate);
    let mut epoch_gauge_votes: Vec<Pubkey> = Vec::new();
    for (key, escrow) in escrows.clone() {
        let vote_accounts = resolve_vote_keys(&key, gauge, epoch);
        let gauge_vote = vote_accounts.epoch_gauge_vote;
        epoch_gauge_votes.push(gauge_vote);
    }
    let epoch_gauge_vote_accounts = get_multiple_accounts(client, epoch_gauge_votes);
    let mut total_power: u64 = 0;
    for (index, account) in epoch_gauge_vote_accounts.iter().enumerate() {


        let epoch_gauge_vote_data: Option<EpochGaugeVote> = match account {
            Some(account) => {
                Some(EpochGaugeVote::try_deserialize(&mut account.data.as_slice()).unwrap())
            },
            None => {
                None
            }
        };
        match epoch_gauge_vote_data {
            Some(data) => {
                println!("account: {:?}, vote: {:?}", escrows[index].1.owner, data.allocated_power);
                total_power += data.allocated_power;
            },
            None => {
                println!("account: {:?}, Hasn't voted", escrows[index].1.owner);
            }
        }
    }
    println!("total power: {:?}", total_power);

}