import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import {
  Keypair,
  PublicKey,
} from "@solana/web3.js";
import { expect } from "chai";
import {
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const user = provider.wallet.payer;
  const connection = provider.connection;

  const amount = new anchor.BN(1_000_000_000);

  let mint: PublicKey;
  let userTokenAccount: PublicKey;
  let chrono: PublicKey;
  let vault: PublicKey;

  before("Create new token mint and mint tokens", async () => {
    // create new mint
    mint = await createMint(
      connection,             // connection
      user,                   // fee payer
      user.publicKey,         // mint authority
      null,                   // freeze authority (none)
      9                       // decimals
    );
    console.log("Mint address:", mint.toBase58());

    // create ATA for user
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint,
      user.publicKey
    );
    userTokenAccount = ata.address;
    console.log("User ATA:", userTokenAccount.toBase58());

    // mint tokens to user ATA
    await mintTo(
      connection,
      user,
      mint,
      userTokenAccount,
      user, // authority
      5_000_000_000
    );

    const accountInfo = await getAccount(connection, userTokenAccount);
    console.log("User token balance:", accountInfo.amount.toString());
    expect(Number(accountInfo.amount)).to.eq(5_000_000_000);

    // derive chrono PDA
    [chrono] = PublicKey.findProgramAddressSync(
      [Buffer.from("chrono"), user.publicKey.toBuffer()],
      program.programId
    );

    // derive vault ATA owned by chrono PDA
    vault = getAssociatedTokenAddressSync(mint, chrono, true);
    console.log("Vault ATA:", vault.toBase58());
  });

  
  it("Initialized vault", async () => {
    const lockDuration = new anchor.BN(2);
    
    const tx = await program.methods
    .initialize(lockDuration, mint)
    .accounts({
        user: user.publicKey,
        mint: mint,
        userAta: userTokenAccount,
        chronoAccount: chrono,
        vault: vault,
      })
      .rpc();
      
      console.log("Transaction signature (initialize): ", tx);
    });
    
    it("Should fail due to invalid time duration", async () => {
      const lockDuration = new anchor.BN(-1);
  
      try {
        const tx = await program.methods
        .initialize(lockDuration, mint)
        .accounts({
          user: user.publicKey,
          mint: mint,
          userAta: userTokenAccount,
          chronoAccount: chrono,
          vault: vault,
        })
        .rpc();
        
        console.log("Transaction signature (initialize): ", tx);
        expect.fail("Should fail when invalid lock duration given");
      } catch (error) {

      }
    });

    it("Transfer amount to vault", async () => {
      const tx = await program.methods
      .deposit(amount)
      .accounts({
        user: user.publicKey,
        mint: mint,
        userAta: userTokenAccount,
        chronoAccount: chrono,
        vault: vault,
      })
      .rpc();

    const vaultAccount = await getAccount(connection, vault);
    console.log("Vault balance:", vaultAccount.amount.toString());
    console.log("Transaction signature (deposit): ", tx);

    expect(Number(vaultAccount.amount)).to.eq(1_000_000_000);
  });

  it("Cannot withdraw when vault is locked", async() => {
    try {
      const tx = await program.methods
        .withdraw(amount)
        .accounts({
          user: user.publicKey,
          mint: mint,
          userAta: userTokenAccount,
          chronoAccount: chrono,
          vault: vault,
      })
      .rpc();
      expect.fail("Should fail cause vault is locked yet");
    } catch (error) {
      console.log("Some thing went wrong")
    }
  })

  it("Transfer amount to vault after vault unlocks", async () => {

    console.log("Waiting to pass time .....");

    await new Promise((res:any) => {
      setTimeout(()=>{
        res("");
      }, 2000)
    })

    const tx = await program.methods
    .withdraw(amount)
    .accounts({
        user: user.publicKey,
        mint: mint,
        userAta: userTokenAccount,
        chronoAccount: chrono,
        vault: vault,
      })
      .rpc();

    const userAta = await getAccount(connection, userTokenAccount);
    console.log("User ATA balance:", userAta.amount.toString());
    console.log("Transaction signature (deposit): ", tx);
  });
});
