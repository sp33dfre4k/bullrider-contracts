#!/bin/bash

# Set environment to Solana Devnet
solana config set --url https://api.devnet.solana.com

# Set the Token-2022 Program ID
TOKEN_2022_PROGRAM_ID="TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"

# Create Token with Interest-Bearing (7% APR) and Transfer Fee (25%)
echo "Creating BULL token mint..."
BULL_MINT=$(spl-token --program-id $TOKEN_2022_PROGRAM_ID create-token \
    --interest-rate 7 \
    --transfer-fee-basis-points 2500 \
    --transfer-fee-maximum-fee 1000000 \
    --decimals 9 | grep "Address:" | awk '{print $2}')

echo "BULL Token Mint: $BULL_MINT"

# Create Associated Token Account for your wallet
echo "Creating Associated Token Account for your wallet..."
BULL_TOKEN_ACCOUNT=$(spl-token --program-id $TOKEN_2022_PROGRAM_ID create-account $BULL_MINT | grep "Address:" | awk '{print $2}')

echo "Token Account Created: $BULL_TOKEN_ACCOUNT"

# Mint initial supply to your token account (1 billion BULL)
INITIAL_SUPPLY=1_000_000_000
echo "Minting initial supply ($INITIAL_SUPPLY BULL) to your account..."
spl-token --program-id $TOKEN_2022_PROGRAM_ID mint $BULL_MINT $INITIAL_SUPPLY

echo "🎉 BULL Token deployed successfully!"
echo "Mint Address: $BULL_MINT"
echo "Your Token Account: $BULL_TOKEN_ACCOUNT"
