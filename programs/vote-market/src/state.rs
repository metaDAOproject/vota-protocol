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
    script_authority: Pubkey,
    gaugemeister: Pubkey,
}

impl VoteMarketConfig {
    pub const LEN: usize = 8 + 32 + 32;
}

