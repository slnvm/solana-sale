use anchor_lang::prelude::*;

#[account]
pub struct Beneficiary {
  token_amount: u128,
}

impl Beneficiary {
  pub fn init(
    &mut self,
  ) -> Result<()> {
    self.token_amount = 0;

    Ok(())
  }

  pub fn set_token_amount(
    &mut self,
    token_amount: u128,
  ) -> Result<()> {
    self.token_amount += token_amount;

    Ok(())
  }

  pub fn get_token_amount(
    &mut self,
  ) -> u128 {
    self.token_amount
  }
}