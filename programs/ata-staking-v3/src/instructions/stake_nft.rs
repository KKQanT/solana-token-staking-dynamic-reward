use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use crate::state::{
  VaultAccount, 
  PoolAccount, 
  WhitelistNFTInfoAccount,
  EpochStateAccount
};
use crate::errors::AtaSkakingError;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use crate::constant::{METADATA_PROGRAM_ID, EPOCH_DURATION, EPOCH_START_TS};
use crate::utils::{
  print_epoch_state_account, 
  print_vault_account
};

#[derive(Accounts)]
#[instruction(
  vault_id: Pubkey,
  pool_account_owner: Pubkey,
  current_epoch: i64,
  mint_address: Pubkey,
  pool_bump: u8,
  epoch_bump: u8,
  whitelist_nft_bump: u8
)]

pub struct StakeNFT<'info> {
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
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(mut)]
  pub vault_nft_token_account: Account<'info, token::TokenAccount>, //Check in handler
  #[account(mut)]
  pub user_nft_token_account: Account<'info, token::TokenAccount>,
  ///CHECK: checked via instruction
  pub metadata_account: AccountInfo<'info>,
  ///CHECK : check via #[account(address = crate::address::METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
  #[account(address = METADATA_PROGRAM_ID.parse::<Pubkey>().unwrap())]
  pub token_metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
}

pub fn handler(
  ctx: Context<StakeNFT>,
  vault_id: Pubkey,
  _pool_account_owner: Pubkey,
  current_epoch: i64,
  mint_address: Pubkey,
  _pool_bump: u8,
  _epoch_bump: u8,
  _whitelist_nft_bump: u8,
  package_number: u8
) -> Result<()> {
  if package_number < 1 && package_number > 4 {
    return  err!(AtaSkakingError::UnknownError);
  }

  let user_nft_token_account = &ctx.accounts.user_nft_token_account;
  let user = &ctx.accounts.user;

  if user_nft_token_account.owner != user.key() {
    return  err!(AtaSkakingError::UnknownError);
  }

  if user_nft_token_account.mint != mint_address {
    return  err!(AtaSkakingError::UnknownError);
  }

  if user_nft_token_account.amount != 1 {
    return  err!(AtaSkakingError::UnknownError);
  }

  let nft_metadata_account = &ctx.accounts.metadata_account;

  if nft_metadata_account.owner.key() != ctx.accounts.token_metadata_program.key() {
    return err!(AtaSkakingError::UnknownError)
  };

  let metadata_seed = &[
    b"metadata",
    ctx.accounts.token_metadata_program.key.as_ref(),
    user_nft_token_account.mint.as_ref(),
  ];

  let (expected_metadata_key, _metadata_bump) = Pubkey::find_program_address(
    metadata_seed, 
    ctx.accounts.token_metadata_program.key
  );

  if nft_metadata_account.key() != expected_metadata_key {
    return err!(AtaSkakingError::UnknownError);
  }

  if nft_metadata_account.data_is_empty() {
    return  err!(AtaSkakingError::UnknownError);
  }

  let nft_metadata: Metadata = Metadata::from_account_info(&nft_metadata_account)?;
  let nft_first_creator = &nft_metadata.data.creators.unwrap()[0];
  
  if !nft_first_creator.verified {
    return  err!(AtaSkakingError::UnknownError);
  }

  if nft_first_creator.address.to_string() != crate::constant::EXPECTED_NFT_CREATOR_ADDRESS {
    return  err!(AtaSkakingError::UnknownError);
  }
  
  let whitelist_nft_info_account = &mut ctx.accounts.whitelist_nft_info_account;
  
  if whitelist_nft_info_account.is_staking {
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

  let staked_amount = whitelist_nft_info_account.ata_value;

  let vault_account = &mut ctx.accounts.vault_account;
  vault_account.owner = ctx.accounts.user.key();
  vault_account.pool = ctx.accounts.pool_account.key();
  vault_account.vault_id = vault_id;
  vault_account.package_number = package_number;
  vault_account.staked_amount = staked_amount;
  vault_account.staked_time = staked_time;
  vault_account.unlock_time = staked_time + lock_duration;
  vault_account.use_nft = true;

  let expected_vault_token_account = associated_token::get_associated_token_address(
    &vault_account.key(), 
    &mint_address
  );

  if ctx.accounts.vault_nft_token_account.key() != expected_vault_token_account {
    return err!(AtaSkakingError::UnknownError);
  }

  let cpi_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    token::Transfer {
        from: user_nft_token_account.to_account_info(),
        to: ctx.accounts.vault_nft_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    },
  ) ;
  token::transfer(cpi_ctx, 1)?;

  msg!("nft transfered");

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