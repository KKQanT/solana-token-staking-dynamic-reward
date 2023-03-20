use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::state::{
  VaultAccount, 
  PoolAccount, 
  EpochStateAccount,
  ClaimState,
  TotalEarn
};
use crate::{errors::*, get_ten_time_weight};
use crate::constant::{EPOCH_DURATION, EPOCH_START_TS};

#[derive(Accounts)]
#[instruction( 
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  epoch: i64,
  vault_bump: u8,
  pool_bump: u8,
  claim_state_bump: u8,
  epoch_bump: u8,
  total_earn_bump: u8
)]
pub struct ClaimReward<'info> {
  #[account(
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
    seeds = [
      b"epoch_state",
      epoch.to_le_bytes().as_ref(),
      pool_account_owner.as_ref()
    ],
    bump=epoch_bump
  )]
  pub epoch_state_account: Account<'info, EpochStateAccount>,
  #[account(
    mut,
    seeds = [
      b"total_earn",
      user.key().as_ref()
    ],
    bump = total_earn_bump
  )]
  pub total_earn_account: Account<'info, TotalEarn>,
  #[account(mut)]
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
  epoch: i64,
  _vault_bump: u8,
  pool_bump:u8,
  _claim_state_bump: u8,
  _epoch_bump: u8,
  _total_earn_bump: u8
) -> Result<()> {

  let vault_account = &ctx.accounts.vault_account;
  
  let now_ts = Clock::get().unwrap().unix_timestamp;
  if now_ts < vault_account.unlock_time {
    msg!("not unlock and vesing period time");
    return err!(TimeError::InvalidTime);
  }

  let current_epoch = (now_ts - EPOCH_START_TS)/EPOCH_DURATION;
  if epoch >= current_epoch {
    msg!("epoch >= current_epoch");
    return err!(TimeError::InvalidTime);
  }

  let staked_time = vault_account.staked_time;
  let staked_epoch = (staked_time - EPOCH_START_TS)/EPOCH_DURATION;

  if staked_epoch > epoch {
    msg!("staked_epoch > epoch");
    return err!(TimeError::InvalidEpoch)
  }

  if vault_account.package_number != 1 {
    let vesting_end_time = vault_account.vesting_end_time;
    let vesting_end_epoch = (vesting_end_time - EPOCH_START_TS)/EPOCH_DURATION;

    if epoch > vesting_end_epoch {
      msg!("epoch > vesting_end_epoch");
      return  err!(TimeError::InvalidEpoch);
    }
  }
  
  let claim_state_account = &mut ctx.accounts.claim_state_account;
  if claim_state_account.is_claimed {
    msg!("is claimed = True");
    return err!(ConditionError::InvalidCondition);
  } 

  let epoch_state_account = &ctx.accounts.epoch_state_account;

  let total_weighted_stake: u64 = epoch_state_account.total_weighted_stake;
  let total_reward_per_epoch: u64 = epoch_state_account.total_reward_per_epoch;
  let weight: u64 = get_ten_time_weight(&vault_account.package_number);
  let weighted_stake: u64 = (weight/10)*vault_account.staked_amount;
  
  if weighted_stake > total_weighted_stake {
    msg!("weighted_stake > total_weighted_stake");
    return err!(ConditionError::InvalidCondition);
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

  let total_earn_account = &mut ctx.accounts.total_earn_account;
  total_earn_account.total_earn_amount += reward_amount;

  msg!("epoch {} has is_claimed = {}", epoch, claim_state_account.is_claimed);

  Ok(())

}