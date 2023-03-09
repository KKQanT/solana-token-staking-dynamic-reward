use anchor_lang::prelude::*;
use crate::state::{
  ClaimState, 
  EpochStateAccount, 
  VaultAccount,
  PoolAccount
};
use crate::errors::AtaSkakingError;
use crate::constant::{EPOCH_DURATION, EPOCH_START_TS};

#[derive(Accounts)]
#[instruction(
  pool_account_owner: Pubkey,
  vault_id: Pubkey,
  epoch: i64,
  pool_bump: u8,
  epoch_bump: u8
)]
pub struct InitClaimReward<'info> {
  #[account(
    init,
    seeds = [
      b"claim_state",
      vault_account.key().as_ref(),
      pool_account.key().as_ref(),
      epoch.to_le_bytes().as_ref()
    ],
    bump,
    payer = user,
    space = ClaimState::LEN
  )]
  pub claim_state_account : Account<'info, ClaimState>,
  #[account(
    mut,
    seeds = [
        b"pool",
        pool_account_owner.as_ref()
    ],
    bump=pool_bump
  )]
  pub pool_account: Account<'info, PoolAccount>,
  #[account(
    mut,
    seeds = [
      b"epoch_state",
      epoch.to_le_bytes().as_ref(),
      pool_account_owner.as_ref()
    ],
    bump=epoch_bump
  )]
  pub epoch_state_account: Account<'info, EpochStateAccount>,
  #[account(
    init,
    seeds = [
      b"vault",
      vault_id.as_ref(),
      pool_account.key().as_ref(),
      user.key().as_ref()
    ],
    bump,
    payer = user,
    space = VaultAccount::LEN
  )]
  pub vault_account: Account<'info, VaultAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitClaimReward>,
  _pool_account_owner: Pubkey,
  _vault_id: Pubkey,
  epoch: i64,
  _pool_bump: u8,
  _epoch_bump: u8
) -> Result<()> {
  let epoch_state_account = &ctx.accounts.epoch_state_account;
  if epoch_state_account.total_weighted_stake == 0 {
    return err!(AtaSkakingError::UnknownError)
  }

  let vault_account = &ctx.accounts.vault_account;
  let staked_time = vault_account.staked_time;
  let staked_epoch = (staked_time - EPOCH_START_TS)/EPOCH_DURATION;

  if staked_epoch > epoch {
    return err!(AtaSkakingError::UnknownError)
  } 

  let claim_state_account = &mut ctx.accounts.claim_state_account;
  claim_state_account.is_claimed = false;

  msg!("claim state account initialized");
  msg!("is claimed: {}", claim_state_account.is_claimed);
  
  Ok(())
}