use anchor_lang::prelude::*;

declare_id!("8oCRvRy6ScwN1JX9QvJsmS9UrcdqiGpcBTZLBDhX5vHb");

#[program]
pub mod bullrider {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
