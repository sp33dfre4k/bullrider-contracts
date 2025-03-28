use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    MintToChecked, TransferChecked, mint_to_checked, transfer_checked, TokenAccount, TokenInterface, Mint,
    interest_bearing_mint
};

#[derive(Accounts)]
pub struct ClaimInterest<'info> {
    /// The mint for our token.
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// The PDA authority for fee withdrawals (must match the seeds used in token deployment).
    #[account(
        mut,
        seeds = [b"withheld", mint.key().as_ref()],
        bump,
    )]
    pub withdraw_authority: UncheckedAccount<'info>,

    /// The fee pool account that holds collected fees.
    #[account(mut)]
    pub fee_pool: InterfaceAccount<'info, TokenAccount>,

    /// The user's token account.
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    /// The token program.
    pub token_program: Interface<'info, TokenInterface>,
}

/// Helper to compute the effective (UI) balance for a token account, including accrued interest,
/// using the interest-bearing mint extension's helper `amount_to_ui_amount`.
fn compute_effective_balance(token_account: &TokenAccount, mint: &Mint) -> Result<u64> {
    let clock = Clock::get()?;
    interest_bearing_mint::amount_to_ui_amount(token_account.amount, mint, clock.unix_timestamp)
        .map_err(|_| error!(ClaimInterestError::InterestCalculationFailed))
}

/// Processes a user's claim for accrued interest.
///
/// The claimable interest is computed as:
///   effective_balance (including interest) - raw_balance
///
/// In the example, if a user bought 100 tokens and their effective balance becomes 110 tokens,
/// then 10 tokens of interest are claimable. The fee pool is used to pay for interest first,
/// and if insufficient, new tokens are minted.
pub fn handler(ctx: Context<ClaimInterest>) -> Result<()> {
    // Compute the effective (UI) balance using the interest-bearing helper.
    let effective_balance = compute_effective_balance(&ctx.accounts.user_token_account, &ctx.accounts.mint)?;
    msg!("Effective balance (including accrued interest): {}", effective_balance);
    msg!("Stored raw balance: {}", ctx.accounts.user_token_account.amount);

    // Calculate claimable interest as the difference between effective balance and raw balance.
    let claimable_interest = effective_balance
        .checked_sub(ctx.accounts.user_token_account.amount)
        .ok_or(ClaimInterestError::InvalidInterestCalculation)?;
    msg!("Claimable interest: {}", claimable_interest);

    // If there's no interest accrued, exit early.
    if claimable_interest == 0 {
        msg!("No claimable interest available.");
        return Ok(());
    }

    // Determine how much of the claimable interest to cover from the fee pool versus minting new tokens.
    let fee_pool_balance = ctx.accounts.fee_pool.amount;
    let (from_pool, to_mint) = if claimable_interest <= fee_pool_balance {
        (claimable_interest, 0)
    } else {
        (fee_pool_balance, claimable_interest - fee_pool_balance)
    };

    // Prepare signer seeds for CPI calls.
    let mint_key = ctx.accounts.mint.key();
    let bump = ctx.bumps.withdraw_authority;
    let seeds_slice: &[&[u8]] = &[b"withheld", mint_key.as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds_slice];

    // Transfer from fee pool if available.
    if from_pool > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.fee_pool.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.withdraw_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            signer_seeds,
        );
        transfer_checked(transfer_ctx, from_pool, ctx.accounts.mint.decimals)?;
        msg!("Transferred {} tokens from fee pool.", from_pool);
    }

    // Mint new tokens for any remaining interest.
    if to_mint > 0 {
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintToChecked {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.withdraw_authority.to_account_info(),
            },
            signer_seeds,
        );
        mint_to_checked(mint_ctx, to_mint, ctx.accounts.mint.decimals)?;
        msg!("Minted {} new tokens.", to_mint);
    }

    Ok(())
}

#[error_code]
pub enum ClaimInterestError {
    #[msg("Invalid interest calculation.")]
    InvalidInterestCalculation,
    #[msg("Failed to compute effective balance.")]
    InterestCalculationFailed,
}
