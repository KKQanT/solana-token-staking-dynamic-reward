use anchor_lang::prelude::*;
use crate::state::{EpochStateAccount};
use crate::utils::print_epoch_state_account;
use crate::constant::{EPOCH_DURATION, EPOCH_START_TS};
use crate::errors::AtaSkakingError;

#[derive(Accounts)]
#[instruction(
  target_epoch: i64, 
  prev_epoch: i64,
  pool_owner: Pubkey,
  target_epoch_bump: u8,
  prev_epoch_bump: u8
)]

pub struct UpdateEpoch<'info> {
  #[account(
    mut,
    seeds=[
      b"epoch_state",
      target_epoch.to_le_bytes().as_ref(),
      pool_owner.as_ref(),
    ],
    bump = target_epoch_bump
  )]
  pub epoch_state_account : Account<'info, EpochStateAccount>,
  #[account(
    seeds=[
      b"epoch_state",
      prev_epoch.to_le_bytes().as_ref(),
      pool_owner.as_ref(),
    ],
    bump = prev_epoch_bump
  )]
  pub prev_epoch_state_account : Account<'info, EpochStateAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>
}

pub fn handler(
  ctx: Context<UpdateEpoch>,
  target_epoch: i64, 
  _prev_epoch: i64,
  _pool_owner: Pubkey,
  _target_epoch_bump: u8,
  _prev_epoch_bump: u8
) -> Result<()> {
  let now_ts = Clock::get().unwrap().unix_timestamp;
  let expected_current_epoch = (now_ts - EPOCH_START_TS)/EPOCH_DURATION;

  if expected_current_epoch < target_epoch {
    msg!("expected_current_epoch < target_epoch");
    return err!(AtaSkakingError::UnknownError)
  }

  let epoch_state_account = &mut ctx.accounts.epoch_state_account;
  let prev_epoch_state_account = &ctx.accounts.prev_epoch_state_account;

  msg!("prev_epoch");
  print_epoch_state_account(prev_epoch_state_account);
  msg!("target_epoch");
  print_epoch_state_account(epoch_state_account);

  if epoch_state_account.total_weighted_stake > 0 {
    msg!("total_weighted_stake > 0");
    return err!(AtaSkakingError::UnknownError)
  }

  if prev_epoch_state_account.total_weighted_stake == 0 {
    msg!("total_weighted_stake_prev = 0");
    return err!(AtaSkakingError::UnknownError)
  }

  let prev_total_weighted_stake = prev_epoch_state_account.total_weighted_stake;
  epoch_state_account.total_weighted_stake = prev_total_weighted_stake;

  msg!("prev_epoch");
  print_epoch_state_account(prev_epoch_state_account);
  msg!("updated_target_epoch");
  print_epoch_state_account(epoch_state_account);

  Ok(())

}