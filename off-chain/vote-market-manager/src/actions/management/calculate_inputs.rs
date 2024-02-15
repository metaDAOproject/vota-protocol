use crate::actions::management::data::{EpochData, GaugeInfo};
use crate::actions::management::oracle::{fetch_token_prices, KnownTokens};
use crate::actions::queries::vote_buys::get_all_vote_buys;
use crate::{GAUGEMEISTER, LOCKER};
use anchor_lang::AnchorDeserialize;
use chrono::Utc;
use gauge_state::Gaugemeister;

use solana_client::rpc_client::RpcClient;

use crate::accounts::resolve::{get_delegate, get_epoch_gauge_voter, get_gauge_voter};
use crate::actions::queries::direct_votes::get_direct_votes;
use locked_voter_state::Locker;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use spl_token::state::Mint;
use std::collections::HashMap;
use std::fs;

/// Creates a json file containing all the data needed to calculate algorithmic
/// vote weights and the maximum amount of bribes that meet the efficiency
/// ratio requirements for one epoch and one [`vote_market::state::VoteMarketConfig`]
///
/// The json file will be named `epoch_{epoch}_vote_info{timestamp}.json`
/// and have the following format
/// ```
///{
///   "epoch": 98,
///   "direct_votes": 0,
///   "delegated_votes": 5970510976,
///   "total_vote_buy_value": 5717.58,
///   "gauges": [
///     {
///       "gauge": "3V7SVqXAMGzezRfe3LGhELZFNMCH2jVsu5TmT8CawK5y",
///       "payment": 5717.58,
///       "votes": 0
///     }
///   ],
///   "prices": {
///     "BLZE": 0.00285879,
///     "SBR": 0.00290758
///   },
///   "escrows": "[BEwbnYCmqQ8pi59s7E6uK26hMhy1GJivqsRaeWU4PHUW,DyBaLYzwbbWnPBAa23LyrAw2sHxYS2C2DDmq711yg5on,9uVg1hWhmn7qPaMT8pAeNV1yFFSwmFPTNyJ9xT5SaQgf]"
/// }
/// ```
pub(crate) fn calculate_inputs(
    client: &RpcClient,
    config: &Pubkey,
    epoch: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("calculate_inputs");

    //Get the vote buy accounts
    let vote_buys = get_all_vote_buys(epoch, config);
    println!("vote_buys: {:?}", vote_buys);

    // Get all of the relevant prices for calculating weights and efficiency
    // get tokens used in vote buys
    let mut tokens: Vec<KnownTokens> = vote_buys.iter().map(|x| x.mint.into()).collect();

    // Add SBR price
    tokens.push(KnownTokens::SBR);

    // Get USD values of relevant tokens
    let mut prices: HashMap<KnownTokens, f64> = HashMap::new();
    fetch_token_prices(&mut prices, tokens)?;

    // Find direct votes

    let direct_votes = get_direct_votes(client, epoch)?;

    println!("account len: {:?}", direct_votes.len());
    let mut total_power: u64 = 0;
    let mut gauges: Vec<GaugeInfo> = Vec::new();
    let mut total_vote_buy_value: f64 = 0.0;
    for epoch_gauge in direct_votes {
        // Only count gauges that have an associated vote buy
        if !vote_buys.iter().any(|x| x.gauge == epoch_gauge.gauge) {
            continue;
        }
        total_power += epoch_gauge.total_power;
        let mut payment = 0.0;
        if let Some(vote_buy) = vote_buys.iter().find(|x| x.gauge == epoch_gauge.gauge) {
            println!("found vote buy: {:?}", vote_buy);
            let mint_account = client.get_account(&vote_buy.mint).unwrap();
            let decimals = Mint::unpack(mint_account.data.as_slice())?.decimals;
            let amount = spl_token::amount_to_ui_amount(vote_buy.amount, decimals);
            let price = prices.get(&vote_buy.mint.into()).unwrap();
            payment = amount * price;
            total_vote_buy_value += payment;
        }

        gauges.push(GaugeInfo {
            gauge: epoch_gauge.gauge,
            payment,
            votes: epoch_gauge.total_power,
        });
    }

    // Find delegated votes and get totals for gauges that have already voted.

    let delegate = get_delegate(config);
    let delegated_voters = crate::escrows::get_delegated_escrows(client, &delegate);
    // check if voters are prepped, and prep if not.
    let locker_account = client.get_account(&LOCKER).unwrap();
    let locker_data = Locker::deserialize(&mut &locker_account.data[8..]).unwrap();
    let gaugemeister_account = client.get_account(&GAUGEMEISTER).unwrap();
    let gaugemeister_data =
        Gaugemeister::deserialize(&mut &gaugemeister_account.data[8..]).unwrap();
    let mut already_voted_count = 0;
    let mut total_delegated_votes: u64 = 0;
    for (key, escrow) in &delegated_voters {
        // check if escrow is already voting
        let gauge_voter_address = get_gauge_voter(key);
        let epoch_gauge_voter_address = get_epoch_gauge_voter(&gauge_voter_address, epoch);
        match client.get_account(&epoch_gauge_voter_address) {
            Err(_) => {
                let voting_power = escrow
                    .voting_power_at_time(
                        &locker_data.params,
                        gaugemeister_data.next_epoch_starts_at as i64,
                    )
                    .unwrap();
                total_delegated_votes += voting_power;
                println!(
                    "escrow: {:?}",
                    escrow
                        .voting_power_at_time(
                            &locker_data.params,
                            gaugemeister_data.next_epoch_starts_at as i64
                        )
                        .unwrap()
                );
            }
            Ok(_) => {
                already_voted_count += 1;
            }
        }

        // if not, prep it
    }
    println!("{:?} escrows already voted", already_voted_count);

    //To get algorithmic votes subtract votes that are already used from the total of all epoch gauges

    println!("total_power: {:?}", total_power);

    // epoch stats

    let epoch_votes = EpochData {
        config: *config,
        epoch,
        direct_votes: total_power,
        delegated_votes: total_delegated_votes,
        total_vote_buy_value,
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
