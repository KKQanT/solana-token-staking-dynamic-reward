use crate::{PoolAccount, EpochStateAccount, VaultAccount};

use anchor_lang::prelude::*;

pub fn print_pool_account(pool_account: &Account<PoolAccount>) {
  msg!("stake pool created");
  msg!("owner = {}", pool_account.owner.to_string());
}

pub fn print_epoch_state_account(epoch_state_account: &Account<EpochStateAccount>) {
  msg!("epoch_state_account created");
  msg!("total_weighted_stake = {}", epoch_state_account.total_weighted_stake);
  msg!("total_reward_per_epoch = {}", epoch_state_account.total_reward_per_epoch);
}

pub fn print_vault_account(vault_account: &Account<VaultAccount>) {
  msg!("owner : {}", vault_account.owner);
  msg!("pool : {}", vault_account.pool);
  msg!("vault_id : {}", vault_account.vault_id);
  msg!("package_number : {}", vault_account.package_number);
  msg!("staked_amount : {}", vault_account.staked_amount);
  msg!("staked_time : {}", vault_account.staked_time);
  msg!("unlock_time : {}", vault_account.unlock_time);
  msg!("use_nft : {}", vault_account.use_nft);
}