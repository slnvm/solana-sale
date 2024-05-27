use anchor_lang::prelude::*;

pub const TREASURY: &str = "<>";
pub const SOL_USD_PRICEFEED: &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";

pub const STABLE_PRECISION: u32 = 3;
pub const USDT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
pub const USDC: &str = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr";

const ADMINS: &[&str] = &["3tHXr7UqSpiHJsJjpPSxs823HdPysKuREaayzWFh6vxD"];

pub fn only_admin(address: Pubkey) -> bool {
  return  ADMINS.contains(&address.to_string().as_str());
}