import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MealPrepMarketplace } from "../target/types/meal_prep_marketplace";
import { airdropSol } from "./utils";
import { assert } from "chai";

describe("users testing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace
    .meal_prep_marketplace as Program<MealPrepMarketplace>;

  it("cook initialization", async () => {
    const cookKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, cookKey.publicKey);

    const [cookPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), cookKey.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser("Simon", "London", {
        cook: {},
      })
      .accountsPartial({
        user: cookKey.publicKey,
        userAccount: cookPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([cookKey])
      .rpc();

    const userAccount = await program.account.user.fetch(cookPda);
    assert.equal(userAccount.username, "Simon");
    assert.equal(userAccount.location, "London");
    assert.equal(userAccount.listingsCount.toNumber(), 1);
    assert.deepEqual(userAccount.userType, { cook: {} });
  });

  it("customer initialization", async () => {
    const customerKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, customerKey.publicKey);

    const [customerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), customerKey.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser("fred", "London", {
        customer: {},
      })
      .accountsPartial({
        user: customerKey.publicKey,
        userAccount: customerPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerKey])
      .rpc();

    const userAccount = await program.account.user.fetch(customerPda);
    assert.equal(userAccount.username, "fred");
    assert.equal(userAccount.location, "London");
    assert.equal(userAccount.listingsCount.toNumber(), 1);
    assert.deepEqual(userAccount.userType, { customer: {} });
  });
});
