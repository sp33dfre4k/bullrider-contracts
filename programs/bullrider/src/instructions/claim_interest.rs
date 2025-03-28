use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    MintToChecked, TransferChecked, mint_to_checked, transfer_checked, TokenAccount, TokenInterface, Mint,
};
use anchor_spl::token_2022::{amount_to_ui_amount, AmountToUiAmount};

#[derive(Accounts)]
pub struct ClaimInterest<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This is a PDA that we control
    #[account(
        mut,
        seeds = [b"withheld", mint.key().as_ref()],
        bump,
    )]
    pub withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_pool: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Helper to compute the effective (UI) balance for a token account, including accrued interest
fn calculate_effective_balance(ctx: &Context<ClaimInterest>) -> Result<u64> {
    let amount_to_ui_amount_ctx: CpiContext<'_, '_, '_, '_, AmountToUiAmount<'_>> = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        AmountToUiAmount {
            account: ctx.accounts.user_token_account.to_account_info(),
        },
    );
    
    // Convert the returned string to u64
    let ui_amount: String = amount_to_ui_amount(
        amount_to_ui_amount_ctx, 
        ctx.accounts.user_token_account.amount
    ).map_err(|_| error!(ClaimInterestError::BalanceCalculationFailed))?;
    
    // Parse the UI amount string back to f64, then convert to u64 based on decimals
    let float_amount: f64 = ui_amount
        .parse::<f64>()
        .map_err(|_| error!(ClaimInterestError::BalanceCalculationFailed))?;
    
    // Convert to raw amount
    let raw_amount: u64 = (float_amount * 10f64.powi(ctx.accounts.mint.decimals as i32)) as u64;
    
    Ok(raw_amount)
}

pub fn handler(ctx: Context<ClaimInterest>) -> Result<()> {

    let effective_balance: u64 = calculate_effective_balance(&ctx)?;
    msg!("Effective balance (including accrued interest): {}", effective_balance);
    msg!("Stored raw balance: {}", ctx.accounts.user_token_account.amount);

    let claimable_interest: u64 = effective_balance
        .checked_sub(ctx.accounts.user_token_account.amount)
        .ok_or(ClaimInterestError::InvalidInterestCalculation)?;
    msg!("Claimable interest: {}", claimable_interest);

    // If there's no interest accrued, exit early.
    if claimable_interest == 0 {
        msg!("No claimable interest available.");
        return Ok(());
    }

    let fee_pool_balance = ctx.accounts.fee_pool.amount;
    let (from_pool, to_mint) = if claimable_interest <= fee_pool_balance {
        (claimable_interest, 0)
    } else {
        (fee_pool_balance, claimable_interest - fee_pool_balance)
    };

    let mint_key = ctx.accounts.mint.key();
    let bump = ctx.bumps.withdraw_authority;
    let seeds_slice: &[&[u8]] = &[b"withheld", mint_key.as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds_slice];

    // Transfer from fee pool if available.
    if from_pool > 0 {
        let transfer_ctx: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new_with_signer(
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
        let mint_ctx: CpiContext<'_, '_, '_, '_, MintToChecked<'_>> = CpiContext::new_with_signer(
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
    BalanceCalculationFailed,
}
