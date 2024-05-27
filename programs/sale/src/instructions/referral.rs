use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use crate::auth::{ USDC, USDT };

use crate::events;
use crate::errors;
use crate::state::referral::*;

pub fn initialize_referral(
  ctx: Context<InitReferral>,
  main_reward: u64,
  secondary_reward: u64,
) -> Result<()> {
  let referral = &mut ctx.accounts.referral;
  referral.init(main_reward, secondary_reward)
}

pub fn set_referral_reward(
  ctx: Context<SetReferralReward>,
  main_reward: u64,
  secondary_reward: u64,
) -> Result<()> {
  let referral = &mut ctx.accounts.referral;
  referral.set_reward(main_reward, secondary_reward)
}

pub fn enable_referral(
  ctx: Context<SetReferralEnabled>,
) -> Result<()> {
  let referral = &mut ctx.accounts.referral;
  referral.enable()
}

pub fn disable_referral(
  ctx: Context<SetReferralDisabled>,
) -> Result<()> {
  let referral = &mut ctx.accounts.referral;
  referral.disable()
}

pub fn withdraw(
  ctx: Context<Withdraw>,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let referral = &mut ctx.accounts.referral;
  
  let sol_reward = referral.get_sol_reward_amount();
  if sol_reward > 0 {
    referral.reset_sol_reward_amount().unwrap();

    referral.sub_lamports(sol_reward).unwrap();
    payer.add_lamports(sol_reward).unwrap();

    emit!(events::WithdrawSolEvent {
      referral: payer.key(),
      sol_amount: sol_reward,
    });
  }

  Ok(())
}

pub fn withdraw_usdc(
  ctx: Context<WithdrawUSDC>,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let referral = &mut ctx.accounts.referral;
  
  let referral_ata = &ctx.accounts.referral_ata;
  let referral_pda_ata = &ctx.accounts.referral_pda_ata;
  let program = &ctx.accounts.token_program;

  let amount = referral.get_usdc_reward_amount();
  if amount == 0 {
    return err!(errors::Sale::ReferralNoFunds);
  }

  referral.reset_usdc_reward_amount().unwrap();

  let payer_key = payer.key();
  let bump = &[ctx.bumps.referral];
  let seeds: &[&[u8]] = &[REFERRAL_TAG, b"_", payer_key.as_ref(), bump];
  let signer_seeds = &[&seeds[..]];

  let cpi_accounts = SplTransfer {
    from: referral_pda_ata.to_account_info(),
    to: referral_ata.to_account_info(),
    authority: referral.to_account_info(),
  };
  let ctx = CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds);
  token::transfer(ctx, amount).unwrap();

  emit!(events::WithdrawUsdcEvent {
    referral: payer.key(),
    usdc_amount: amount,
  });

  Ok(())
}

pub fn withdraw_usdt(
  ctx: Context<WithdrawUSDT>,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let referral = &mut ctx.accounts.referral;
  
  let referral_ata = &ctx.accounts.referral_ata;
  let referral_pda_ata = &ctx.accounts.referral_pda_ata;
  let program = &ctx.accounts.token_program;

  let amount = referral.get_usdt_reward_amount();
  if amount == 0 {
    return err!(errors::Sale::ReferralNoFunds);
  }

  referral.reset_usdt_reward_amount().unwrap();

  let payer_key = payer.key();
  let bump = &[ctx.bumps.referral];
  let seeds: &[&[u8]] = &[REFERRAL_TAG, b"_", payer_key.as_ref(), bump];
  let signer_seeds = &[&seeds[..]];

  let cpi_accounts = SplTransfer {
    from: referral_pda_ata.to_account_info(),
    to: referral_ata.to_account_info(),
    authority: referral.to_account_info(),
  };
  let ctx = CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds);
  token::transfer(ctx, amount).unwrap();

  emit!(events::WithdrawUsdtEvent {
    referral: payer.key(),
    usdt_amount: amount,
  });

  Ok(())
}

pub const REFERRAL_TAG: &[u8] = b"REFERRAL";
pub const EMPTY_REFERRAL_KEY: &str  = "4B8zY1AsDUhv1s5Ftvh8QLvXJm9En2M1bNvrWinYoLDv";

#[derive(Accounts)]
#[instruction(ref_key: Pubkey)]
pub struct InitReferral<'info> {
  #[account(
    init,
    payer = payer,
    space = 680,
    seeds = [
      REFERRAL_TAG,
      b"_",
      ref_key.key().as_ref()
    ],
    bump
  )]
  pub referral: Account<'info, Referral>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetReferralReward<'info> {
  #[account(mut)]
  pub referral: Account<'info, Referral>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetReferralEnabled<'info> {
  #[account(mut)]
  pub referral: Account<'info, Referral>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetReferralDisabled<'info> {
  #[account(mut)]
  pub referral: Account<'info, Referral>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
  #[account(
    mut,
    seeds = [
      REFERRAL_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub referral: Account<'info, Referral>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawUSDC<'info> {
  #[account(
    mut,
    seeds = [
      REFERRAL_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub referral: Account<'info, Referral>,
  #[account(
    mut,
    constraint = referral_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = referral_ata.owner == payer.key(),
  )]
  pub referral_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = referral_pda_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = referral_pda_ata.owner == referral.key(),
  )]
  pub referral_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawUSDT<'info> {
  #[account(
    mut,
    seeds = [
      REFERRAL_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub referral: Account<'info, Referral>,
  #[account(
    mut,
    constraint = referral_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = referral_ata.owner == payer.key(),
  )]
  pub referral_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = referral_pda_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = referral_pda_ata.owner == referral.key(),
  )]
  pub referral_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  #[account(mut)]
  pub payer: Signer<'info>,
}