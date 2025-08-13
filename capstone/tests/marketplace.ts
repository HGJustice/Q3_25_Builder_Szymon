import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MealPrepMarketplace } from "../target/types/meal_prep_marketplace";
import { assert } from "chai";

describe("marketplace testing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace
    .meal_prep_marketplace as Program<MealPrepMarketplace>;

  it("Marketplace initialization", async () => {
    const [marketplacePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace")],
      program.programId
    );

    const marketplaceAccount = await program.account.marketplace.fetch(
      marketplacePda
    );
    assert.equal(marketplaceAccount.fees, 250);
  });
});
