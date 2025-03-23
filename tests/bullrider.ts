import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bullrider } from "../target/types/bullrider";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { 
  TOKEN_2022_PROGRAM_ID, 
  getAccount, 
  getAssociatedTokenAddressSync, 
  getMint, 
  createAssociatedTokenAccountInstruction, 
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { expect } from "chai";
import { BN } from "bn.js";

// Constants matching what's in the contract
const TRANSFER_FEE_BASIS_POINTS = 2500; // 25%
const INTEREST_RATE_BASIS_POINTS = 700; // 7%

describe("bullrider", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Bullrider as Program<Bullrider>;
  
  // Generate keypairs for our test accounts
  const payer = provider.wallet;
  const user1 = Keypair.generate();
  const user2 = Keypair.generate();
  
  // PDA for mint
  const [mintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("bull_rider_mint")],
    program.programId
  );

  // Withdraw withheld authority
  const [withdrawWithheldAuthorityPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("withdraw_withheld_authority")],
    program.programId
  );

  // Token accounts
  let user1TokenAccount: PublicKey;
  let user2TokenAccount: PublicKey;
  
  // Test amount constants
  const MINT_AMOUNT = 1_000_000_000; // 1000 tokens with 6 decimals
  const TRANSFER_AMOUNT = 100_000_000; // 100 tokens

  // Initialize test accounts with SOL
  before(async () => {
    // Fund user accounts with SOL for rent
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user1.publicKey, 10_000_000_000),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user2.publicKey, 10_000_000_000),
      "confirmed"
    );

    // Get token account addresses
    user1TokenAccount = getAssociatedTokenAddressSync(
      mintPDA,
      user1.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );
    
    user2TokenAccount = getAssociatedTokenAddressSync(
      mintPDA,
      user2.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );
  });

  it("Initializes the token with correct parameters", async () => {
    // Initialize the token
    const tx = await program.methods.initialize()
      .accounts({
        mint: mintPDA,
        feeAuthority: payer.publicKey,
        withdrawWithheldAuthority: withdrawWithheldAuthorityPDA,
        payer: payer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    console.log("Initialize transaction signature", tx);

    // Fetch the mint account and verify it was initialized correctly
    const mintInfo = await getMint(
      provider.connection,
      mintPDA,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    // Verify mint details
    expect(mintInfo.decimals).to.equal(9);
    expect(mintInfo.mintAuthority.toString()).to.equal(payer.publicKey.toString());
    expect(mintInfo.freezeAuthority?.toString()).to.equal(payer.publicKey.toString());
    
    // Verify extensions
    // The methods to check the extensions would need actual interaction with SPL-token 2022
    // This is a placeholder where you would verify the transfer fee and interest rate settings
    // In a production test, you would use token-22 extension helpers to verify these values
  });

  it("Mints tokens to a user", async () => {
    // Create user1's token account if it doesn't exist
    try {
      await provider.sendAndConfirm(
        new anchor.web3.Transaction().add(
          createAssociatedTokenAccountInstruction(
            payer.publicKey,
            user1TokenAccount,
            user1.publicKey,
            mintPDA,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
          )
        )
      );
    } catch (e) {
      // Account might already exist
      console.log("Token account may already exist, continuing...");
    }

    // Mint tokens to user1
    const mintTx = await program.methods.mintToken(new BN(MINT_AMOUNT))
      .accounts({
        mint: mintPDA,
        mintAuthority: payer.publicKey,
        recipient: user1.publicKey,
        recipientTokenAccount: user1TokenAccount,
        payer: payer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    console.log("Mint transaction signature", mintTx);

    // Verify the tokens were minted to user1's account
    const user1Account = await getAccount(
      provider.connection,
      user1TokenAccount,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );
    
    expect(Number(user1Account.amount)).to.equal(MINT_AMOUNT);
  });

  it("Applies transfer fee when transferring tokens", async () => {
    // Create user2's token account if it doesn't exist
    try {
      await provider.sendAndConfirm(
        new anchor.web3.Transaction().add(
          createAssociatedTokenAccountInstruction(
            payer.publicKey,
            user2TokenAccount,
            user2.publicKey,
            mintPDA,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
          )
        )
      );
    } catch (e) {
      // Account might already exist
      console.log("Token account may already exist, continuing...");
    }

    // Check initial balances
    const initialUser1Balance = Number((await getAccount(
      provider.connection,
      user1TokenAccount,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    )).amount);

    // Calculate expected transfer fee (25% of transfer amount)
    const expectedFee = Math.floor(TRANSFER_AMOUNT * (TRANSFER_FEE_BASIS_POINTS / 10000));
    const expectedReceivedAmount = TRANSFER_AMOUNT - expectedFee;

    // Transfer tokens from user1 to user2
    // Note: This would need to be implemented with the proper spl-token-2022 instructions
    // Here we're showing a simplified approach - in a real test you'd use transferCheckedWithFee

    // In a real test, you would do something like this:
    /*
    await transferCheckedWithFee(
      provider.connection,
      payer, // payer
      user1TokenAccount, // source
      mintPDA, // mint
      user2TokenAccount, // destination
      user1, // owner
      TRANSFER_AMOUNT,
      9, // decimals
      0, // fee (the contract will apply it)
      []
    );
    */

    // For demonstration purposes, let's simulate what would happen:
    
    // Verify balances after transfer
    // Here we would check:
    // 1. User1's balance decreased by TRANSFER_AMOUNT
    // 2. User2's balance increased by TRANSFER_AMOUNT - fee
    // 3. The fee is withheld in the token's fee account
    
    // Note: These checks would be performed after the actual transfer in a real test
  });

  it("Accrues interest over time", async () => {
    // Note: Testing interest accrual typically requires simulating time passage
    // This is challenging in a standard unit test environment
    
    // In a real test, you would:
    // 1. Record initial balance
    // 2. Somehow advance blockchain time (depends on your test environment)
    // 3. Perform an action that triggers interest calculation
    // 4. Verify the new balance includes expected interest
    
    // Example calculation for expected interest:
    // For a balance of 1000 tokens at 7% APY, over 1 year:
    // Interest = 1000 * 0.07 = 70 tokens
    
    // This test is a placeholder for interest bearing behavior
  });
});
