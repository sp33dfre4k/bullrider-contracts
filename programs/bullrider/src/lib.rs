use anchor_lang::prelude::*;

declare_id!("8oCRvRy6ScwN1JX9QvJsmS9UrcdqiGpcBTZLBDhX5vHb");

pub mod instructions;

use instructions::*;


#[program]
pub mod bullrider {
    use super::*;

    pub fn claim_interest(ctx: Context<ClaimInterest>) -> Result<()> {
        claim_interest::handler(ctx)
    }
}
