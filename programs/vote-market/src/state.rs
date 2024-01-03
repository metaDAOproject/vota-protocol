use anchor_lang::prelude::*;
#[account]
pub struct GaugeBuy {
    token_buys: Vec<TokenBuy>,
    buyer: Pubkey,
}

impl GaugeBuy {
    pub fn len(&self) -> usize {
        8 + 32 + self.token_buys.len()
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone)]
pub struct TokenBuy {
    mint: Pubkey,
    amount: u64,
    percent_to_use_bps: u64,
}

impl TokenBuy {
    pub const LEN: usize = 8 + 32 + 8 + 8;
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

