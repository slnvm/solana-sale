use anchor_lang::prelude::*;

#[account]
pub struct Referral {
  main_reward: u64,
  secondary_reward: u64,

  sol_reward_amount: u64,
  usdt_reward_amount: u64,
  usdc_reward_amount: u64,
  token_reward_amount: u128,

  enabled: bool,
}

impl Referral {
  pub fn init(
    &mut self,
    main_ref_reward: u64,
    secondary_ref_reward: u64,
  ) -> Result<()> {
    self.main_reward = main_ref_reward;
    self.secondary_reward = secondary_ref_reward;

    self.sol_reward_amount = 0;
    self.usdt_reward_amount = 0;
    self.usdc_reward_amount = 0;
    self.token_reward_amount = 0;

    self.enabled = true;

    Ok(())
  }

  pub fn set_reward(
    &mut self,
    main_reward: u64,
    secondary_reward: u64,
  ) -> Result<()> {
    self.main_reward = main_reward;
    self.secondary_reward = secondary_reward;

    Ok(())
  }

  pub fn set_sol_reward_amount(
    &mut self,
    reward_amount: u64,
  ) -> Result<()> {
    self.sol_reward_amount += reward_amount;

    Ok(())
  }

  pub fn reset_sol_reward_amount(
    &mut self,
  ) -> Result<()> {
    self.sol_reward_amount = 0;

    Ok(())
  }

  pub fn set_usdt_reward_amount(
    &mut self,
    usdt_reward_amount: u64,
  ) -> Result<()> {
    self.usdt_reward_amount += usdt_reward_amount;

    Ok(())
  }

  pub fn reset_usdt_reward_amount(
    &mut self,
  ) -> Result<()> {
    self.usdt_reward_amount = 0;

    Ok(())
  }

  pub fn set_usdc_reward_amount(
    &mut self,
    usdc_reward_amount: u64,
  ) -> Result<()> {
    self.usdc_reward_amount += usdc_reward_amount;

    Ok(())
  }

  pub fn reset_usdc_reward_amount(
    &mut self,
  ) -> Result<()> {
    self.usdc_reward_amount = 0;

    Ok(())
  }

  pub fn set_token_reward_amount(
    &mut self,
    token_reward_amount: u128,
  ) -> Result<()> {
    self.token_reward_amount += token_reward_amount;

    Ok(())
  }

  pub fn get_reward(
    &mut self,
  ) -> (u64, u64) {
    (self.main_reward, self.secondary_reward)
  }

  pub fn get_sol_reward_amount(
    &mut self,
  ) -> u64 {
    self.sol_reward_amount
  }

  pub fn get_usdt_reward_amount(
    &mut self,
  ) -> u64 {
    self.usdt_reward_amount
  }

  pub fn get_usdc_reward_amount(
    &mut self,
  ) -> u64 {
    self.usdc_reward_amount
  }

  pub fn get_token_reward_amount(
    &mut self,
  ) -> u128 {
    self.token_reward_amount
  }

  pub fn enable(
    &mut self,
  ) -> Result<()> {
    self.enabled = true;

    Ok(())
  }

  pub fn disable(
    &mut self,
  ) -> Result<()> {
    self.enabled = false;

    Ok(())
  }
}