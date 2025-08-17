import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.anchorVault as Program<AnchorVault>;
  
  const user = provider.wallet.payer;
  const [vault, bump] = PublicKey.findProgramAddressSync([Buffer.from("vault"), user.publicKey.toBuffer()], program.programId)
  const amount = new anchor.BN(1_000_000_000)

  it("Transfer amount to vault", async () => {


    const tx = await program.methods.deposit(amount).accounts({user: user.publicKey, vault: vault}).signers([user]).rpc();

    const vaultBalance = await provider.connection.getBalance(vault);

    expect(vaultBalance).to.be.eq(amount.toNumber());
    console.log("Your transaction signature", tx);
  });


  it("Withdraw vault amount", async () => {
    const tx = await program.methods.withdraw(amount).accounts({user: user.publicKey, vault: vault}).rpc();

    const vaultBalance = await provider.connection.getBalance(vault);

    expect(vaultBalance).to.be.eq(0);
    console.log("Your transaction signature", tx);
  });
});
