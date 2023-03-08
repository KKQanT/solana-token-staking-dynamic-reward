use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use crate::state::{
  VaultAccount, 
  PoolAccount, 
  EpochStateAccount,
  ClaimState
};
use crate::errors::AtaSkakingError;
use crate::utils::{print_vault_account, print_epoch_state_account};
use crate::constant::time::{EPOCH_DURATION, EPOCH_START_TS};

#[derive(Accounts)]
#[instruction( 
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  epoch: i64,
)]
pub struct ClaimReward<'info> {
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
}