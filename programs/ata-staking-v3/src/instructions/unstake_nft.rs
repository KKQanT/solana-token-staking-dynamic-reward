use anchor_lang::prelude::*;
use anchor_spl::{token};
use crate::print_vault_account;
use crate::state::{
  VaultAccount, 
  PoolAccount, 
  EpochStateAccount, 
  WhitelistNFTInfoAccount
};
use crate::errors::AtaSkakingError;
use std::ops::DerefMut;
use std::io::Cursor;
use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use std::io::Write;
use crate::constant::{METADATA_PROGRAM_ID, EPOCH_DURATION, EPOCH_START_TS};
use crate::utils::{print_epoch_state_account};
use anchor_spl::associated_token::get_associated_token_address;


#[derive(Accounts)]
#[instruction(
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  mint_address: Pubkey,
  epoch: i64,
  vault_bump: u8,
  pool_bump: u8,
  whitelist_nft_bump: u8,
  epoch_bump: u8,
)]
pub struct UnstakeNFT<'info> {
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
  pub user_nft_token_account: Box<Account<'info, token::TokenAccount>>,
  #[account(mut)]
  pub vault_nft_token_account: Box<Account<'info, token::TokenAccount>>,
  #[account(
    mut,
    seeds=[
      b"whitelist_nft", 
      pool_account_owner.key().as_ref(), 
      mint_address.as_ref()
      ],
    bump=whitelist_nft_bump
  )]
  pub whitelist_nft_info_account: Account<'info, WhitelistNFTInfoAccount>,
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
  ///CHECK: checked via instruction
  pub metadata_account: AccountInfo<'info>,
  ///CHECK : check via #[account(address = crate::address::METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
  #[account(address = METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
  pub token_metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
}

pub fn handler(
  ctx: Context<UnstakeNFT>,
  vault_id: Pubkey,
  _pool_account_owner: Pubkey,
  mint_address: Pubkey,
  epoch: i64,
  vault_bump: u8,
  _pool_bump: u8,
  _whitelist_nft_bump: u8,
  _epoch_bump: u8,
) -> Result<()> {
  
  let vault_account = &mut ctx.accounts.vault_account;
  let now_ts = Clock::get().unwrap().unix_timestamp;

  if (vault_account.package_number != 1) && (now_ts < vault_account.unlock_time) {
    return  err!(AtaSkakingError::UnknownError);
  }

  print_vault_account(vault_account);

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

  if ctx.accounts.vault_nft_token_account.mint != mint_address {
    return err!(AtaSkakingError::UnknownError);
  }

  if ctx.accounts.user_nft_token_account.mint != mint_address {
    return err!(AtaSkakingError::UnknownError);
  }

  let expected_user_nft_token_account = get_associated_token_address(
    &user_key, 
    &mint_address
  );

  if expected_user_nft_token_account != ctx.accounts.user_nft_token_account.key() {
    return err!(AtaSkakingError::UnknownError);
  }

  let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(), 
    token::Transfer {
        from: ctx.accounts.vault_nft_token_account.to_account_info(),
        to: ctx.accounts.user_nft_token_account.to_account_info(),
        authority: vault_account.to_account_info()
    }, 
    &vault_signer
  );

  token::transfer(cpi_ctx, 1)?;

  let weight = 1;

  let expected_current_epoch = (now_ts - EPOCH_START_TS)/EPOCH_DURATION;

  if epoch != expected_current_epoch {
    return err!(AtaSkakingError::UnknownError)
  }

  let epoch_state_account = &mut ctx.accounts.epoch_state_account;

  if epoch_state_account.total_weighted_stake == 0 {
    return err!(AtaSkakingError::UnknownError);
  }

  epoch_state_account.total_weighted_stake -= weight*vault_account.staked_amount;

  print_epoch_state_account(epoch_state_account);

  let should_close_vault_token_account = {
    ctx.accounts.vault_nft_token_account.reload()?;
    ctx.accounts.vault_nft_token_account.amount == 0
  };

  if should_close_vault_token_account {
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::CloseAccount {
            account: ctx.accounts.vault_nft_token_account.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: vault_account.to_account_info()
        },
        &vault_signer
    );
    token::close_account(cpi_ctx)?;
  }

  let whitelist_nft_info_account = &mut ctx.accounts.whitelist_nft_info_account;
  whitelist_nft_info_account.is_staking = false;

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