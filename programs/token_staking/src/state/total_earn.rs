use anchor_lang::prelude::*;

#[account]
pub struct TotalEarn {
  pub total_earn_amount: u64
}

impl TotalEarn {
    pub const LEN : usize = 8 + 8;
}