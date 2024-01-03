mod state;
use anchor_lang::prelude::*;
use crate::state::{AllowedMints, VoteMarketConfig};

declare_id!("CgpagJ94phFKHBKkk4pd4YdKgfNCp5SzsiNwcLe73dc");

#[program]
pub mod vote_market {
    use super::*;


    pub fn create_config(ctx: Context<CreateConfig>, mints: Vec<Pubkey>, gaugemeister: Pubkey, efficiency_ratio: u64, script_authority: Option<Pubkey> ) -> Result<()> {
        if let Some(script_authority) = script_authority {
            ctx.accounts.config.script_authority = script_authority;
        } else {
            ctx.accounts.config.script_authority = *ctx.accounts.payer.key;
        }
        ctx.accounts.config.gaugemeister = gaugemeister;
        ctx.accounts.allowed_mints.mints = mints;
        ctx.accounts.config.admin = *ctx.accounts.payer.key;
        ctx.accounts.config.efficiency_ratio = efficiency_ratio;
        Ok(())
    }

    pub fn update_admin(ctx: Context<UpdateAdmin>, admin: Pubkey) -> Result<()> {
        ctx.accounts.config.admin = admin;
        Ok(())
    }

    pub fn update_script_authority(ctx: Context<UpdateScriptAuthority>, script_authority: Pubkey) -> Result<()> {
        ctx.accounts.config.script_authority = script_authority;
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

#[derive(Accounts)]
#[instruction(mints: Vec<Pubkey>)]
pub struct CreateConfig<'info> {
    #[account(
        init,
        payer = payer,
        space = VoteMarketConfig::LEN,
        )]
    pub config: Account<'info, VoteMarketConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = AllowedMints::len(mints.len()),
        seeds = [b"allow-list".as_ref(), config.to_account_info().key.as_ref()],
        bump)]
    pub allowed_mints: Account<'info, AllowedMints>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateScriptAuthority<'info> {
    #[account(mut)]
    pub config: Account<'info, VoteMarketConfig>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub config: Account<'info, VoteMarketConfig>,
    pub admin: Signer<'info>,
}
