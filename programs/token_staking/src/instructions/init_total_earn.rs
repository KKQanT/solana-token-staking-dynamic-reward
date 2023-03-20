use anchor_lang::prelude::*;
use crate::state::TotalEarn;
#[derive(Accounts)]
pub struct InitTotalEarn<'info> {
  #[account(
    init,
    seeds = [
      b"total_earn",
      user.key().as_ref(),
    ],
    bump,
    payer = user,
    space = TotalEarn::LEN
  )]
  pub total_earn_account: Account<'info, TotalEarn>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  _ctx: Context<InitTotalEarn>
) -> Result<()> {
  msg!("initialized total_earn_account to display total earning");
  Ok(())
}

