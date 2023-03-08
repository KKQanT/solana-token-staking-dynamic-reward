use anchor_lang::prelude::*;

#[account]
pub struct ClaimState {
  pub is_claimed: bool
}

impl ClaimState {
    pub const LEN : usize = 1;
}