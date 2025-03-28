use anchor_lang::prelude::*;

declare_id!("8oCRvRy6ScwN1JX9QvJsmS9UrcdqiGpcBTZLBDhX5vHb");

pub mod instructions;

use instructions::*;


#[program]
pub mod bullrider {
    use super::*;

    pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
        instructions::collect_fees(ctx)
    }

    pub fn claim_interest(ctx: Context<ClaimInterest>) -> Result<()> {
        instructions::claim_interest(ctx)
    }
}
