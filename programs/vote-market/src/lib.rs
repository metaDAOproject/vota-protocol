pub mod errors;
pub mod state;
pub mod util;

use crate::state::{AllowedMints, VoteBuy, VoteMarketConfig};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gauge_state::GaugeProgram;
use locked_voter_state::LockedVoterProgram;

declare_id!("VotAjwzAEF9ZLNAYEB1ivXt51911EqYGVu9NeaEKRyy");

#[program]
pub mod vote_market {
    use super::*;
    use crate::util::vote_math::{get_fee, get_user_payment};
    use anchor_lang::solana_program;
    use anchor_lang::solana_program::program::{invoke, invoke_signed};
    use anchor_lang::solana_program::system_instruction;
    use anchor_spl::token::spl_token;
    use std::cmp::min;

    pub fn create_config(
        ctx: Context<CreateConfig>,
        mints: Vec<Pubkey>,
        claim_fee: u16,
        script_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.script_authority = script_authority;
        ctx.accounts.config.gaugemeister = ctx.accounts.gaugemeister.key();
        ctx.accounts.allowed_mints.mints = mints;
        ctx.accounts.config.admin = ctx.accounts.payer.key();
        ctx.accounts.config.claim_fee = claim_fee;
        Ok(())
    }

    pub fn update_admin(ctx: Context<UpdateAdmin>, admin: Pubkey) -> Result<()> {
        ctx.accounts.config.admin = admin;
        Ok(())
    }

    pub fn update_claim_fee(ctx: Context<UpdateAdmin>, claim_fee: u16) -> Result<()> {
        ctx.accounts.config.claim_fee = claim_fee;
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
        if ctx.accounts.vote_buy.buyer == Pubkey::default()
            && ctx.accounts.vote_buy.mint == Pubkey::default()
        {
            ctx.accounts.vote_buy.buyer = ctx.accounts.buyer.key();
            ctx.accounts.vote_buy.mint = ctx.accounts.mint.key();
        }
        if ctx.accounts.vote_buy.buyer != ctx.accounts.buyer.key() {
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
        ctx.accounts.vote_buy.gauge = ctx.accounts.gauge.key();
        Ok(())
    }

    pub fn claim_vote_payment(ctx: Context<ClaimVotePayment>, epoch: u32) -> Result<()> {
        //seed checks. Doing this on the Accounts struct uses too much stack space
        msg!("Claiming payment");
        let (expected_epoch_gauge, _) = Pubkey::find_program_address(
            &[
                b"EpochGauge".as_ref(),
                ctx.accounts.gauge.key().as_ref(),
                epoch.to_le_bytes().as_ref(),
            ],
            &gauge_state::id(),
        );
        require_keys_eq!(expected_epoch_gauge, ctx.accounts.epoch_gauge.key());
        let (expected_epoch_guage_vote, _) = Pubkey::find_program_address(
            &[
                b"EpochGaugeVote".as_ref(),
                ctx.accounts.gauge_vote.key().as_ref(),
                epoch.to_le_bytes().as_ref(),
            ],
            &gauge_state::id(),
        );
        require_keys_eq!(
            expected_epoch_guage_vote,
            ctx.accounts.epoch_gauge_vote.key()
        );
        if epoch > ctx.accounts.gaugemeister.current_rewards_epoch {
            return err!(errors::VoteMarketError::EpochVotingNotCompleted);
        }
        let total_power = ctx.accounts.epoch_gauge.total_power;
        let allocated_power = ctx.accounts.epoch_gauge_vote.allocated_power;

        let vote_buy = &ctx.accounts.vote_buy;
        let total_vote_payment = match vote_buy.max_amount {
            Some(max_amount) => min(max_amount, vote_buy.amount),
            None => {
                return err!(errors::VoteMarketError::MaxVoteBuyAmountNotSet);
            }
        };
        msg!("Total Power: {}", total_power);
        msg!("Allocated Power: {}", allocated_power);
        msg!("Total Vote Payment: {}", total_vote_payment);
        let total_payment = get_user_payment(total_power, allocated_power, total_vote_payment)?;
        let fee = get_fee(total_payment, ctx.accounts.config.claim_fee)?;
        let payment_to_user = total_payment - fee;
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
        let transfer_to_treasury_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.treasury.key(),
            &vote_buy.key(),
            &[],
            fee,
        )?;
        invoke_signed(
            &transfer_to_treasury_ix,
            &[
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
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
        require_keys_eq!(expected_vote_delegate, ctx.accounts.vote_delegate.key());
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

    pub fn commit_vote(ctx: Context<CommitVote>, epoch: u32) -> Result<()> {
        let data: Vec<u8> =
            solana_program::hash::hash(b"global:gauge_commit_vote_v2").to_bytes()[..8].to_vec();
        let set_weight_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.gaugemeister.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge_voter.key(), false),
                AccountMeta::new_readonly(ctx.accounts.gauge_vote.key(), false),
                AccountMeta::new(ctx.accounts.epoch_gauge.key(), false),
                AccountMeta::new(ctx.accounts.epoch_gauge_voter.key(), false),
                AccountMeta::new(ctx.accounts.epoch_gauge_vote.key(), false),
                AccountMeta::new(ctx.accounts.vote_delegate.key(), true),
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
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
        require_keys_eq!(expected_vote_delegate, ctx.accounts.vote_delegate.key());
        invoke_signed(
            &set_weight_ix,
            &[
                ctx.accounts.gaugemeister.to_account_info(),
                ctx.accounts.gauge.to_account_info(),
                ctx.accounts.gauge_voter.to_account_info(),
                ctx.accounts.gauge_vote.to_account_info(),
                ctx.accounts.epoch_gauge.to_account_info(),
                ctx.accounts.epoch_gauge_voter.to_account_info(),
                ctx.accounts.epoch_gauge_vote.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
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

    pub fn vote_buy_refund(ctx: Context<VoteBuyRefund>, epoch: u32) -> Result<()> {
        msg!(
            "Epoch: {} Current Rewards epoch {}",
            epoch,
            ctx.accounts.gaugemeister.current_rewards_epoch
        );
        let mut refund_amount = ctx.accounts.vote_buy.amount;
        if epoch < ctx.accounts.gaugemeister.current_rewards_epoch {
            msg!("Claiming refund for expired claims");
        } else {
            msg!("Claiming refund for excess buy value");
            if let Some(max_amount) = ctx.accounts.vote_buy.max_amount {
                refund_amount = ctx
                    .accounts
                    .vote_buy
                    .amount
                    .checked_sub(max_amount)
                    .ok_or(errors::VoteMarketError::InvalidRefund)?;
                ctx.accounts.vote_buy.amount -= refund_amount;
            } else {
                return err!(errors::VoteMarketError::MaxVoteBuyAmountNotSet);
            }
        }
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.buyer_token_account.key(),
            &ctx.accounts.vote_buy.key(),
            &[],
            refund_amount,
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
                ctx.accounts.buyer_token_account.to_account_info(),
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
        Ok(())
    }
}

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
    pub seller_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = vote_buy,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = admin,
    )]
    /// CHECK Checked by seed constraints
    pub treasury: Box<Account<'info, TokenAccount>>,
    /// CHECK Not enough stack space to deserialize. Only used to check treasury seeds.
    pub admin: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister, has_one = script_authority, has_one = admin)]
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
    #[account(has_one = gauge, owner = gauge_program.key())]
    // Seeds checked in instruction body
    pub epoch_gauge: Account<'info, gauge_state::EpochGauge>,
    #[account(mut, owner = gauge_program.key())]
    // Seeds checked in instruction body
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
pub struct CommitVote<'info> {
    #[account(has_one = gaugemeister, has_one = script_authority)]
    pub config: Account<'info, VoteMarketConfig>,
    #[account(mut)]
    pub script_authority: Signer<'info>,
    #[account(owner = gauge_program.key())]
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    #[account(
    seeds=[b"GaugeVote",
    gauge_voter.key().as_ref(),
    gauge.key().as_ref()],
    bump,
    seeds::program = gauge_program.key(),
    )]
    pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    #[account(mut)]
    pub epoch_gauge: Account<'info, gauge_state::EpochGauge>,
    #[account(mut,
    has_one = gauge_voter, owner = gauge_program.key(),
    seeds=[b"EpochGaugeVoter",
    gauge_voter.key().as_ref(),
    epoch.to_le_bytes().as_ref()],
    bump,
    seeds::program = gauge_program.key(),
    )]
    pub epoch_gauge_voter: Account<'info, gauge_state::EpochGaugeVoter>,
    #[account(mut,
    seeds=[b"EpochGaugeVote",
    gauge_vote.key().as_ref(),
    epoch_gauge_voter.voting_epoch.to_le_bytes().as_ref()],
    bump,
    seeds::program = gauge_program.key(),
    )]
    /// CHECK This will be initialized through a CPI
    pub epoch_gauge_vote: UncheckedAccount<'info>,
    #[account(mut,
    seeds =
    [b"vote-delegate", config.key().as_ref()],
    bump)]
    pub vote_delegate: SystemAccount<'info>,
    pub gauge_program: Program<'info, GaugeProgram>,
    pub system_program: Program<'info, System>,
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
#[derive(Accounts)]
#[instruction(epoch: u32)]
pub struct VoteBuyRefund<'info> {
    pub buyer: Signer<'info>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = vote_buy
    )]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(
    mut,
    has_one = mint,
    has_one = buyer,
    seeds = [
    b"vote-buy".as_ref(),
    epoch.to_le_bytes().as_ref(),
    config.key().as_ref(),
    gauge.key().as_ref()],
    bump
    )]
    pub vote_buy: Account<'info, VoteBuy>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister)]
    pub config: Account<'info, VoteMarketConfig>,
    pub gauge: Account<'info, gauge_state::Gauge>,
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    pub token_program: Program<'info, Token>,
}
