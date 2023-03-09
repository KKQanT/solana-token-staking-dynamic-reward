use anchor_lang::prelude::*;
use anchor_spl::{token};
use crate::state::{VaultAccount, PoolAccount};
use crate::errors::AtaSkakingError;
use std::ops::DerefMut;
use std::io::Cursor;
use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use std::io::Write;
use crate::constant::ATA_TOKEN_ADDRESS;

#[derive(Accounts)]
#[instruction(
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  vault_bump: u8,
  pool_bump: u8
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
)]
pub vault_account: Account<'info, VaultAccount>,
#[account(
  mut,
  seeds=[
      b"pool",
      pool_account_owner.as_ref()
  ],
  bump=pool_bump
)]
pub pool_account: Account<'info, PoolAccount>,
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
  vault_bump: u8,
  _pool_bump: u8
) -> Result<()> {
  
  let vault_account = &mut ctx.accounts.vault_account;
  let now_ts = Clock::get().unwrap().unix_timestamp;

  if  (vault_account.package_number != 1) && (now_ts < vault_account.unlock_time) {
    return  err!(AtaSkakingError::UnknownError);
  }

  if ctx.accounts.vault_ata_token_account.mint != ATA_TOKEN_ADDRESS.parse::<Pubkey>().unwrap() {
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

  **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? +=
      **ctx.accounts.vault_account.to_account_info().try_borrow_lamports()?;
  **ctx.accounts.vault_account.to_account_info().try_borrow_mut_lamports()? = 0;

  let user_vault_account = ctx.accounts.vault_account.to_account_info();
  let mut data = user_vault_account.try_borrow_mut_data()?;
  for byte in data.deref_mut().iter_mut() {
      *byte = 0;
  }

  let dst: &mut [u8] = &mut data;
      let mut cursor = Cursor::new(dst);
      cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR)
            .map_err(|_| error!(ErrorCode::AccountDidNotSerialize))
            .unwrap();

  Ok(())
}
