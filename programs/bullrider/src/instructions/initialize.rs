use anchor_lang::prelude::*;
use anchor_spl::token_2022::{
    InitializeMint2, initialize_mint2,
};
use anchor_spl::token_2022_extensions::{
    interest_bearing_mint::{
        interest_bearing_mint_initialize,
        InterestBearingMintInitialize,
    },
    transfer_fee::{
        transfer_fee_initialize,
        TransferFeeInitialize,
    },
};
use crate::constants::*;
use crate::state::Initialize;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Initializing Bull Rider (BULL)");
    
    // Get required accounts
    let mint = &ctx.accounts.mint;
    let payer = &ctx.accounts.payer;
    
    // Initialize the mint with 9 decimals
    initialize_mint2(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint2 {
                mint: mint.to_account_info(),
            },
        ),
        9, // 9 decimals
        &payer.key(),
        Some(&payer.key()),
    )?;
    
    // Initialize transfer fee configuration
    transfer_fee_initialize(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferFeeInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: mint.to_account_info(),
            },
        ),
        Some(&ctx.accounts.fee_authority.key()),
        Some(&ctx.accounts.withdraw_withheld_authority.key()),
        TRANSFER_FEE_BASIS_POINTS,
        MAXIMUM_FEE,
    )?;
    
    // Initialize interest rate configuration
    interest_bearing_mint_initialize(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InterestBearingMintInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: mint.to_account_info(),
            },
        ),
        Some(payer.key()),
        INTEREST_RATE_BASIS_POINTS as i16,
    )?;
    
    msg!("Bull Rider token initialized with 25% transfer fee and 7% interest rate");
    
    Ok(())
} 