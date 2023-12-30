mod state;

use anchor_lang::prelude::*;

declare_id!("CgpagJ94phFKHBKkk4pd4YdKgfNCp5SzsiNwcLe73dc");

#[program]
pub mod vote_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_config(ctx: Context<Initialize>, allowed_mints: Vec<Pubkey>) -> Result<()> {
        Ok(())
    }

    pub fn update_allowed_mints(ctx: Context<Initialize>, allowed_mints: Vec<Pubkey>) -> Result<()> {
        Ok(())
    }

    pub fn buy_votes(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_seller_vote_payment(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_seller_rewards(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_buyer_rewards(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn vote(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
