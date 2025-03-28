#!/bin/bash
set -e

# point to devnet
export SOLANA_CLUSTER="devnet"

TOKEN_NAME="Test Token 2022"
TOKEN_SYMBOL="TEST"
TOKEN_URI="https://raw.githubusercontent.com/sp33dfre4k/bullrider-contracts/refs/heads/trunk/metadata/metadata.json"
TRANSFER_FEE_BASIS_POINTS=2500
TRANSFER_FEE_MAXIMUM_FEE=500000000
INTEREST_RATE=3393000 # 1% daily yield
TOTAL_SUPPLY=1000000000

# check if wallet exists
if [ ! -f "wallet.json" ]; then
    echo "wallet.json not found. please generate a wallet first with solana-keygen"
    exit 1
fi

WALLET_PUBKEY=$(solana-keygen pubkey wallet.json)
echo -e "Using wallet: $WALLET_PUBKEY\n"

# configure solana to use our wallet
solana config set --keypair wallet.json

echo -e "\nDeploying $TOKEN_NAME token on Devnet...\n"

# Add debug line to see raw output
echo -e "Creating token..."
TOKEN_ADDRESS=$(spl-token create-token --program-2022 \
  --enable-metadata \
  --transfer-fee-basis-points $TRANSFER_FEE_BASIS_POINTS \
  --transfer-fee-maximum-fee $TRANSFER_FEE_MAXIMUM_FEE \
  --interest-rate $INTEREST_RATE \
  --mint-authority $WALLET_PUBKEY \
  --output json | jq -r '.commandOutput.address')

echo -e "Token created: $TOKEN_ADDRESS\n"

echo -e "Initializing metadata..."
spl-token initialize-metadata $TOKEN_ADDRESS "$TOKEN_NAME" "$TOKEN_SYMBOL" "$TOKEN_URI"

echo -e "Authorizing withheld withdraw authority..."
spl-token authorize --program-2022 $TOKEN_ADDRESS withheld-withdraw $WALLET_PUBKEY

echo -e "Creating associated token account..."
spl-token create-account $TOKEN_ADDRESS


echo -e "Minting supply...\n"
# Mint supply
spl-token mint $TOKEN_ADDRESS $TOTAL_SUPPLY

echo -e "Devnet deployment complete."
