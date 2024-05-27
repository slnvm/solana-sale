use anchor_lang::prelude::*;

#[error_code]
pub enum Sale {
  #[msg("Unauthorized")]
  Unauthorized,
  #[msg("Sale already opened")]
  SaleOpened,
  #[msg("Sale already closed")]
  SaleClosed,
  #[msg("Sale not opened")]
  SaleNotOpened,
  #[msg("Sale min investment larger than max investment")]
  SaleMinInvestmentTooLarge,
  #[msg("Sale min investment not reached")]
  SaleMinInvestmentNotReached,
  #[msg("Sale max investment exceeded")]
  SaleMaxInvestmentExceeded,
  #[msg("Sale main ref reward too large")]
  SaleMainRefRewardTooLarge,
  #[msg("Sale secondary ref reward too large")]
  SaleSecondaryRefRewardTooLarge,
  #[msg("Round supply is too small")]
  RoundSupplyTooSmall,
  #[msg("Round already opened")]
  RoundOpened,
  #[msg("Round already closed")]
  RoundClosed,
  #[msg("Round not opened")]
  RoundNotOpened,
  #[msg("Round total supply exceeded")]
  RoundSupplyExceeded,
  #[msg("Inactive round account")]
  InactiveRound,
  #[msg("Wrong price feed account")]
  WrongPriceFeedId,
  #[msg("Wrong stablecoin account")]
  WrongStablecoin,
  #[msg("Wrong treasury account")]
  WrongTreasury,
  #[msg("Oracle price is down")]
  PriceIsDown,
  #[msg("Referral no funds")]
  ReferralNoFunds,
}