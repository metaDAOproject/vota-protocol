use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use crate::accounts::resolve::{get_escrow_address_for_owner, resolve_vote_keys, VoteCreateStep};

pub fn prepare_vote(client: &RpcClient, owner: Pubkey, gauge: Pubkey, epoch: u32) {
    let escrow_address = get_escrow_address_for_owner(&owner);
    println!("Prepare vote for escrow: {:?}", escrow_address);
    let vote_keys = resolve_vote_keys(escrow_address, gauge, epoch);
    let steps = vote_keys.get_missing_accounts(client);
    for step in steps {
        match step {
            VoteCreateStep::GaugeVote(key) => {
                println!("Creating gauge vote {}", key);
            },
            VoteCreateStep::GaugeVoter(key) => {
                println!("Creating gauge voter {}", key);
            },
            VoteCreateStep::EpochGaugeVote(key) => {
                println!("Creating epoch gauge vote {}", key);
            },
            VoteCreateStep::EpochGaugeVoter(key) => {
                println!("Creating epoch gauge voter {}", key);
            },
        }
    }
}