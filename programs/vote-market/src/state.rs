use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct VoteBuy {
    pub gauge: Pubkey,
    /// The mint of the token used to buy votes. This must be included in
    /// [AllowedMints::mints] for the gauge.
    pub mint: Pubkey,
    pub amount: u64,
    /// This amount or the amount field, whichever is less, will be distributed
    pub max_amount: Option<u64>,
    /// Anyone can create a vault for buying tokens. This is the creator
    /// of the vault who is will be entitled to any incentives and is
    /// allowed to add additional funds.
    pub reward_receiver: Pubkey,
}

impl VoteBuy {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 8 + 32;
}

#[account]
pub struct VoteMarketConfig {
    pub script_authority: Pubkey,
    pub gaugemeister: Pubkey,
    pub admin: Pubkey,
    pub efficiency_ratio: u64,
}

impl VoteMarketConfig {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8;
}

#[account]
pub struct AllowedMints {
    pub mints: Vec<Pubkey>,
}

impl AllowedMints {
    pub fn len(mints: usize) -> usize {
        8 + 4 + 32 * mints
    }
}
