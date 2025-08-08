import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MealPrepMarketplace } from "../target/types/meal_prep_marketplace";
import { airdropSol } from "./utils";
import { assert } from "chai";

describe("meal_prep_marketplace", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace
    .meal_prep_marketplace as Program<MealPrepMarketplace>;

  const [marketplacePda, marketplaceBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace")],
      program.programId
    );

  it("User initialization", async () => {
    const userKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, userKey.publicKey);

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), userKey.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser("Simon", "London", {
        cook: {},
      })
      .accountsPartial({
        user: userKey.publicKey,
        userAccount: userPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKey])
      .rpc();

    const userAccount = await program.account.userManagement.fetch(userPda);
    assert.equal(userAccount.username, "Simon");
    assert.equal(userAccount.location, "London");
    assert.deepEqual(userAccount.userType, { cook: {} });
  });
});
