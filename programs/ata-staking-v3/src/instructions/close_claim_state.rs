use anchor_lang::prelude::*;
use crate::state::{
  ClaimState, 
  VaultAccount,
  PoolAccount
};

#[derive(Accounts)]
#[instruction( 
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  epoch: i64,
  vault_bump: u8,
  pool_bump: u8,
  claim_state_bump: u8,
)]

pub struct CloseClaimReward<'info> {
  #[account(
    seeds = [
      b"vault",
      vault_id.as_ref(),
      pool_account.key().as_ref(),
      user.key().as_ref()
    ],
    bump=vault_bump,
  )]
  pub vault_account: Account<'info, VaultAccount>,
  #[account(
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
      b"claim_state",
      vault_account.key().as_ref(),
      pool_account.key().as_ref(),
      epoch.to_le_bytes().as_ref()
    ],
    bump=claim_state_bump,
    close = user
  )]
  pub claim_state_account : Account<'info, ClaimState>,
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  _ctx: Context<CloseClaimReward>,
  _vault_id: Pubkey,
  _pool_account_owner: Pubkey,
  _epoch: i64,
  _vault_bump: u8,
  _pool_bump: u8,
  _claim_state_bump: u8,
) -> Result<()> {
  Ok(())
}