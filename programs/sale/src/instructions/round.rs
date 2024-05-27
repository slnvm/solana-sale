use anchor_lang::prelude::*;
use crate::state::round::Round;
use crate::state::sale::Sale;

pub fn initialize_round(
  ctx: Context<InitRound>,
  id: i16,
  price: u64,
  total_supply: u128,
) -> Result<()> {
  let round = &mut ctx.accounts.round;
  round.init(id, price, total_supply)
}

pub fn set_round_price(
  ctx: Context<SetRoundPrice>,
  price: u64,
) -> Result<()> {
  let round = &mut ctx.accounts.round;
  round.set_price(price)
}

pub fn set_round_supply(
  ctx: Context<SetRoundSupply>,
  total_supply: u128
) -> Result<()> {
  let round = &mut ctx.accounts.round;
  round.set_total_supply(total_supply)
}

pub fn open_round(
  ctx: Context<SetRoundOpened>,
) -> Result<()> {
  let round = &mut ctx.accounts.round;
  round.set_open().unwrap();

  let sale = &mut ctx.accounts.sale;
  sale.set_round(round.get_id())
}

pub fn close_round(
  ctx: Context<SetRoundClosed>,
) -> Result<()> {
  let round = &mut ctx.accounts.round;
  round.set_close()
}

pub const ROUND_TAG: &[u8] = b"ROUND";

#[derive(Accounts)]
#[instruction(id: i16)]
pub struct InitRound<'info> {
  #[account(
    init,
    payer = payer,
    space = 680,
    seeds = [
      ROUND_TAG,
      b"_",
      &id.to_le_bytes()
    ],
    bump,
  )]
  pub round: Account<'info, Round>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct SetRoundPrice<'info> {
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(total_supply: u128)]
pub struct SetRoundSupply<'info> {
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetRoundOpened<'info> {
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetRoundClosed<'info> {
  #[account(mut)]
  pub round: Account<'info, Round>,
  #[account(mut)]
  pub sale: Account<'info, Sale>,
  #[account(mut)]
  pub payer: Signer<'info>,
}