use anchor_lang::prelude::*;
use anchor_spl::token_2022::{
    MintTo, mint_to,
};
use crate::state::MintToken;

pub fn handler(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    // Mint tokens to the recipient
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        amount,
    )?;
    
    msg!("Minted {} BULL tokens to {}", amount, ctx.accounts.recipient.key());
    
    Ok(())
} 