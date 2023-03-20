use crate::{PoolAccount, EpochStateAccount, VaultAccount};
use crate::constant::{
  VESTING_DURATION_2, 
  VESTING_DURATION_3, 
  VESTING_DURATION_4,
  LOCK_DURTION_1,
  LOCK_DURTION_2,
  LOCK_DURTION_3,
  LOCK_DURTION_4
};
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
  msg!("vesting_end_time : {}", vault_account.vesting_end_time);
  msg!("use_nft : {}", vault_account.use_nft);
}

pub fn get_ten_time_weight(&package_number: &u8) -> u64 {
  let weight: u64 = if package_number == 1 {
    10
  } else if package_number == 2 {
    12
  } else if package_number == 3 {
    30
  } else if package_number == 4 {
    50
  } else {
    10
  };
  weight
}

pub fn get_lock_duration(&package_number: &u8) -> i64 {
  let lock_duration: i64 = if package_number == 1 {
    LOCK_DURTION_1 
  } else if package_number == 2 {
    LOCK_DURTION_2 
  } else if package_number == 3 {
    LOCK_DURTION_3 
  } else {
    LOCK_DURTION_4
  };
  lock_duration
}

pub fn get_vesting_duration(&package_number: &u8) -> i64 {
  let vesting_duration: i64 = if package_number == 1 {
    0
  } else if package_number == 2 {
    VESTING_DURATION_2
  } else if package_number == 3 {
    VESTING_DURATION_3
  } else {
    VESTING_DURATION_4
  };
  vesting_duration
}