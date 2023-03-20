use anchor_lang::prelude::*;
use crate::state::{EpochStateAccount};
use crate::utils::print_epoch_state_account;

#[derive(Accounts)]
#[instruction(total_reward_per_epoch: u64, epoch: i64)]
pub struct InitializeEpochStateAccount<'info> {
  #[account(
    init,
    seeds=[
      b"epoch_state",
      epoch.to_le_bytes().as_ref(),
      pool_owner_account.key().as_ref(),
    ],
    bump,
    payer = pool_owner_account,
    space = EpochStateAccount::LEN
  )]
  pub epoch_state_account : Account<'info, EpochStateAccount>,
  #[account(mut)]
  pub pool_owner_account: Signer<'info>,
  pub system_program: Program<'info, System>
}

pub fn handler(
  ctx: Context<InitializeEpochStateAccount>,
  total_reward_per_epoch: u64,
  _epoch: i64
) -> Result<()> {
  let epoch_state_account = &mut ctx.accounts.epoch_state_account;
  epoch_state_account.total_weighted_stake = 0;
  epoch_state_account.total_reward_per_epoch = total_reward_per_epoch;
  
  print_epoch_state_account(epoch_state_account);

  Ok(())
}
