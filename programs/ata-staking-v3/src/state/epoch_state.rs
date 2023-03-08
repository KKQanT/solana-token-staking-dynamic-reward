use anchor_lang::prelude::*;

#[account]
pub struct EpochStateAccount {
  pub total_weighted_stake: u64, //8
  pub total_reward_per_epoch: u64, //8
}

impl EpochStateAccount {
  pub const LEN : usize = 8 + 8 + 8;
}