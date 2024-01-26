use anchor_lang::prelude::*;
declare_id!("LocktDzaV1W2Bm9DeZeiyz4J9zs4fRqNiYqQyracRXw");

#[derive(Clone)]
pub struct LockedVoterProgram;

impl Id for LockedVoterProgram {
    fn id() -> Pubkey {
        ID
    }
}
const PUBKEY_BYTES: usize = 32;
/// Locks tokens on behalf of a user.
#[account]
#[derive(Copy, Debug, Default)]
pub struct Escrow {
    /// The [Locker] that this [Escrow] is part of.
    pub locker: Pubkey,
    /// The key of the account that is authorized to stake into/withdraw from this [Escrow].
    pub owner: Pubkey,
    /// Bump seed.
    pub bump: u8,

    /// The token account holding the escrow tokens.
    pub tokens: Pubkey,
    /// Amount of tokens staked.
    pub amount: u64,
    /// When the [Escrow::owner] started their escrow.
    pub escrow_started_at: i64,
    /// When the escrow unlocks; i.e. the [Escrow::owner] is scheduled to be allowed to withdraw their tokens.
    pub escrow_ends_at: i64,

    /// Account that is authorized to vote on behalf of this [Escrow].
    /// Defaults to the [Escrow::owner].
    pub vote_delegate: Pubkey,
}

impl Escrow {
    /// Number of bytes in an [Escrow].
    pub const LEN: usize = PUBKEY_BYTES * 2 + 1 + PUBKEY_BYTES + 8 + 8 + 8 + PUBKEY_BYTES;
}
/// A group of [Escrow]s.
#[account]
#[derive(Copy, Debug, Default)]
pub struct Locker {
    /// Base account used to generate signer seeds.
    pub base: Pubkey,
    /// Bump seed.
    pub bump: u8,
    /// Mint of the token that must be locked in the [Locker].
    pub token_mint: Pubkey,
    /// Total number of tokens locked in [Escrow]s.
    pub locked_supply: u64,
    /// Governor associated with the [Locker].
    pub governor: Pubkey,
    /// Mutable parameters of how a [Locker] should behave.
    pub params: LockerParams,
}

impl Locker {
    /// Number of bytes in a [Locker].
    pub const LEN: usize = PUBKEY_BYTES + 1 + PUBKEY_BYTES + 8 + PUBKEY_BYTES + LockerParams::LEN;
}

/// Contains parameters for the [Locker].
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LockerParams {
    /// Whether or not the locking whitelist system is enabled.
    pub whitelist_enabled: bool,
    /// The weight of a maximum vote lock relative to the total number of tokens locked.
    /// For example, veCRV is 10 because 1 CRV locked for 4 years = 10 veCRV.
    pub max_stake_vote_multiplier: u8,
    /// Minimum staking duration.
    pub min_stake_duration: u64,
    /// Maximum staking duration.
    pub max_stake_duration: u64,
    /// Minimum number of votes required to activate a proposal.
    pub proposal_activation_min_votes: u64,
}

impl LockerParams {
    /// Number of bytes in a [LockerParams].
    pub const LEN: usize = 1 + 1 + 8 + 8 + 8;
}
