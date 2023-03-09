use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use crate::state::{VaultAccount, PoolAccount, EpochStateAccount};
use crate::errors::AtaSkakingError;
use crate::utils::{print_vault_account, print_epoch_state_account};
use crate::constant::time::{EPOCH_DURATION, EPOCH_START_TS};

#[derive(Accounts)]
#[instruction(
  pool_account_owner: Pubkey, 
  vault_id: Pubkey,
  current_epoch: i64,
  pool_bump: u8,
  epoch_bump: u8,
)]
pub struct Stake<'info> {
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
      current_epoch.to_le_bytes().as_ref(),
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
  pub vault_ata_token_account: Box<Account<'info, token::TokenAccount>>, /// checked in handler
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
}

pub fn handler(
  ctx: Context<Stake>, 
  _pool_account_owner: Pubkey, 
  vault_id: Pubkey,
  current_epoch: i64,
  _pool_bump: u8,
  _epoch_bump: u8,
  staked_amount: u64,
  package_number: u8,
) -> Result<()> {
  
  if package_number < 1 && package_number > 4 {
    return  err!(AtaSkakingError::UnknownError);
  }

  let staked_time = Clock::get().unwrap().unix_timestamp;

  let lock_duration: i64 = if package_number == 1 {
    crate::constant::LOCK_DURTION_1 //1 month
  } else if package_number == 2 {
    crate::constant::LOCK_DURTION_2 //3 months
  } else if package_number == 3 {
    crate::constant::LOCK_DURTION_3 // 1 year
  } else {
    crate::constant::LOCK_DURTION_4 // 2 years
  };
  
  let vault_account = &mut ctx.accounts.vault_account;
  vault_account.owner = ctx.accounts.user.key();
  vault_account.pool = ctx.accounts.pool_account.key();
  vault_account.vault_id = vault_id;
  vault_account.package_number = package_number;
  vault_account.staked_amount = staked_amount;
  vault_account.staked_time = staked_time;
  vault_account.unlock_time = staked_time + lock_duration;
  vault_account.use_nft = false;

  
  let expected_vault_token_account = associated_token::get_associated_token_address(
    &vault_account.key(), 
    &crate::constant::ATA_TOKEN_ADDRESS.parse::<Pubkey>().unwrap()
  );

  if ctx.accounts.vault_ata_token_account.key() != expected_vault_token_account {
    return err!(AtaSkakingError::UnknownError);
  }

  let cpi_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    token::Transfer {
        from: ctx.accounts.user_ata_token_account.to_account_info(),
        to: ctx.accounts.vault_ata_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    },
  );
  token::transfer(cpi_ctx, staked_amount)?;

  msg!("token transfered");

  msg!("vault detail");
  print_vault_account(vault_account);

  let weight = 1;

  let expected_current_epoch = (staked_time - EPOCH_START_TS)/EPOCH_DURATION;

  if current_epoch != expected_current_epoch {
    return err!(AtaSkakingError::UnknownError)
  }

  let epoch_state_account = &mut ctx.accounts.epoch_state_account;

  if epoch_state_account.total_weighted_stake == 0 {
    return err!(AtaSkakingError::UnknownError);
  }
  epoch_state_account.total_weighted_stake += weight*staked_amount;
  

  print_epoch_state_account(epoch_state_account);
  
  Ok(())
}
