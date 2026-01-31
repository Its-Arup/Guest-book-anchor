import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Guestbook } from "../target/types/guestbook";
import { assert } from "chai";

describe("guestbook", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.guestbook as Program<Guestbook>;
  const provider = anchor.getProvider();

  it("Is initialized!", async () => {
    // Derive the PDA for the guestbook account
    const [guestbookPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("guestbook"), provider.publicKey.toBuffer()],
      program.programId
    );

    // Call the initialize instruction
    const tx = await program.methods
      .initialize()
      .accounts({
        signer: provider.publicKey,
        entries: guestbookPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    // Fetch the created guestbook account
    const guestbookAccount = await program.account.guestbook.fetch(guestbookPDA);

    // Verify the guestbook was initialized correctly
    assert.strictEqual(
      guestbookAccount.signer.toString(),
      provider.publicKey.toString(),
      "Signer should match the provider's public key"
    );
    
    assert.strictEqual(
      guestbookAccount.entries.length,
      0,
      "Entries should be empty after initialization"
    );

    console.log("Guestbook initialized successfully!");
    console.log("Guestbook PDA:", guestbookPDA.toString());
    console.log("Signer:", guestbookAccount.signer.toString());
    console.log("Entries count:", guestbookAccount.entries.length);
  });
});
