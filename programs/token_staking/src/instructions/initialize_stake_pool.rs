use anchor_lang::prelude::*;

use crate::state::{PoolAccount};
use crate::utils::print_pool_account;

#[derive(Accounts)]
pub struct  InitializeStakePool<'info> {
  #[account(
    init,
    seeds=[b"pool", pool_owner.key().as_ref()],
    bump,
    payer = pool_owner,
    space = PoolAccount::LEN,
)]
pub pool_account: Account<'info, PoolAccount>,
#[account(mut)]
pub pool_owner: Signer<'info>,
pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeStakePool>) -> Result<()> {
  msg!("creating stake pool");
  let pool_account = &mut ctx.accounts.pool_account;
  pool_account.owner = ctx.accounts.pool_owner.key();
  print_pool_account(pool_account);
  Ok(())
}