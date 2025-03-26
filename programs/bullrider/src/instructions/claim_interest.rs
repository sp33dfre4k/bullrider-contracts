use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to_checked, transfer_checked, TokenAccount, TokenInterface, Mint};

#[derive(Accounts)]
pub struct ClaimInterest<'info> {
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

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<CInterest>, interest_amount: u64) -> Result<()> {
    let signer_seeds = &[
        b"withheld",
        ctx.accounts.mint.key().as_ref(),
        &[ctx.bumps.withdraw_authority],
    ];

    let fee_pool_balance = ctx.accounts.fee_pool.amount;
    let (from_pool, to_mint) = if interest_amount <= fee_pool_balance {
        (interest_amount, 0)
    } else {
        (fee_pool_balance, interest_amount - fee_pool_balance)
    };

    if from_pool > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer {
                from: ctx.accounts.fee_pool.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.withdraw_authority.to_account_info(),
            },
            &[signer_seeds],
        );
        transfer_checked(transfer_ctx, from_pool, ctx.accounts.mint.decimals)?;
    }

    if to_mint > 0 {
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_to {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.withdraw_authority.to_account_info(),
            },
            &[signer_seeds],
        );
        mint_to_checked(mint_ctx, to_mint, ctx.accounts.mint.decimals)?;
    }

    Ok(())
}