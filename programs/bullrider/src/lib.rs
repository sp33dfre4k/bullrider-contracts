use anchor_lang::prelude::*;

pub mod instructions;
use instructions::{WithdrawAndBurn, ClaimInterest};

declare_id!("8oCRvRy6ScwN1JX9QvJsmS9UrcdqiGpcBTZLBDhX5vHb");

#[program]
pub mod bullrider {
    use super::*;

    pub fn withdraw_and_burn(ctx: Context<WithdrawAndBurn>) -> Result<()> {
        instructions::withdraw_and_burn_ix::handler(ctx)
    }

    pub fn claim_interest(ctx: Context<ClaimInterest>, interest_amount: u64) -> Result<()> {
        instructions::claim_interest_ix::handler(ctx, interest_amount)
    }
}
