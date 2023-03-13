use anchor_lang::prelude::*;
use crate::state::{VaultAccount, PoolAccount};
use crate::utils::{print_vault_account};

#[derive(Accounts)]
#[instruction( 
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  vault_bump: u8,
  pool_bump: u8,
)]
pub struct InitUnstake<'info> {
  #[account(
    mut,
    seeds = [
      b"vault",
      vault_id.as_ref(),
      pool_account.key().as_ref(),
      user.key().as_ref()
    ],
    bump=vault_bump
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
  pub user: Signer<'info>,
}

pub fn handler(
  ctx: Context<InitUnstake>,
  _vault_id: Pubkey,
  _pool_account_owner: Pubkey,
  _vault_bump: u8,
  _pool_bump: u8,
) -> Result<()> {
  let vault_account = &mut ctx.accounts.vault_account;
  vault_account.initialized_close_vault = true;
  print_vault_account(vault_account);
  Ok(())
}