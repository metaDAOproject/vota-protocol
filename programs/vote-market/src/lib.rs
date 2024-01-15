mod errors;
mod state;
mod util;


use crate::state::{AllowedMints, TokenBuy, VoteMarketConfig};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use sha2::{Digest, Sha256};
use gauge_state::GaugeProgram;

declare_id!("CgpagJ94phFKHBKkk4pd4YdKgfNCp5SzsiNwcLe73dc");

#[program]
pub mod vote_market {
    use super::*;
    use anchor_lang::solana_program::program::{invoke, invoke_signed};
    use anchor_lang::solana_program::system_instruction;
    use anchor_spl::token::spl_token;
    use crate::util::math::calculate_voter_share;

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
            ctx.accounts.config.script_authority = ctx.accounts.payer.key();
        }
        ctx.accounts.config.gaugemeister = gaugemeister;
        ctx.accounts.allowed_mints.mints = mints;
        ctx.accounts.config.admin = ctx.accounts.payer.key();
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

    pub fn increase_vote_buy(ctx: Context<IncreaseVoteBuy>, epoch: u32, amount: u64) -> Result<()> {
        //Check buyer
        if ctx.accounts.token_buy.reward_receiver == Pubkey::default() {
            ctx.accounts.token_buy.reward_receiver = ctx.accounts.buyer.key();
        }
        if ctx.accounts.token_buy.reward_receiver != ctx.accounts.buyer.key() {
            return Err(errors::ErrorCode::InvalidBuyer.into());
        }
        // Check epoch
        let gaugemeister_info = ctx.accounts.gaugemeister.to_account_info();
        let mut gaugemeister = &gaugemeister_info.data.borrow_mut()[..];
        let gaugemeister_data: gauge_state::Gaugemeister =
            gauge_state::Gaugemeister::try_deserialize(&mut gaugemeister)?;
        if gaugemeister_data.current_rewards_epoch.checked_add(1)
            .ok_or::<Error>(errors::ErrorCode::EpochOverflow.into()).unwrap() > epoch {
            return Err(errors::ErrorCode::CompletedEpoch.into());
        }
        // Check if mint is valid
        ctx.accounts.allowed_mints.mints.iter().find(|mint| mint == &&ctx.accounts.mint.key())
            .ok_or::<Error>(errors::ErrorCode::InvalidMint.into())?;
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.buyer_token_account.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.buyer.key(),
            &[],
            amount,
        )?;
        invoke(
            &transfer_ix,
            &[
                ctx.accounts.buyer_token_account.to_account_info(),
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
        )?;
        ctx.accounts.token_buy.amount += amount;
        ctx.accounts.token_buy.mint = ctx.accounts.mint.key();
        ctx.accounts.token_buy.percent_to_use_bps = 0;
        Ok(())
    }

    pub fn claim_vote_payment(ctx: Context<ClaimVotePayment>, epoch: u32) -> Result<()> {
        if epoch > ctx.accounts.gaugemeister.current_rewards_epoch  {
            return Err(errors::ErrorCode::EpochVotingNotCompleted.into());
        }
        let total_power = ctx.accounts.epoch_gauge.total_power;
        let allocated_power = ctx.accounts.epoch_gauge_vote.allocated_power;
        let total_vote_payment = ctx.accounts.token_buy.amount;
        let voter_share = calculate_voter_share(total_power, total_vote_payment, allocated_power)?;
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.seller_token_account.key(),
            &ctx.accounts.token_buy.key(),
            &[],
            voter_share,
        )?;
        let (expected_token_buy, bump) = Pubkey::find_program_address(
            &[b"token-buy".as_ref(), epoch.to_le_bytes().as_ref(), ctx.accounts.config.key().as_ref(), ctx.accounts.gauge.key().as_ref()],
            ctx.program_id,
        );
        if expected_token_buy != ctx.accounts.token_buy.key() {
            return Err(ProgramError::InvalidSeeds.into());
        }
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.seller_token_account.to_account_info(),
                ctx.accounts.token_buy.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            &[&[b"token-buy".as_ref(), epoch.to_le_bytes().as_ref(), ctx.accounts.config.key().as_ref(), ctx.accounts.gauge.key().as_ref(), &[bump]]],
        )?;

        //Calculating the discriminator manually instead of including the crate
        //because the anchor_lang version of gauge is not compatible with this program.
        let mut hasher = Sha256::new();
        hasher.update(b"global:close_epoch_gauge_vote");
        let mut data: Vec<u8> = hasher.finalize()[..8].into();
        data.extend_from_slice(&epoch.to_le_bytes());
        let (expected_vote_delegate, vote_delegate_bump) = Pubkey::find_program_address(
            &[b"vote-delegate".as_ref(), ctx.accounts.config.key().as_ref()],
            ctx.program_id,
        );
        if expected_vote_delegate != ctx.accounts.vote_delegate.key() {
            return Err(ProgramError::InvalidSeeds.into());
        }
        let close_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.gauge_program.key(),
            accounts: vec![
                AccountMeta {
                    pubkey: ctx.accounts.epoch_gauge_vote.key(),
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: ctx.accounts.gaugemeister.key(),
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: ctx.accounts.gauge.key(),
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: ctx.accounts.gauge_voter.key(),
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: ctx.accounts.gauge_vote.key(),
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: ctx.accounts.escrow.key(),
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: ctx.accounts.vote_delegate.key(),
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: ctx.accounts.vote_delegate.key(),
                    is_signer: false,
                    is_writable: true,
                },
            ],
            data,
        };
        invoke_signed(
            &close_ix,
            &[
                ctx.accounts.epoch_gauge_vote.to_account_info(),
                ctx.accounts.gaugemeister.to_account_info(),
                ctx.accounts.gauge.to_account_info(),
                ctx.accounts.gauge_voter.to_account_info(),
                ctx.accounts.gauge_vote.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
            ],
            &[&[b"vote-delegate".as_ref(), ctx.accounts.config.key().as_ref(), &[vote_delegate_bump]]],
        )?;
        Ok(())
    }

    pub fn claim_rewards_as_vote_seller(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn claim_rewards_as_vote_buyer(ctx: Context<Initialize>) -> Result<()> {
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
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(init_if_needed,
    payer = buyer,
    associated_token::mint = mint,
    associated_token::authority = token_buy
    )]
    pub token_vault: Account<'info, TokenAccount>,
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
    #[account(seeds = [b"allow-list".as_ref(), config.key().as_ref()], bump)]
    pub allowed_mints: Account<'info, AllowedMints>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(epoch: u32)]
pub struct ClaimVotePayment<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = token_buy,
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister)]
    pub config: Account<'info, VoteMarketConfig>,
    #[account(mut,
    seeds = [b"token-buy".as_ref(), epoch.to_le_bytes().as_ref(), config.key().as_ref(), gauge.key().as_ref()], bump)]
    pub token_buy: Account<'info, TokenBuy>,
    #[account(mut, seeds = [b"vote-delegate", config.key().as_ref()], bump)]
    pub vote_delegate: SystemAccount<'info>,
    #[account(has_one = vote_delegate)]
    pub escrow: Account<'info, locked_voter_state::Escrow>,
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(has_one = gaugemeister, has_one = escrow)]
    pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    #[account(has_one = gauge_voter, has_one = gauge)]
    pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    #[account(has_one = gauge_voter)]
    pub epoch_gauge_voter: Account<'info, gauge_state::EpochGaugeVoter>,
    #[account(has_one = gaugemeister)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    pub epoch_gauge: Account<'info, gauge_state::EpochGauge>,
    #[account(mut)]
    pub epoch_gauge_vote: Account<'info, gauge_state::EpochGaugeVote>,
    pub gauge_program: Program<'info, GaugeProgram>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}