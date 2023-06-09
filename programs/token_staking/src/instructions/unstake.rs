use anchor_lang::prelude::*;
use anchor_spl::{token};
use crate::get_ten_time_weight;
use crate::state::{VaultAccount, PoolAccount, EpochStateAccount};
use crate::errors::AtaSkakingError;
use crate::constant::{ATA_TOKEN_ADDRESS, EPOCH_DURATION, EPOCH_START_TS};
use crate::utils::print_epoch_state_account;

#[derive(Accounts)]
#[instruction(
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  epoch: i64,
  vault_bump: u8,
  pool_bump: u8,
  epoch_bump: u8,
)]

pub struct Unstake<'info> {
  #[account(
    mut,
    seeds=[
        b"vault",
        vault_id.key().as_ref(), 
        pool_account.key().as_ref(), 
        user.key().as_ref()
        ],
    bump=vault_bump,
    close = user
)]
pub vault_account: Account<'info, VaultAccount>,
#[account(
  seeds=[
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
#[account(mut)]
pub user: Signer<'info>,
#[account(mut)]
pub user_ata_token_account: Box<Account<'info, token::TokenAccount>>,
#[account(mut)]
pub vault_ata_token_account: Box<Account<'info, token::TokenAccount>>,
pub system_program: Program<'info, System>,
pub token_program: Program<'info, token::Token>,
}

pub fn handler(
  ctx: Context<Unstake>,
  vault_id: Pubkey,
  _pool_account_owner: Pubkey,
  epoch: i64,
  vault_bump: u8,
  _pool_bump: u8,
  _epoch_bump: u8,
) -> Result<()> {
  
  let vault_account = &mut ctx.accounts.vault_account;
  let now_ts = Clock::get().unwrap().unix_timestamp;

  if  (vault_account.package_number != 1) && (now_ts < vault_account.unlock_time) {
    msg!("invalid package number");
    return  err!(AtaSkakingError::UnknownError);
  }

  if ctx.accounts.vault_ata_token_account.mint != ATA_TOKEN_ADDRESS.parse::<Pubkey>().unwrap() {
    msg!("invalid ATA_TOKEN_ADDRESS");
    return err!(AtaSkakingError::UnknownError);
  }

  let pool_account_address = ctx.accounts.pool_account.key();
  let user_key = ctx.accounts.user.key();
  let vault_seeds = &[
      b"vault",
      vault_id.as_ref(),
      pool_account_address.as_ref(),
      user_key.as_ref(),
      &[vault_bump]
  ];

  let vault_signer = [&vault_seeds[..]];

  let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(), 
    token::Transfer {
        from: ctx.accounts.vault_ata_token_account.to_account_info(),
        to: ctx.accounts.user_ata_token_account.to_account_info(),
        authority: vault_account.to_account_info()
    }, 
    &vault_signer
  );

  token::transfer(cpi_ctx, vault_account.staked_amount)?;

  let weight = get_ten_time_weight(&vault_account.package_number);

  let expected_current_epoch = (now_ts - EPOCH_START_TS)/EPOCH_DURATION;

  if epoch != expected_current_epoch {
    msg!("invalid epoch");
    return err!(AtaSkakingError::UnknownError)
  }

  let epoch_state_account = &mut ctx.accounts.epoch_state_account;

  if epoch_state_account.total_weighted_stake == 0 {
    msg!("total_weighted_stake = 0");
    return err!(AtaSkakingError::UnknownError);
  }

  epoch_state_account.total_weighted_stake -= (weight/10)*vault_account.staked_amount;

  print_epoch_state_account(epoch_state_account);

  let should_close_vault_token_account = {
    ctx.accounts.vault_ata_token_account.reload()?;
    ctx.accounts.vault_ata_token_account.amount == 0
  };

  if should_close_vault_token_account {
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::CloseAccount {
            account: ctx.accounts.vault_ata_token_account.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: vault_account.to_account_info()
        },
        &vault_signer
    );
    token::close_account(cpi_ctx)?;
  }

  Ok(())
}
