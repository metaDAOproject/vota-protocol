mod state;

use crate::state::{AllowedMints, TokenBuy, VoteMarketConfig};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("CgpagJ94phFKHBKkk4pd4YdKgfNCp5SzsiNwcLe73dc");

#[program]
pub mod vote_market {
    use super::*;
    use anchor_lang::solana_program::program::invoke;
    use anchor_lang::solana_program::system_instruction;

    pub fn create_config(
        ctx: Context<CreateConfig>,
        mints: Vec<Pubkey>,
        gaugemeister: Pubkey,
        efficiency_ratio: u64,
        script_authority: Option<Pubkey>,
    ) -> Result<()> {
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

    pub fn update_script_authority(
        ctx: Context<UpdateScriptAuthority>,
        script_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.script_authority = script_authority;
        Ok(())
    }

    pub fn update_allowed_mints(
        ctx: Context<UpdateAllowedMints>,
        allowed_mints: Vec<Pubkey>,
    ) -> Result<()> {
        let allowed_mints_size = AllowedMints::len(ctx.accounts.allowed_mints.mints.len());
        let next_allowed_mints_size = AllowedMints::len(allowed_mints.len());
        if next_allowed_mints_size > allowed_mints_size {
            let allowed_mints_account_info = ctx.accounts.allowed_mints.to_account_info();
            allowed_mints_account_info.realloc(next_allowed_mints_size, false)?;
            let rent = Rent::get()?;
            let next_rent_exemption = rent.minimum_balance(next_allowed_mints_size);
            if allowed_mints_account_info.lamports() < next_rent_exemption {
                let required_lamports = next_rent_exemption - allowed_mints_account_info.lamports();
                let transfer_rent = system_instruction::transfer(
                    ctx.accounts.admin.key,
                    &ctx.accounts.allowed_mints.key(),
                    required_lamports,
                );
                invoke(
                    &transfer_rent,
                    &[
                        ctx.accounts.admin.to_account_info(),
                        ctx.accounts.allowed_mints.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;
            }
        }

        ctx.accounts.allowed_mints.mints = allowed_mints;
        Ok(())
    }

    pub fn increase_vote_buy(ctx: Context<Initialize>) -> Result<()> {
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
    #[account(mut, has_one = admin)]
    pub config: Account<'info, VoteMarketConfig>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, VoteMarketConfig>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAllowedMints<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, VoteMarketConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"allow-list".as_ref(), config.to_account_info().key.as_ref()],
        bump)]
    pub allowed_mints: Account<'info, AllowedMints>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(epoch: u32)]
pub struct IncreaseVoteBuy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister)]
    pub config: Account<'info, VoteMarketConfig>,
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(init_if_needed,
    payer = buyer,
    space = TokenBuy::LEN,
    seeds = [b"token-buy".as_ref(), epoch.to_le_bytes().as_ref(), config.key().as_ref(), gauge.key().as_ref()],
    bump)]
    pub token_buy: Account<'info, TokenBuy>,
    pub gauge: Account<'info, gauge_state::Gauge>,
    pub system_program: Program<'info, System>,
}
