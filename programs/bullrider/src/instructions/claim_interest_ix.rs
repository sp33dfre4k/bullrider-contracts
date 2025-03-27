use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    MintToChecked, TransferChecked, mint_to_checked, transfer_checked, TokenAccount, TokenInterface, Mint,
};

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

pub fn handler(ctx: Context<ClaimInterest>, interest_amount: u64) -> Result<()> {
    let mint_key = ctx.accounts.mint.key();
    let bump = ctx.bumps.withdraw_authority;

    let signer_seeds: &[&[u8]; 3] = &[
        b"withheld",
        mint_key.as_ref(),
        &[bump],
    ];

    let signer_seeds: &[&[&[u8]]; 1] = &[&signer_seeds[..]];

    let fee_pool_balance = ctx.accounts.fee_pool.amount;
    let (from_pool, to_mint) = if interest_amount <= fee_pool_balance {
        (interest_amount, 0)
    } else {
        (fee_pool_balance, interest_amount - fee_pool_balance)
    };

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
    }

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
    }

    Ok(())
}
