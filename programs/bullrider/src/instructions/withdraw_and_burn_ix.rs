use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Burn, TokenAccount, TokenInterface, Mint, WithdrawWithheldTokensFromMint,
};

#[derive(Accounts)]
pub struct WithdrawAndBurn<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"withheld", mint.key().as_ref()],
        bump,
    )]
    pub withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_pool: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<WithdrawAndBurn>) -> Result<()> {
    let mint_key: Pubkey = ctx.accounts.mint.key();
    let bump: u8 = ctx.bumps.withdraw_authority;

    let signer_seeds: &[&[u8]; 3] = &[
        b"withheld",
        mint_key.as_ref(),
        &[bump],
    ];

    let signer_seeds: &[&[&[u8]]; 1] = &[&signer_seeds[..]];

    // Withdraw withheld tokens from mint to fee pool
    let cpi_ctx: CpiContext<'_, '_, '_, '_, WithdrawWithheldTokensFromMint<'_>> = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        WithdrawWithheldTokensFromMint {
            mint: ctx.accounts.mint.to_account_info(),
            destination: ctx.accounts.fee_pool.to_account_info(),
            authority: ctx.accounts.withdraw_authority.to_account_info(),
            token_program_id: ctx.accounts.token_program.to_account_info(),
        },
        signer_seeds,
    );
    anchor_spl::token_interface::withdraw_withheld_tokens_from_mint(cpi_ctx)?;

    // Burn the withdrawn tokens
    let amount: u64 = ctx.accounts.fee_pool.amount;
    let burn_ctx: CpiContext<'_, '_, '_, '_, Burn<'_>> = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.fee_pool.to_account_info(),
            authority: ctx.accounts.withdraw_authority.to_account_info(),
        },
        signer_seeds,
    );
    anchor_spl::token_interface::burn(burn_ctx, amount)?;

    Ok(())
}
