use anchor_lang::prelude::*;

#[account]
pub struct PoolAccount {
    pub owner: Pubkey, //32
}

impl PoolAccount {
    pub const LEN: usize = 8 + 32;
}