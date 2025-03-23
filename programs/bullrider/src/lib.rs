use anchor_lang::prelude::*;

mod constants;
mod instructions;
mod state;

declare_id!("8oCRvRy6ScwN1JX9QvJsmS9UrcdqiGpcBTZLBDhX5vHb");

#[program]
pub mod bullrider {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }
    
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        instructions::mint_token::handler(ctx, amount)
    }
}

// Re-export account structures
pub use state::*;