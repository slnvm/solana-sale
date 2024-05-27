use anchor_lang::{
  prelude::*,
  solana_program::{ program::invoke, system_instruction::transfer },
};
use anchor_spl::token::{ self, Token, TokenAccount, Transfer as SplTransfer };
use pyth_sdk_solana::{ load_price_feed_from_account_info, PriceFeed, Price };
use std::str::FromStr;

use crate::errors;
use crate::events;
use crate::state::sale::*;
use crate::state::round::Round;
use crate::state::referral::Referral;
use crate::state::beneficiary::Beneficiary;
use crate::auth::{ SOL_USD_PRICEFEED, TREASURY, USDC, USDT, STABLE_PRECISION };
use crate::referral::{ REFERRAL_TAG, EMPTY_REFERRAL_KEY };

const PRECISION: u32                = 9;
const STALENESS_THRESHOLD: u64      = 60;

pub fn initialize_sale(
  ctx: Context<InitSale>,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;
  sale.init()
}

pub fn set_sale_investment(
  ctx: Context<SetSaleInvestment>,
  max_investment: u64,
  min_investment: u64,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;
  sale.set_investment(max_investment, min_investment)
}

pub fn set_sale_reward(
  ctx: Context<SetSaleReward>,
  main_reward: u64,
  secondary_reward: u64,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;
  sale.set_reward(main_reward, secondary_reward)
}

pub fn open_sale(
  ctx: Context<SetSaleOpened>,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;
  sale.set_open()
}

pub fn close_sale(
  ctx: Context<SetSaleClosed>,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;
  sale.set_close()
}

pub fn deposit(
  ctx: Context<Deposit>,
  ref_key: Pubkey,
  amount: u64,
) -> Result<()> {
  let to_account_infos = &mut ctx.accounts.to_account_infos();
  let payer = &mut ctx.accounts.payer;
  let sale = &mut ctx.accounts.sale;
  let round = &mut ctx.accounts.round;
  let beneficiary = &mut ctx.accounts.beneficiary;
  let referral = &mut ctx.accounts.referral;
  let price_info = &ctx.accounts.price_info;
  let treasury_info = &mut ctx.accounts.treasury_info;

  if !sale.is_open() {
    return err!(errors::Sale::SaleNotOpened);
  }

  if !round.is_open() {
    return err!(errors::Sale::RoundNotOpened);
  }

  if sale.get_round() != round.get_id() {
    return err!(errors::Sale::InactiveRound);
  }

  if Pubkey::from_str(TREASURY) != Ok(treasury_info.key()){
    return Err(error!(errors::Sale::WrongTreasury))
  };

  if Pubkey::from_str(SOL_USD_PRICEFEED) != Ok(price_info.key()){
    return Err(error!(errors::Sale::WrongPriceFeedId))
  };
  
  let (price, expo) = get_price(&price_info).unwrap();
  let usd_amount = u128::from(amount) * price / 10u128.pow(expo);
  let token_amount = usd_amount * 10u128.pow(PRECISION) / u128::from(round.get_price());

  if sale.get_max_investment() < usd_amount {
    return err!(errors::Sale::SaleMaxInvestmentExceeded);
  }

  if sale.get_min_investment() > usd_amount {
    return err!(errors::Sale::SaleMinInvestmentNotReached);
  }

  if round.get_total_sold() + token_amount > round.get_total_supply() {
    return err!(errors::Sale::RoundSupplyExceeded);
  }
  
  let (sol_reward_amount, token_reward_amount) = get_reward(sale, ref_key, referral, amount, token_amount).unwrap();
  let mut to_amount = amount;
  if sol_reward_amount > 0 {
    to_amount = to_amount - sol_reward_amount;
  }

  let instruction = &transfer(&payer.key(), &treasury_info.key(), to_amount);
  invoke(instruction, to_account_infos).unwrap();

  if sol_reward_amount > 0 {
    let instruction = &transfer(&payer.key(), &referral.key(), sol_reward_amount);
    invoke(instruction, to_account_infos).unwrap();
  }

  // Updating sale details
  sale.set_total_sold(token_amount).unwrap();

  // Updating round details
  round.set_total_sold(token_amount).unwrap();

  // Updating beneficiary details
  beneficiary.set_token_amount(token_amount).unwrap();

  // Updating referral details
  if Pubkey::from_str(EMPTY_REFERRAL_KEY) != Ok(ref_key){
    referral.set_sol_reward_amount(sol_reward_amount).unwrap();
    referral.set_token_reward_amount(token_reward_amount).unwrap();
  };

  emit!(events::DepositSolEvent {
    round: round.get_id(),
    beneficiary: payer.key(),
    referral: ref_key,
    sol_amount: amount,
    token_amount: token_amount,
  });
  Ok(())
}

pub fn deposit_usdc(
  ctx: Context<DepositUSDC>,
  ref_key: Pubkey,
  amount: u64,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let sale = &mut ctx.accounts.sale;
  let round = &mut ctx.accounts.round;
  let beneficiary = &mut ctx.accounts.beneficiary;
  let referral = &mut ctx.accounts.referral;

  let beneficiary_ata = &ctx.accounts.beneficiary_ata;
  let treasury_ata = &ctx.accounts.treasury_ata;
  let referral_pda_ata = &ctx.accounts.referral_pda_ata;
  let token_program = &ctx.accounts.token_program;

  if !sale.is_open() {
    return err!(errors::Sale::SaleNotOpened);
  }

  if !round.is_open() {
    return err!(errors::Sale::RoundNotOpened);
  }

  if sale.get_round() != round.get_id() {
    return err!(errors::Sale::InactiveRound);
  }

  let usd_amount = u128::from(amount) * 10u128.pow(STABLE_PRECISION);
  let token_amount = usd_amount * 10u128.pow (PRECISION) / u128::from(round.get_price());

  if sale.get_max_investment() < usd_amount {
    return err!(errors::Sale::SaleMaxInvestmentExceeded);
  }

  if sale.get_min_investment() > usd_amount {
    return err!(errors::Sale::SaleMinInvestmentNotReached);
  }

  if round.get_total_sold() + token_amount > round.get_total_supply() {
    return err!(errors::Sale::RoundSupplyExceeded);
  }

  let (stable_reward_amount, token_reward_amount) = get_reward(sale, ref_key, referral, amount, token_amount).unwrap();
  let mut to_amount = amount;
  if stable_reward_amount > 0 {
    to_amount = to_amount - stable_reward_amount;
  }

  let cpi_accounts = SplTransfer {
    from: beneficiary_ata.to_account_info(),
    to: treasury_ata.to_account_info(),
    authority: payer.to_account_info(),
  };
  let cpi_program = token_program.to_account_info();
  token::transfer(CpiContext::new(cpi_program, cpi_accounts), to_amount).unwrap();
  
  if stable_reward_amount > 0 {
    let cpi_accounts = SplTransfer {
      from: beneficiary_ata.to_account_info(),
      to: referral_pda_ata.to_account_info(),
      authority: payer.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), stable_reward_amount).unwrap();
  }

  // Updating sale details
  sale.set_total_sold(token_amount).unwrap();

  // Updating round details
  round.set_total_sold(token_amount).unwrap();

  // Updating beneficiary details
  beneficiary.set_token_amount(token_amount).unwrap();

  // Updating referral details
  if Pubkey::from_str(EMPTY_REFERRAL_KEY) != Ok(ref_key){
    referral.set_usdc_reward_amount(stable_reward_amount).unwrap();
    referral.set_token_reward_amount(token_reward_amount).unwrap();
  };

  emit!(events::DepositUsdcEvent {
    round: round.get_id(),
    beneficiary: payer.key(),
    referral: ref_key,
    usdc_amount: amount,
    token_amount: token_amount,
  });

  Ok(())
}

pub fn deposit_usdt(
  ctx: Context<DepositUSDT>,
  ref_key: Pubkey,
  amount: u64,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let sale = &mut ctx.accounts.sale;
  let round = &mut ctx.accounts.round;
  let beneficiary = &mut ctx.accounts.beneficiary;
  let referral = &mut ctx.accounts.referral;

  let beneficiary_ata = &ctx.accounts.beneficiary_ata;
  let treasury_ata = &ctx.accounts.treasury_ata;
  let referral_pda_ata = &ctx.accounts.referral_pda_ata;
  let token_program = &ctx.accounts.token_program;

  if !sale.is_open() {
    return err!(errors::Sale::SaleNotOpened);
  }

  if !round.is_open() {
    return err!(errors::Sale::RoundNotOpened);
  }

  if sale.get_round() != round.get_id() {
    return err!(errors::Sale::InactiveRound);
  }

  let usd_amount = u128::from(amount) * 10u128.pow(STABLE_PRECISION);
  let token_amount = usd_amount * 10u128.pow (PRECISION) / u128::from(round.get_price());

  if sale.get_max_investment() < usd_amount {
    return err!(errors::Sale::SaleMaxInvestmentExceeded);
  }

  if sale.get_min_investment() > usd_amount {
    return err!(errors::Sale::SaleMinInvestmentNotReached);
  }

  if round.get_total_sold() + token_amount > round.get_total_supply() {
    return err!(errors::Sale::RoundSupplyExceeded);
  }

  let (stable_reward_amount, token_reward_amount) = get_reward(sale, ref_key, referral, amount, token_amount).unwrap();
  let mut to_amount = amount;
  if stable_reward_amount > 0 {
    to_amount = to_amount - stable_reward_amount;
  }

  let cpi_accounts = SplTransfer {
    from: beneficiary_ata.to_account_info(),
    to: treasury_ata.to_account_info(),
    authority: payer.to_account_info(),
  };
  let cpi_program = token_program.to_account_info();
  token::transfer(CpiContext::new(cpi_program, cpi_accounts), to_amount).unwrap();
  
  if stable_reward_amount > 0 {
    let cpi_accounts = SplTransfer {
      from: beneficiary_ata.to_account_info(),
      to: referral_pda_ata.to_account_info(),
      authority: payer.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), stable_reward_amount).unwrap();
  }

  // Updating sale details
  sale.set_total_sold(token_amount).unwrap();

  // Updating round details
  round.set_total_sold(token_amount).unwrap();

  // Updating beneficiary details
  beneficiary.set_token_amount(token_amount).unwrap();

  // Updating referral details
  if Pubkey::from_str(EMPTY_REFERRAL_KEY) != Ok(ref_key){
    referral.set_usdt_reward_amount(stable_reward_amount).unwrap();
    referral.set_token_reward_amount(token_reward_amount).unwrap();
  };

  emit!(events::DepositUsdtEvent {
    round: round.get_id(),
    beneficiary: payer.key(),
    referral: ref_key,
    usdt_amount: amount,
    token_amount: token_amount,
  });

  Ok(())
}

pub fn get_price(price_info: &AccountInfo)
  -> Result<(u128, u32)>
{
  /*
  let price_feed: PriceFeed = load_price_feed_from_account_info( &price_info ).unwrap();
  let current_timestamp = Clock::get()?.unix_timestamp;
  let current_price: Price = price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

  let price = u64::try_from(current_price.price).unwrap();
  let expo = u32::try_from(-current_price.expo).unwrap();
  Ok((price, expo))
  */
  return Ok((14400000000, 8));
}

pub fn get_reward(
  sale: &mut Account<Sale>,
  ref_key: Pubkey,
  referral: &mut Account<Referral>,
  amount: u64,
  token_amount: u128,
)
  -> Result<(u64, u128)>
{
  if Pubkey::from_str(EMPTY_REFERRAL_KEY) == Ok(ref_key){
    return Ok((0, 0));
  };

  let (sale_main_reward, sale_secondary_reward) = sale.get_reward();
  let (ref_main_reward, ref_secondary_reward) = referral.get_reward();

  let main_reward = u64::max(sale_main_reward, ref_main_reward);
  let secondary_reward = u64::max(sale_secondary_reward, ref_secondary_reward);

  let sol_reward_amount = amount * main_reward / 10u64.pow(PRECISION);
  let token_reward_amount = token_amount * u128::from(secondary_reward) / 10u128.pow(PRECISION);

  Ok((sol_reward_amount, token_reward_amount))
}

#[derive(Accounts)]
pub struct InitSale<'info> {
  #[account(
    init,
    payer = payer,
    space = 680,
    seeds = [],
    bump,
  )]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(max_investment: u64, min_investment: u64)]
pub struct SetSaleInvestment<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(main_reward: u64, secondary_reward: u64)]
pub struct SetSaleReward<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSaleOpened<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSaleClosed<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

pub const BENEFICIARY_TAG: &[u8] = b"BENEFICIARY";
#[derive(Accounts)]
#[instruction(ref_key: Pubkey, amount: u64)]
pub struct Deposit<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 680,
    seeds = [
      BENEFICIARY_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub beneficiary: Account<'info, Beneficiary>,
  #[account(
    init_if_needed,
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
  /// CHECK : We will manually check this against the Pubkey of the price feed
  pub price_info : AccountInfo<'info>,
  /// CHECK : We will manually check this against the Pubkey of the treasury
  #[account(mut)]
  pub treasury_info : AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ref_key: Pubkey, amount: u64)]
pub struct DepositUSDC<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 680,
    seeds = [
      BENEFICIARY_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub beneficiary: Account<'info, Beneficiary>,
  #[account(
    init_if_needed,
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
  #[account(
    mut,
    constraint = beneficiary_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = beneficiary_ata.owner == payer.key(),
  )]
  pub beneficiary_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = treasury_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = treasury_ata.owner == TREASURY.parse::<Pubkey>().unwrap(),
  )]
  pub treasury_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = referral_pda_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = referral_pda_ata.owner == referral.key(),
  )]
  pub referral_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ref_key: Pubkey, amount: u64)]
pub struct DepositUSDT<'info> {
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 680,
    seeds = [
      BENEFICIARY_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub beneficiary: Account<'info, Beneficiary>,
  #[account(
    init_if_needed,
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
  #[account(
    mut,
    constraint = beneficiary_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = beneficiary_ata.owner == payer.key(),
  )]
  pub beneficiary_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = treasury_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = treasury_ata.owner == TREASURY.parse::<Pubkey>().unwrap(),
  )]
  pub treasury_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = referral_pda_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = referral_pda_ata.owner == referral.key(),
  )]
  pub referral_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}
