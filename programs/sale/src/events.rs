use anchor_lang::prelude::*;

#[event]
pub struct DepositSolEvent {
  pub round: i16,
  pub beneficiary: Pubkey,
  pub referral: Pubkey,
  pub sol_amount: u64,
  pub token_amount: u128,
}

#[event]
pub struct DepositUsdtEvent {
  pub round: i16,
  pub beneficiary: Pubkey,
  pub referral: Pubkey,
  pub usdt_amount: u64,
  pub token_amount: u128,
}

#[event]
pub struct DepositUsdcEvent {
  pub round: i16,
  pub beneficiary: Pubkey,
  pub referral: Pubkey,
  pub usdc_amount: u64,
  pub token_amount: u128,
}

#[event]
pub struct WithdrawSolEvent {
  pub referral: Pubkey,
  pub sol_amount: u64,
}

#[event]
pub struct WithdrawUsdtEvent {
  pub referral: Pubkey,
  pub usdt_amount: u64,
}

#[event]
pub struct WithdrawUsdcEvent {
  pub referral: Pubkey,
  pub usdc_amount: u64,
}
