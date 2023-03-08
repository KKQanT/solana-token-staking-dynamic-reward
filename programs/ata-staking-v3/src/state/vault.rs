use anchor_lang::prelude::*;

#[account]
pub struct VaultAccount {
  pub owner: Pubkey, //32
  pub pool: Pubkey, //32
  pub vault_id: Pubkey, //32
  pub package_number: u8, //1
  pub staked_amount: u64, //8
  pub staked_time: i64, //8
  pub unlock_time: i64, //8
  pub use_nft: bool, //1
}

impl VaultAccount {
  pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 8 + 8 + 8  + 1;
}