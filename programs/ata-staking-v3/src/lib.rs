use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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
        epoch: u64
    ) -> Result<()> {
        instructions::initialize_epoch_state_account::handler(
            ctx, total_reward_per_epoch, epoch
        )
    }

    pub fn stake(
        ctx: Context<Stake>, 
        pool_account_owner: Pubkey, 
        vault_id: Pubkey,
        current_epoch: i64,
        prev_epoch: i64,
        pool_bump: u8,
        epoch_bump: u8,
        prev_epoch_bump: u8,
        staked_amount: u64,
        package_number: u8,
    ) -> Result<()> {
        instructions::stake::handler(
            ctx, 
            pool_account_owner, 
            vault_id, 
            current_epoch, 
            prev_epoch, 
            pool_bump, 
            epoch_bump, 
            prev_epoch_bump, 
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
        epoch_bump: u8
    ) -> Result<()> {
        instructions::initialize_claim_state::handler(
            ctx, 
            pool_account_owner, 
            vault_id, 
            epoch, 
            pool_bump, 
            epoch_bump
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
        epoch_bump: u8
    ) -> Result<()> {
        instructions::claim_reward::handler(
            ctx, 
            vault_id, 
            pool_account_owner, 
            epoch, 
            vault_bump, 
            pool_bump,
            claim_state_bump, 
            epoch_bump
        )
    }

    
}

#[derive(Accounts)]
pub struct Initialize {}
