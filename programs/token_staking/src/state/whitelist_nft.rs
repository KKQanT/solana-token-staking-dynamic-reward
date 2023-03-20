use anchor_lang::prelude::*;

#[account]
pub struct  WhitelistNFTInfoAccount {
    pub mint_address: Pubkey, //32
    pub ata_value: u64, // 8
    pub is_staking: bool, // 1
}

impl WhitelistNFTInfoAccount {
    pub const LEN : usize = 8 + 32 + 8 +1;
}