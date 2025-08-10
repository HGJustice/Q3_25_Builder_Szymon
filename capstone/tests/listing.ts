import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MealPrepMarketplace } from "../target/types/meal_prep_marketplace";
import { airdropSol } from "./utils";
import { assert } from "chai";

describe("listing testing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace
    .meal_prep_marketplace as Program<MealPrepMarketplace>;

  it("Listing initialization", async () => {
    let userKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, userKey);

    const [userPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), userKey.publicKey.toBuffer()],
      program.programId
    );

      const [listingPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), userKey.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.cookCreatesListing(
      "Pierogi",
      "potato and cheese dumplings, 8 dumplings in each portion",
      3,
      10,
      false,
      null
    ).accountsPartial({
        user: userKey.publicKey,
        userAccount: userPDA, 
        userListing: 
    });

    console.log(tx);
  });
});
