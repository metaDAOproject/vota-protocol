use std::fs;
use anchor_lang::AnchorDeserialize;
use solana_client::rpc_client::RpcClient;
use chrono::Utc;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_program::pubkey::Pubkey;
use gauge_state::EpochGauge;
use locked_voter_state::Escrow;
use crate::actions::queries::vote_buys::get_all_vote_buys;
use crate::ANCHOR_DISCRIMINATOR_SIZE;
use crate::management::data::{EpochStats, EpochVoteInfo, GaugeStats, GaugeVoteInfo, VoteInfo};

pub(crate) fn calculate_inputs(client: &RpcClient,config: &Pubkey, epoch: u32) {
    println!("calculate_inputs");


    // Find direct votes


    let accounts = client
        .get_program_accounts_with_config(
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
        )
        .unwrap();
    println!("account len: {:?}", accounts.len());
    let mut total_power: u64 = 0;
    let mut gauges: Vec<GaugeVoteInfo> = Vec::new();
    for (addr, account) in accounts {
        let epoch_guage = EpochGauge::deserialize(& mut &account.data[8..]);
        match epoch_guage {
            Ok(epoch_guage) => {
                println!("epoch_guage: {:?}", epoch_guage);
                total_power += epoch_guage.total_power;
                gauges.push(GaugeVoteInfo {
                    gauge: addr,
                    info: VoteInfo {
                        buys: 0,
                        delegated_votes: 0,
                        direct_votes: epoch_guage.total_power,
                    }
                });
            }
            Err(e) => {
                println!("error deserailzing: {:?}", addr);
            }
        }
    }

    //Get the vote buy accounts
    let vote_buys = get_all_vote_buys(epoch, config);
    println!("vote_buys: {:?}", vote_buys);

    //Create an epoch guage if one doesn't already exist for any of the vote buys

    // Find delegated votes and get totals for gauges that have already voted.

    //To get algorithmic votes subtract votes that are already used from the total of all epoch gauges

    println!("total_power: {:?}", total_power);

    // epoch stats

    let epoch_votes = EpochVoteInfo {
        epoch,
        totals: VoteInfo {
            buys: 100,
            delegated_votes: 100,
            direct_votes: total_power,
        },
        gauges,
    };
    let epoch_stats_json = serde_json::to_string(&epoch_votes).unwrap();
    fs::write(format!("./epoch_{}_vote_info{}.json", epoch, Utc::now().format("%Y-%m-%d-%H_%M" )), epoch_stats_json);
}