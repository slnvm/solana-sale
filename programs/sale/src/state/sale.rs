use anchor_lang::prelude::*;
use crate::errors;

pub const MAX_INVESTMENT: u64 = 1000_000_000_000_000; 
pub const MIN_INVESTMENT: u64 = 100_000_000_000;
pub const MAIN_REWARD: u64 = 50_000_000;
pub const SECONDARY_REWARD: u64 = 50_000_000;

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum State {
  None,
  Opened,
  Closed,
}

#[account]
pub struct Sale {
  max_investment: u64,
  min_investment: u64,
  main_reward: u64,
  secondary_reward: u64,
  total_sold: u128,
  round: i16,
  state: State,
  enabled: bool,
}

impl Sale {
  pub fn init(
    &mut self,
  ) -> Result<()> {
    self.round = -1;
    self.max_investment = MAX_INVESTMENT;
    self.min_investment = MIN_INVESTMENT;
    self.main_reward = MAIN_REWARD;
    self.secondary_reward = SECONDARY_REWARD;
    self.total_sold = 0;
    self.state = State::None;
    self.enabled = true;

    Ok(())
  }

  pub fn set_investment(
    &mut self,
    max_investment: u64,
    min_investment: u64,
  ) -> Result<()> {
    if max_investment < min_investment {
      return err!(errors::Sale::SaleMinInvestmentTooLarge);
    }

    self.max_investment = max_investment;
    self.min_investment = min_investment;

    Ok(())
  }

  pub fn set_reward(
    &mut self,
    main_reward: u64,
    secondary_reward: u64,
  ) -> Result<()> {
    if main_reward > 1000_000_000 {
      return err!(errors::Sale::SaleMainRefRewardTooLarge);
    }

    if secondary_reward > 1000_000_000 {
      return err!(errors::Sale::SaleSecondaryRefRewardTooLarge);
    }

    self.main_reward = main_reward;
    self.secondary_reward = secondary_reward;

    Ok(())
  }

  pub fn set_open(
    &mut self,
  ) -> Result<()> {
    if self.state == State::Opened || self.state == State::Closed {
      return err!(errors::Sale::SaleOpened);
    }

    self.state = State::Opened;

    Ok(())
  }

  pub fn set_close(
    &mut self,
  ) -> Result<()> {
    if self.state != State::Opened {
      return err!(errors::Sale::SaleClosed);
    }

    self.state = State::Closed;

    Ok(())
  }

  pub fn set_round(
    &mut self,
    round: i16,
  ) -> Result<()> {
    self.round = round;

    Ok(())
  }

  pub fn set_total_sold(
    &mut self,
    total_sold: u128,
  ) -> Result<()> {
    self.total_sold += total_sold;

    Ok(())
  }

  pub fn get_round(
    &self,
  ) -> i16 {
    self.round
  }

  pub fn get_max_investment(
    &self,
  ) -> u128 {
    u128::from(self.max_investment)
  }

  pub fn get_min_investment(
    &self,
  ) -> u128 {
    u128::from(self.min_investment)
  }

  pub fn get_total_sold(
    &self,
  ) -> u128 {
    self.total_sold
  }

  pub fn is_open(
    &self,
  ) -> bool {
    self.state == State::Opened
  }

  pub fn get_reward(
    &mut self,
  ) -> (u64, u64) {
    (self.main_reward, self.secondary_reward)
  }
}
