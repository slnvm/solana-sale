use anchor_lang::prelude::*;
use crate::errors;

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum State {
  None,
  Opened,
  Closed,
}

#[account]
pub struct Round {
  id: i16,
  price: u64,
  total_sold: u128,
  total_supply: u128,
  state: State,
}

impl Round {
  pub fn init(
    &mut self,
    id: i16,
    price: u64,
    total_supply: u128,
  ) -> Result<()> {
    self.id = id;
    self.price = price;
    self.total_supply = total_supply;
    self.total_sold = 0;
    self.state = State::None;

    Ok(())
  }

  pub fn set_price(
    &mut self,
    price: u64,
  ) -> Result<()> {
    if self.state == State::Opened || self.state == State::Closed {
      return err!(errors::Sale::RoundOpened);
    }

    self.price = price;

    Ok(())
  }

  pub fn set_total_supply(
    &mut self,
    total_supply: u128,
  ) -> Result<()> {
    if self.total_sold > total_supply {
      return err!(errors::Sale::RoundSupplyTooSmall);
    }

    self.total_supply = total_supply;

    Ok(())
  }

  pub fn set_open(
    &mut self,
  ) -> Result<()> {
    if self.state == State::Opened || self.state == State::Closed {
      return err!(errors::Sale::RoundOpened);
    }

    self.state = State::Opened;

    Ok(())
  }

  pub fn set_close(
    &mut self,
  ) -> Result<()> {
    if self.state != State::Opened {
      return err!(errors::Sale::RoundClosed);
    }

    self.state = State::Closed;

    Ok(())
  }

  pub fn set_total_sold(
    &mut self,
    total_sold: u128,
  ) -> Result<()> {
    self.total_sold += total_sold;

    Ok(())
  }

  pub fn get_id(
    &mut self,
  ) -> i16 {
    self.id
  }

  pub fn get_price(
    &mut self,
  ) -> u64 {
    self.price
  }

  pub fn get_total_sold(
    &mut self,
  ) -> u128 {
    self.total_sold
  }

  pub fn get_total_supply(
    &mut self,
  ) -> u128 {
    self.total_supply
  }

  pub fn is_open(
    &self,
  ) -> bool {
    self.state == State::Opened
  }
}