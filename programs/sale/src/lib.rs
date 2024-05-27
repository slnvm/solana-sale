use anchor_lang::prelude::*;
use instructions::*;
pub mod auth;
pub mod errors;
pub mod events;
pub mod state;
pub mod instructions;

declare_id!("9jFfc1hUhszYVtGN1nJY4jGbiiEZVhVNEhd5nvqe9y67");

#[program]
pub mod sale {
  use super::*;

  pub fn initialize(
    ctx: Context<InitSale>,
  ) -> Result<()> {
    instructions::sale::initialize_sale(ctx)
  }

  pub fn set_sale_investment(
    ctx: Context<SetSaleInvestment>,
    max_investment: u64,
    min_investment: u64,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::sale::set_sale_investment(ctx, max_investment, min_investment)
  }

  pub fn set_sale_ref_reward(
    ctx: Context<SetSaleReward>,
    main_reward: u64,
    secondary_reward: u64,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::sale::set_sale_reward(ctx, main_reward, secondary_reward)
  }

  pub fn open_sale(
    ctx: Context<SetSaleOpened>,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::sale::open_sale(ctx)
  }

  pub fn close_sale(
    ctx: Context<SetSaleClosed>,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::sale::close_sale(ctx)
  }

  pub fn deposit(
    ctx: Context<Deposit>,
    ref_key: Pubkey,
    amount: u64,
  ) -> Result<()> {
    instructions::sale::deposit(ctx, ref_key, amount)
  }

  pub fn deposit_usdc(
    ctx: Context<DepositUSDC>,
    ref_key: Pubkey,
    amount: u64,
  ) -> Result<()> {
    instructions::sale::deposit_usdc(ctx, ref_key, amount)
  }

  pub fn deposit_usdt(
    ctx: Context<DepositUSDT>,
    ref_key: Pubkey,
    amount: u64,
  ) -> Result<()> {
    instructions::sale::deposit_usdt(ctx, ref_key, amount)
  }

  pub fn init_round(
    ctx: Context<InitRound>,
    id: i16,
    price: u64,
    total_supply: u128,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::round::initialize_round(ctx, id, price, total_supply)
  }

  pub fn set_round_price(
    ctx: Context<SetRoundPrice>,
    price: u64,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::round::set_round_price(ctx, price)
  }

  pub fn set_round_supply(
    ctx: Context<SetRoundSupply>,
    total_supply: u128,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::round::set_round_supply(ctx, total_supply)
  }

  pub fn open_round(
    ctx: Context<SetRoundOpened>,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::round::open_round(ctx)
  }

  pub fn close_round(
    ctx: Context<SetRoundClosed>,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::round::close_round(ctx)
  }

  pub fn init_referral(
    ctx: Context<InitReferral>,
    _ref_key: Pubkey,
    main_reward: u64,
    secondary_reward: u64,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::referral::initialize_referral(ctx, main_reward, secondary_reward)
  }

  pub fn set_referral_reward(
    ctx: Context<SetReferralReward>,
    main_reward: u64,
    secondary_reward: u64,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::referral::set_referral_reward(ctx, main_reward, secondary_reward)
  }

  pub fn enable_referral(
    ctx: Context<SetReferralEnabled>,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::referral::enable_referral(ctx)
  }

  pub fn disable_referral(
    ctx: Context<SetReferralDisabled>,
  ) -> Result<()> {
    if !auth::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Sale::Unauthorized);
    }

    instructions::referral::disable_referral(ctx)
  }

  pub fn withdraw_ref(
    ctx: Context<Withdraw>,
  ) -> Result<()> {
    instructions::referral::withdraw(ctx)
  }

  pub fn withdraw_ref_usdc(
    ctx: Context<WithdrawUSDC>,
  ) -> Result<()> {
    instructions::referral::withdraw_usdc(ctx)
  }

  pub fn withdraw_ref_usdt(
    ctx: Context<WithdrawUSDT>,
  ) -> Result<()> {
    instructions::referral::withdraw_usdt(ctx)
  }
}
