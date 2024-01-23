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
