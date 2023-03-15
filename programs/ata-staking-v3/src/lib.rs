use anchor_lang::prelude::*;

declare_id!("AtA3riahYEopN59jQu6gTL8Dix7XKunY2PBH3jnRv55u");

pub mod instructions;
pub mod state;
pub mod constant;
pub mod utils;
pub mod errors;

pub use instructions::*;
pub use state::*;
pub use constant::*;
pub use utils::*;
pub use errors::*;

#[program]
pub mod ata_staking_v3 {
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {
        instructions::initialize_stake_pool::handler(ctx)
    }

    pub fn initialize_epoch_state_account(
        ctx: Context<InitializeEpochStateAccount>,
        total_reward_per_epoch: u64,
        epoch: i64,
    ) -> Result<()> {
        instructions::initialize_epoch_state_account::handler(
            ctx, total_reward_per_epoch, epoch
        )
    }

    pub fn update_epoch(
        ctx: Context<UpdateEpoch>,
        target_epoch: i64, 
        prev_epoch: i64,
        pool_owner: Pubkey,
        target_epoch_bump: u8,
        prev_epoch_bump: u8
    ) -> Result<()> {
        instructions::update_epoch::handler(
            ctx, 
            target_epoch, 
            prev_epoch, 
            pool_owner, 
            target_epoch_bump, 
            prev_epoch_bump
        )
    }

    pub fn authority_update_epoch(
        ctx: Context<AuthorityUpdateEpoch>,
        total_weighted_stake: u64,
        total_reward_per_epoch: u64,
        epoch: i64,
        epoch_bump: u8,
    ) -> Result<()> {
        instructions::authority_update_epoch::handler(
            ctx, 
            total_weighted_stake, 
            total_reward_per_epoch, 
            epoch, 
            epoch_bump
        )
    }

    pub fn stake(
        ctx: Context<Stake>, 
        pool_account_owner: Pubkey, 
        vault_id: Pubkey,
        current_epoch: i64,
        pool_bump: u8,
        epoch_bump: u8,
        staked_amount: u64,
        package_number: u8,
    ) -> Result<()> {
        instructions::stake::handler(
            ctx, 
            pool_account_owner, 
            vault_id, 
            current_epoch, 
            pool_bump, 
            epoch_bump, 
            staked_amount, 
            package_number
        )
    }

    pub fn initialize_claim_state(
        ctx: Context<InitClaimReward>,
        pool_account_owner: Pubkey,
        vault_id: Pubkey,
        epoch: i64,
        pool_bump: u8,
        epoch_bump: u8,
        vault_bump: u8
    ) -> Result<()> {
        instructions::initialize_claim_state::handler(
            ctx, 
            pool_account_owner, 
            vault_id, 
            epoch, 
            pool_bump, 
            epoch_bump,
            vault_bump
        )
    }

    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        vault_id: Pubkey,
        pool_account_owner: Pubkey,
        epoch: i64,
        vault_bump: u8,
        pool_bump:u8,
        claim_state_bump: u8,
        epoch_bump: u8,
        total_earn_bump: u8
    ) -> Result<()> {
        instructions::claim_reward::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            epoch, 
            vault_bump, 
            pool_bump,
            claim_state_bump, 
            epoch_bump,
            total_earn_bump
        )
    }

    pub fn unstake(
        ctx: Context<Unstake>,
        vault_id: Pubkey,
        pool_account_owner: Pubkey,
        epoch: i64,
        vault_bump: u8,
        pool_bump: u8,
        epoch_bump: u8,
    ) -> Result<()> {
        instructions::unstake::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            epoch, 
            vault_bump, 
            pool_bump, 
            epoch_bump
        )
    }

    pub fn whitelist_nft(
        ctx: Context<WhitelistNFT>,
        mint_address: Pubkey,
        ata_value: u64 
    ) -> Result<()> {
        instructions::whitelist_nft::handler(
            ctx, mint_address, ata_value
        )
    }

    pub fn stake_nft(
        ctx: Context<StakeNFT>,
        vault_id: Pubkey,
        pool_account_owner: Pubkey,
        current_epoch: i64,
        mint_address: Pubkey,
        pool_bump: u8,
        epoch_bump: u8,
        whitelist_nft_bump: u8,
        package_number: u8
    ) -> Result<()> {
        instructions::stake_nft::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            current_epoch, 
            mint_address, 
            pool_bump, 
            epoch_bump, 
            whitelist_nft_bump, 
            package_number
        )
    }

    pub fn unstake_nft(
        ctx: Context<UnstakeNFT>,
        vault_id: Pubkey,
        pool_account_owner: Pubkey,
        mint_address: Pubkey,
        epoch: i64,
        vault_bump: u8,
        pool_bump: u8,
        whitelist_nft_bump: u8,
        epoch_bump: u8,
    ) -> Result<()> {
        instructions::unstake_nft::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            mint_address, 
            epoch, 
            vault_bump, 
            pool_bump, 
            whitelist_nft_bump, 
            epoch_bump
        )
    }

    pub fn init_unstake(
        ctx: Context<InitUnstake>,
        vault_id: Pubkey,
        pool_account_owner: Pubkey,
        vault_bump: u8,
        pool_bump: u8,
    ) -> Result<()> {
        instructions::init_unstake::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            vault_bump, 
            pool_bump
        )
    }

    pub fn close_claim_state(
        ctx: Context<CloseClaimReward>,
        vault_id: Pubkey,
        pool_account_owner: Pubkey,
        epoch: i64,
        vault_bump: u8,
        pool_bump: u8,
        claim_state_bump: u8,
    ) -> Result<()> {
        instructions::close_claim_state::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            epoch, 
            vault_bump, 
            pool_bump, 
            claim_state_bump, 
        )
    }

    pub fn init_total_earn(
        ctx: Context<InitTotalEarn>
    ) -> Result<()> {
        instructions::init_total_earn::handler(ctx)
    }

    
}

#[derive(Accounts)]
pub struct Initialize {}
