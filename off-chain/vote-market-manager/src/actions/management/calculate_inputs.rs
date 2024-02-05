use crate::actions::management::data::{EpochInput, GaugeInfo};
use crate::actions::management::oracle::{fetch_token_prices, KnownTokens};
use crate::actions::queries::vote_buys::get_all_vote_buys;
use crate::{ANCHOR_DISCRIMINATOR_SIZE, GAUGEMEISTER, LOCKER};
use anchor_lang::AnchorDeserialize;
use chrono::Utc;
use gauge_state::{EpochGauge, Gaugemeister};
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType::DataSize;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;
use std::fs;
use locked_voter_state::Locker;
use crate::accounts::resolve::{get_delegate, get_epoch_gauge_voter, get_gauge_voter};

pub(crate) fn calculate_inputs(
    client: &RpcClient,
    config: &Pubkey,
    epoch: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("calculate_inputs");

    //Get the vote buy accounts
    let vote_buys = get_all_vote_buys(epoch, config);
    println!("vote_buys: {:?}", vote_buys);

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
    let mut gauges: Vec<GaugeInfo> = Vec::new();
    for (addr, account) in accounts {
        let epoch_guage = EpochGauge::deserialize(&mut &account.data[8..])?;
        println!("epoch_guage: {:?}", epoch_guage);
        total_power += epoch_guage.total_power;
        //let vote_buy = vote_buys.iter().find(|x| x.== addr);
        gauges.push(GaugeInfo {
            gauge: addr,
            payment: 100.0,
            votes: epoch_guage.total_power,
        });
    }


    //Create an epoch gauge if one doesn't already exist for any of the vote buys

    // Find delegated votes and get totals for gauges that have already voted.

    let delegate = get_delegate(config);
    let delegated_voters = crate::escrows::get_delegated_escrows(client, &delegate);
    // check if voters are prepped, and prep if not.
    let locker_account = client.get_account(&LOCKER).unwrap();
    let locker_data = Locker::deserialize(&mut &locker_account.data[8..]).unwrap();
    let gaugemeister_account = client.get_account(&GAUGEMEISTER).unwrap();
    let gaugemeister_data = Gaugemeister::deserialize(&mut &gaugemeister_account.data[8..]).unwrap();
    let mut already_voted_count = 0;
    for (key, escrow) in &delegated_voters {
        // check if escrow is already voting
        let gauge_voter_address = get_gauge_voter(&key);
        let epoch_gauge_voter_address = get_epoch_gauge_voter(&gauge_voter_address, epoch);
        match client.get_account(&epoch_gauge_voter_address) {
            Err(_) => {
                println!("escrow: {:?}",escrow.voting_power_at_time(&locker_data.params, gaugemeister_data.next_epoch_starts_at as i64).unwrap());
            },
            Ok(_) => {
                already_voted_count += 1;
            }
        }


        // if not, prep it
    }
    println!("{:?} escrows already voted", already_voted_count);

    //To get algorithmic votes subtract votes that are already used from the total of all epoch gauges

    println!("total_power: {:?}", total_power);
    // get tokens used in vote buys
    let mut tokens: Vec<KnownTokens> = vote_buys.iter().map(|x| x.mint.into()).collect();

    // Add SBR price
    tokens.push(KnownTokens::SBR);

    // Get USD values of relevant tokens
    let mut prices: HashMap<KnownTokens, f64> = HashMap::new();
    fetch_token_prices(&mut prices, tokens)?;

    // epoch stats

    let epoch_votes = EpochInput {
        epoch,
        direct_votes: total_power,
        delegated_votes: 100,
        gauges,
        prices,
        escrows: delegated_voters.iter().map(|x| x.0).collect(),
    };
    let epoch_stats_json = serde_json::to_string(&epoch_votes).unwrap();
    fs::write(
        format!(
            "./epoch_{}_vote_info{}.json",
            epoch,
            Utc::now().format("%Y-%m-%d-%H_%M")
        ),
        epoch_stats_json,
    )?;
    Ok(())
}