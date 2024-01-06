use anchor_lang::prelude::*;

#[account]
pub struct TokenBuy {
    mint: Pubkey,
    amount: u64,
    percent_to_use_bps: u64,
    reward_receiver: Pubkey,
}

impl TokenBuy {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 32;
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



