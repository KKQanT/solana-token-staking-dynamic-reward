use anchor_lang::prelude::*;
use crate::state::{EpochStateAccount};
use crate::utils::print_epoch_state_account;

#[derive(Accounts)]
#[instruction(
  total_weighted_stake: u64,
  total_reward_per_epoch: u64,
  epoch: i64,
  epoch_bump: u8
)]
pub struct AuthorityUpdateEpoch<'info> {
  #[account(
    mut,
    seeds=[
      b"epoch_state",
      epoch.to_le_bytes().as_ref(),
      pool_owner_account.key().as_ref(),
    ],
    bump=epoch_bump
  )]
  pub epoch_state_account : Account<'info, EpochStateAccount>,
  #[account(mut)]
  pub pool_owner_account: Signer<'info>,
  pub system_program: Program<'info, System>
}

pub fn handler(
  ctx: Context<AuthorityUpdateEpoch>,
  total_weighted_stake: u64,
  total_reward_per_epoch: u64,
  _epoch: i64,
  _epoch_bump: u8,
) -> Result<()> {
  let epoch_state_account = &mut ctx.accounts.epoch_state_account;
  epoch_state_account.total_weighted_stake = total_weighted_stake;
  epoch_state_account.total_reward_per_epoch = total_reward_per_epoch;
  
  print_epoch_state_account(epoch_state_account);

  Ok(())
}
