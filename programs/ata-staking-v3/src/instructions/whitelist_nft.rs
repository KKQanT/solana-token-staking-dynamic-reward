use anchor_lang::prelude::*;
use crate::state::WhitelistNFTInfoAccount;

#[derive(Accounts)]
#[instruction(
  mint_address: Pubkey,
  ata_value: u64, 
)]
pub struct WhitelistNFT<'info> {
  #[account(
    init,
    seeds=[
      b"whitelist_nft", 
      pool_owner_account.key().as_ref(), 
      mint_address.as_ref()
      ],
    bump,
    payer = pool_owner_account,
    space = WhitelistNFTInfoAccount::LEN,
  )]
  pub whitelist_nft_info_account : Account<'info, WhitelistNFTInfoAccount>,
  #[account(mut)]
  pub pool_owner_account: Signer<'info>,
  pub system_program: Program<'info, System>
}

pub fn handler(
  ctx: Context<WhitelistNFT>,
  mint_address: Pubkey,
  ata_value: u64
) -> Result<()> {
  let whitelist_nft_info_account = &mut ctx.accounts.whitelist_nft_info_account;
  whitelist_nft_info_account.mint_address = mint_address;
  whitelist_nft_info_account.ata_value = ata_value;
  whitelist_nft_info_account.is_staking = false;

  msg!("whitelist nft : {} ", whitelist_nft_info_account.mint_address);
  msg!("ata_value : {} ", whitelist_nft_info_account.ata_value);
  msg!("is_staking : {} ", whitelist_nft_info_account.is_staking);

  Ok(())

}