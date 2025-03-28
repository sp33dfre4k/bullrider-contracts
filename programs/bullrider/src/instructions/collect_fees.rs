use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    TokenAccount, TokenInterface, Mint, WithdrawWithheldTokensFromMint,
};

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This is a PDA that we control
    #[account(
        mut,
        seeds = [b"withheld", mint.key().as_ref()],
        bump,
    )]
    pub withdraw_authority: UncheckedAccount<'info>,

    #[account(mut, constraint = fee_pool.mint == mint.key())]
    pub fee_pool: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
    msg!("Collecting withheld fees for mint: {}", ctx.accounts.mint.key());

    let mint_key = ctx.accounts.mint.key();
    let bump = ctx.bumps.withdraw_authority;
    let seeds_slice: &[&[u8]] = &[b"withheld", mint_key.as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds_slice];

    let withdraw_withheld_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        WithdrawWithheldTokensFromMint {
            mint: ctx.accounts.mint.to_account_info(),
            destination: ctx.accounts.fee_pool.to_account_info(),
            authority: ctx.accounts.withdraw_authority.to_account_info(),
            token_program_id: ctx.accounts.token_program.to_account_info(),
        },
        signer_seeds,
    );


    anchor_spl::token_interface::withdraw_withheld_tokens_from_mint(withdraw_withheld_ctx)?;
    msg!("Fees collected successfully into fee pool: {}", ctx.accounts.fee_pool.key());

    Ok(())
}

#[error_code]
pub enum CollectFeesError {
    #[msg("Invalid fee pool token account.")]
    InvalidFeePoolTokenAccount,
}
