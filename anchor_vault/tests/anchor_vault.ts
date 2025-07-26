import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { assert } from "chai";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;
  const provider = anchor.AnchorProvider.env();
  const user = provider.wallet;
  let vaultState: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;

  it("Initialize vault", async () => {
    [vaultState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), user.publicKey.toBuffer()],
      program.programId
    );

    [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          user: user.publicKey,
          vaultState: vaultState,
          vault: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Initialize signature:", tx);
    } catch (error) {
      console.error("Error initializing vault:", error);
      throw error;
    }
  });

  it("Deposit into vault", async () => {
    const depositAmount = new anchor.BN(1000000);

    try {
      const tx = await program.methods
        .deposit(depositAmount)
        .accounts({
          user: user.publicKey,
          vaultState: vaultState,
          vault: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Deposit signature:", tx);
    } catch (error) {
      console.error("Error depositing into vault:", error);
      throw error;
    }
  });

  it("Withdraw from vault", async () => {
    const withdrawAmount = new anchor.BN(500000);

    try {
      const tx = await program.methods
        .withdraw(withdrawAmount)
        .accounts({
          user: user.publicKey,
          vaultState: vaultState,
          vault: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Withdrawal signature:", tx);
    } catch (error) {
      console.error("Error withdrawing from vault:", error);
      throw error;
    }
  });

  it("Close vault", async () => {
    const beforeBalance = await provider.connection.getBalance(user.publicKey);

    try {
      const tx = await program.methods
        .closeVault()
        .accounts({
          user: user.publicKey,
          vaultState: vaultState,
          vault: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

      const afterBalance = await provider.connection.getBalance(user.publicKey);  
      console.log("close vault signature:", tx);
      assert.ok(afterBalance > beforeBalance, "User should receive lamports back");
    } catch (error) {
      console.error("Error closing vault:", error);
      throw error;
    }
  });

});
