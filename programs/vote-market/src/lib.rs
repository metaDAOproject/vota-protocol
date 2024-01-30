pub mod errors;
pub mod state;
pub mod util;

use crate::state::{AllowedMints, VoteBuy, VoteMarketConfig};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gauge_state::GaugeProgram;
use locked_voter_state::LockedVoterProgram;

declare_id!("CgpagJ94phFKHBKkk4pd4YdKgfNCp5SzsiNwcLe73dc");

#[program]
pub mod vote_market {
    use super::*;
    use crate::util::vote_math::get_user_payment;
    use anchor_lang::solana_program;
    use anchor_lang::solana_program::program::{invoke, invoke_signed};
    use anchor_lang::solana_program::system_instruction;
    use anchor_spl::token::spl_token;
    use std::cmp::min;

    pub fn create_config(
        ctx: Context<CreateConfig>,
        mints: Vec<Pubkey>,
        efficiency_ratio: u64,
        script_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.script_authority = script_authority;
        ctx.accounts.config.gaugemeister = ctx.accounts.gaugemeister.key();
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
        //Check buyer and mint
        if ctx.accounts.buyer.key() == Pubkey::default() {
            return err!(errors::VoteMarketError::InvalidBuyer);
        }
        if ctx.accounts.mint.key() == Pubkey::default() {
            return err!(errors::VoteMarketError::InvalidMint);
        }
        if ctx.accounts.vote_buy.reward_receiver == Pubkey::default()
            && ctx.accounts.vote_buy.mint == Pubkey::default()
        {
            ctx.accounts.vote_buy.reward_receiver = ctx.accounts.buyer.key();
            ctx.accounts.vote_buy.mint = ctx.accounts.mint.key();
        }
        if ctx.accounts.vote_buy.reward_receiver != ctx.accounts.buyer.key() {
            return err!(errors::VoteMarketError::InvalidBuyer);
        }
        if ctx.accounts.vote_buy.mint != ctx.accounts.mint.key() {
            return err!(errors::VoteMarketError::InvalidMint);
        }
        // Check epoch
        if ctx.accounts.gaugemeister.current_rewards_epoch + 1 > epoch {
            return err!(errors::VoteMarketError::CompletedEpoch);
        }
        // Check if mint is valid
        ctx.accounts
            .allowed_mints
            .mints
            .iter()
            .find(|mint| mint == &&ctx.accounts.mint.key())
            .ok_or::<Error>(errors::VoteMarketError::InvalidMint.into())?;
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
        ctx.accounts.vote_buy.amount += amount;
        Ok(())
    }

    pub fn claim_vote_payment(ctx: Context<ClaimVotePayment>, epoch: u32) -> Result<()> {
        if epoch > ctx.accounts.gaugemeister.current_rewards_epoch {
            return err!(errors::VoteMarketError::EpochVotingNotCompleted);
        }
        let total_power = ctx.accounts.epoch_gauge.total_power;
        let allocated_power = ctx.accounts.epoch_gauge_vote.allocated_power;

        let vote_buy = &ctx.accounts.vote_buy;
        let total_vote_payment = match vote_buy.max_amount {
            Some(max_amount) => {
                min(max_amount, vote_buy.amount)
            }
            None => {
                return err!(errors::VoteMarketError::MaxVoteBuyAmountNotSet);
            }
        };
        let payment_to_user = get_user_payment(total_power, total_vote_payment, allocated_power)?;
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.seller_token_account.key(),
            &vote_buy.key(),
            &[],
            payment_to_user,
        )?;
        let (_, bump) = Pubkey::find_program_address(
            &[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                ctx.accounts.config.key().as_ref(),
                ctx.accounts.gauge.key().as_ref(),
            ],
            ctx.program_id,
        );
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.seller_token_account.to_account_info(),
                ctx.accounts.vote_buy.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            &[&[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                ctx.accounts.config.key().as_ref(),
                ctx.accounts.gauge.key().as_ref(),
                &[bump],
            ]],
        )?;

        //Calculating the discriminator manually instead of including the crate
        //because the anchor_lang version of gauge is not compatible with this program.
        let mut data: Vec<u8> =
            solana_program::hash::hash(b"global:close_epoch_gauge_vote").to_bytes()[..8].to_vec();
        data.extend_from_slice(&epoch.to_le_bytes());
        let (_, vote_delegate_bump) = Pubkey::find_program_address(
            &[
                b"vote-delegate".as_ref(),
                ctx.accounts.config.key().as_ref(),
            ],
            ctx.program_id,
        );
        let close_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.gauge_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.epoch_gauge_vote.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gaugemeister.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge_voter.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge_vote.key(), false),
                AccountMeta::new_readonly(ctx.accounts.escrow.key(), false),
                AccountMeta::new_readonly(ctx.accounts.vote_delegate.key(), true),
                AccountMeta::new(ctx.accounts.vote_delegate.key(), false),
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
            &[&[
                b"vote-delegate".as_ref(),
                ctx.accounts.config.key().as_ref(),
                &[vote_delegate_bump],
            ]],
        )?;
        Ok(())
    }

    pub fn claim_rewards_as_vote_seller(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn claim_rewards_as_vote_buyer(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, weight: u32) -> Result<()> {
        let mut data: Vec<u8> =
            solana_program::hash::hash(b"global:gauge_set_vote").to_bytes()[..8].to_vec();
        data.extend_from_slice(weight.to_le_bytes().as_ref());
        let set_weight_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.gaugemeister.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge.key(), false),
                AccountMeta::new(ctx.accounts.gauge_voter.key(), false),
                AccountMeta::new(ctx.accounts.gauge_vote.key(), false),
                AccountMeta::new_readonly(ctx.accounts.escrow.key(), false),
                AccountMeta::new(ctx.accounts.vote_delegate.key(), true),
            ],
            data,
        };
        let (expected_vote_delegate, bump) = Pubkey::find_program_address(
            &[
                b"vote-delegate".as_ref(),
                ctx.accounts.config.key().as_ref(),
            ],
            ctx.program_id,
        );
        if expected_vote_delegate != ctx.accounts.vote_delegate.key() {
            return Err(ProgramError::InvalidSeeds.into());
        }
        invoke_signed(
            &set_weight_ix,
            &[
                ctx.accounts.gaugemeister.to_account_info(),
                ctx.accounts.gauge.to_account_info(),
                ctx.accounts.gauge_voter.to_account_info(),
                ctx.accounts.gauge_vote.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
            ],
            &[&[
                b"vote-delegate".as_ref(),
                ctx.accounts.config.key().as_ref(),
                &[bump],
            ]],
        )?;
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn set_max_amount(ctx: Context<SetMaxAmount>, epoch: u32, max_amount: u64) -> Result<()> {
        ctx.accounts.vote_buy.max_amount = Some(max_amount);
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
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
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
    associated_token::authority = vote_buy
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister)]
    pub config: Account<'info, VoteMarketConfig>,
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(init_if_needed,
    payer = buyer,
    space = VoteBuy::LEN,
    seeds = [b"vote-buy".as_ref(),
    epoch.to_le_bytes().as_ref(),
    config.key().as_ref(),
    gauge.key().as_ref()],
    bump)]
    pub vote_buy: Account<'info, VoteBuy>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
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
    pub script_authority: Signer<'info>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = vote_buy,
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister, has_one = script_authority)]
    pub config: Box<Account<'info, VoteMarketConfig>>,
    #[account(mut,
    seeds = [b"vote-buy".as_ref(),
    epoch.to_le_bytes().as_ref(),
    config.key().as_ref(),
    gauge.key().as_ref()], bump)]
    pub vote_buy: Box<Account<'info, VoteBuy>>,
    #[account(mut, seeds = [b"vote-delegate", config.key().as_ref()], bump)]
    pub vote_delegate: SystemAccount<'info>,
    #[account(has_one = vote_delegate,
    constraint = escrow.owner == seller.key(),
    owner = locked_voter_program.key(),
    seeds = [b"Escrow",
        gaugemeister.locker.as_ref(),
        escrow.owner.as_ref()],
    bump,
    seeds::program = locked_voter_state::id())]
    pub escrow: Account<'info, locked_voter_state::Escrow>,
    #[account(owner = gauge_program.key(),
    constraint = gaugemeister.locker == escrow.locker)]
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(has_one = gaugemeister,
    has_one = escrow,
    seeds=[b"GaugeVoter",
    gaugemeister.key().as_ref(),
    escrow.key().as_ref()], bump,
    seeds::program = gauge_program.key(),
    )]
    pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    #[account(has_one = gauge_voter,
    has_one = gauge,
    seeds=[b"GaugeVote",
    gauge_voter.key().as_ref(),
    gauge.key().as_ref()],
    bump,
    seeds::program = gauge_program.key()
    )]
    pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    #[account(has_one = gauge_voter, owner = gauge_program.key(),
    seeds=[b"EpochGaugeVoter",
    gauge_voter.key().as_ref(),
    epoch.to_le_bytes().as_ref()],
    bump,
    seeds::program = gauge_program.key(),
    )]
    pub epoch_gauge_voter: Account<'info, gauge_state::EpochGaugeVoter>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(has_one = gauge, owner = gauge_program.key(),
    // seeds=[b"EpochGauge",
    //     gauge.key().as_ref(),
    //     epoch.to_le_bytes().as_ref()],
    // bump,
    // seeds::program = gauge_program.key(),
    )]
    pub epoch_gauge: Account<'info, gauge_state::EpochGauge>,
    #[account(mut, owner = gauge_program.key())]
    pub epoch_gauge_vote: Account<'info, gauge_state::EpochGaugeVote>,
    pub gauge_program: Program<'info, GaugeProgram>,
    pub locked_voter_program: Program<'info, LockedVoterProgram>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(weight: u32)]
pub struct Vote<'info> {
    #[account(has_one = gaugemeister, has_one = script_authority)]
    pub config: Account<'info, VoteMarketConfig>,
    pub script_authority: Signer<'info>,
    #[account(owner = gauge_program.key())]
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(mut,
    seeds=[b"GaugeVoter",
    gaugemeister.key().as_ref(),
    escrow.key().as_ref()], bump,
    seeds::program = gauge_program.key(),
    )]
    pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    #[account(mut,
    seeds=[b"GaugeVote",
    gauge_voter.key().as_ref(),
    gauge.key().as_ref()],
    bump,
    seeds::program = gauge_program.key(),
    )]
    pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    #[account(has_one = vote_delegate,
    seeds = [b"Escrow",
    gaugemeister.locker.as_ref(),
    escrow.owner.as_ref()],
    bump,
    seeds::program = locked_voter_state::id())]
    pub escrow: Account<'info, locked_voter_state::Escrow>,
    #[account(mut, seeds =
    [b"vote-delegate", config.key().as_ref()],
    bump)]
    pub vote_delegate: SystemAccount<'info>,
    pub gauge_program: Program<'info, GaugeProgram>,
}

#[derive(Accounts)]
#[instruction(epoch: u32)]
pub struct SetMaxAmount<'info> {
    pub config: Account<'info, VoteMarketConfig>,
    // Need to verify seeds to make sure the correct script_authority is used
    #[account(mut,
    seeds = [
        b"vote-buy".as_ref(),
        epoch.to_le_bytes().as_ref(),
        config.key().as_ref(),
        gauge.key().as_ref()], bump)]
    pub vote_buy: Account<'info, VoteBuy>,
    #[account(
    constraint = config.gaugemeister == gauge.gaugemeister,
    constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(address = config.script_authority)]
    pub script_authority: Signer<'info>,
}
