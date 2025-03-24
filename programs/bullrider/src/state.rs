use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
    token_2022::Token2022,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The token mint
    #[account(
        init,
        payer = payer,
        seeds = [b"bull_rider_mint"],
        bump,
        mint::decimals = 6,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    
    /// Fee authority - can change the fee amount
    #[account(mut)]
    pub fee_authority: Signer<'info>,
    
    /// Withdraw withheld authority - can withdraw withheld fees
    #[account(
        seeds = [b"withdraw_withheld_authority"],
        bump
    )]
    pub withdraw_withheld_authority: SystemAccount<'info>,
    
    /// Payer for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// Token mint
    pub mint: Account<'info, Mint>,
    
    /// Mint authority
    pub mint_authority: Signer<'info>,
    
    /// Account that will receive the minted tokens
    pub recipient: SystemAccount<'info>,
    
    /// The associated token account for the recipient. Will be initialized if it doesn't exist.
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient,
        associated_token::token_program = token_program,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    /// Payer for creating the token account (if needed)
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}