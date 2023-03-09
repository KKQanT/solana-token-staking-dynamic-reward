use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::state::{
  VaultAccount, 
  PoolAccount, 
  EpochStateAccount,
  ClaimState
};
use crate::errors::AtaSkakingError;

#[derive(Accounts)]
#[instruction( 
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  epoch: i64,
  vault_bump: u8,
  pool_bump: u8,
  claim_state_bump: u8,
  epoch_bump: u8
)]
pub struct ClaimReward<'info> {
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
  #[account(
    mut,
    seeds = [
      b"claim_state",
      vault_account.key().as_ref(),
      pool_account.key().as_ref(),
      epoch.to_le_bytes().as_ref()
    ],
    bump=claim_state_bump,
  )]
  pub claim_state_account : Account<'info, ClaimState>,
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
  pub user: Signer<'info>,
  #[account(mut)]
  pub user_ata_token_account: Box<Account<'info, token::TokenAccount>>,
  #[account(mut)]
  pub pool_ata_token_account: Box<Account<'info, token::TokenAccount>>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
}

pub fn handler(
  ctx: Context<ClaimReward>,
  _vault_id: Pubkey,
  pool_account_owner: Pubkey,
  _epoch: i64,
  _vault_bump: u8,
  pool_bump:u8,
  _claim_state_bump: u8,
  _epoch_bump: u8
) -> Result<()> {
  let vault_account = &ctx.accounts.vault_account;
  let now_ts = Clock::get().unwrap().unix_timestamp;
  if now_ts < vault_account.unlock_time {
    return err!(AtaSkakingError::UnknownError);
  }
  
  let claim_state_account = &mut ctx.accounts.claim_state_account;
  if claim_state_account.is_claimed {
    return err!(AtaSkakingError::UnknownError);
  } 

  let epoch_state_account = &ctx.accounts.epoch_state_account;

  let total_weighted_stake: u64 = epoch_state_account.total_weighted_stake;
  let total_reward_per_epoch: u64 = epoch_state_account.total_reward_per_epoch;
  let weight: u64 = 1;
  let weighted_stake: u64 = vault_account.staked_amount*weight;
  
  if weighted_stake > total_weighted_stake {
    return err!(AtaSkakingError::UnknownError);
  }

  let reward_amount = (weighted_stake/total_weighted_stake) * total_reward_per_epoch;

  let pool_seeds = &[
      b"pool",
      pool_account_owner.as_ref(),
      &[pool_bump]
  ];
  let pool_signer = [&pool_seeds[..]];
  let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    token::Transfer {
        from: ctx.accounts.pool_ata_token_account.to_account_info(),
        to: ctx.accounts.user_ata_token_account.to_account_info(),
        authority: ctx.accounts.pool_account.to_account_info()
    },
    &pool_signer
  );

  token::transfer(cpi_ctx, reward_amount)?;

  claim_state_account.is_claimed = true;

  Ok(())

}