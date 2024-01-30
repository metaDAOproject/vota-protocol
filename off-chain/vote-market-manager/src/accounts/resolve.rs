use crate::{GAUGEMEISTER, LOCKER};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

pub fn get_escrow_address_for_owner(owner: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"Escrow".as_ref(), LOCKER.as_ref(), owner.as_ref()],
        &locked_voter_state::id(),
    )
    .0
}

pub struct VoteKeys {
    pub gauge_voter: Pubkey,
    pub gauge_vote: Pubkey,
    pub epoch_gauge_voter: Pubkey,
    pub epoch_gauge_vote: Pubkey,
    pub epoch_gauge: Pubkey,
}

pub enum VoteCreateStep {
    GaugeVoter(Pubkey),
    GaugeVote(Pubkey),
    EpochGaugeVoter(Pubkey),
}

impl VoteKeys {
    pub fn get_all_keys(&self) -> Vec<Pubkey> {
        vec![
            self.gauge_voter,
            self.gauge_vote,
            self.epoch_gauge_voter,
            self.epoch_gauge_vote,
            self.epoch_gauge,
        ]
    }
    pub fn get_missing_prepare_vote_accounts(&self, client: &RpcClient) -> Vec<VoteCreateStep> {
        let accounts = client.get_multiple_accounts(&self.get_all_keys()).unwrap();
        let mut steps: Vec<VoteCreateStep> = Vec::new();
        for (index, account) in accounts.iter().enumerate() {
            if account.is_none() {
                // We don't need to match epoch gauge vote, because we can't set that until we
                // have set the vote amount
                match index {
                    0 => steps.push(VoteCreateStep::GaugeVoter(self.gauge_voter)),
                    1 => steps.push(VoteCreateStep::GaugeVote(self.gauge_vote)),
                    2 => steps.push(VoteCreateStep::EpochGaugeVoter(self.epoch_gauge_voter)),
                    _ => {}
                }
            }
        }
        steps
    }
}

pub fn resolve_vote_keys(escrow: &Pubkey, gauge: &Pubkey, epoch: u32) -> VoteKeys {
    let gauge_voter = get_gauge_voter(escrow);
    let gauge_vote = get_gauge_vote(&gauge_voter, gauge);
    let epoch_gauge_voter = get_epoch_gauge_voter(&gauge_voter, epoch);
    let epoch_gauge_vote = get_epoch_gauge_vote(&gauge_vote, epoch);
    let epoch_gauge = get_epoch_gauge(gauge, epoch);
    VoteKeys {
        gauge_voter,
        gauge_vote,
        epoch_gauge_voter,
        epoch_gauge_vote,
        epoch_gauge,
    }
}

fn get_epoch_gauge(gauge: &Pubkey, epoch: u32) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"EpochGauge".as_ref(),
            gauge.as_ref(),
            epoch.to_le_bytes().as_ref(),
        ],
        &gauge_state::id(),
    )
    .0
}

fn get_gauge_voter(escrow: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"GaugeVoter".as_ref(),
            GAUGEMEISTER.as_ref(),
            escrow.as_ref(),
        ],
        &gauge_state::id(),
    )
    .0
}

fn get_gauge_vote(gauge_voter: &Pubkey, gauge: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"GaugeVote".as_ref(), gauge_voter.as_ref(), gauge.as_ref()],
        &gauge_state::id(),
    )
    .0
}

fn get_epoch_gauge_vote(gauge_vote: &Pubkey, epoch: u32) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"EpochGaugeVote".as_ref(),
            gauge_vote.as_ref(),
            epoch.to_le_bytes().as_ref(),
        ],
        &gauge_state::id(),
    )
    .0
}

fn get_epoch_gauge_voter(gauge_voter: &Pubkey, epoch: u32) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"EpochGaugeVoter".as_ref(),
            gauge_voter.as_ref(),
            epoch.to_le_bytes().as_ref(),
        ],
        &gauge_state::id(),
    )
    .0
}

pub fn get_delegate(config: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"vote-delegate".as_ref(), config.as_ref()],
        &vote_market::id(),
    )
    .0
}

pub fn get_vote_buy(config: &Pubkey, gauge: &Pubkey, epoch: u32) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"vote-buy".as_ref(),
            epoch.to_le_bytes().as_ref(),
            config.as_ref(),
            gauge.as_ref(),
        ],
        &vote_market::id(),
    )
    .0
}
