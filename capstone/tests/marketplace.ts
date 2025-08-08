import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MealPrepMarketplace } from "../target/types/meal_prep_marketplace";
import { airdropSol } from "./utils";
import { expect } from "chai";

describe("meal_prep_marketplace", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace
    .meal_prep_marketplace as Program<MealPrepMarketplace>;

  it("Marketplace initialization", async () => {
    const adminKey = anchor.web3.Keypair.generate();
    const platformFee = 250;

    await airdropSol(connection, adminKey.publicKey);

    const [marketplacePda, marketplaceBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace")],
        program.programId
      );

    const [treasuryPda, treasuryBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("treasury"), marketplacePda.toBuffer()],
        program.programId
      );

    try {
      await program.methods
        .initialize(platformFee)
        .accountsPartial({
          admin: adminKey.publicKey,
          marketplace: marketplacePda,
          treasury: treasuryPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([adminKey])
        .rpc();

      const marketplaceAccount = await program.account.marketplace.fetch(
        marketplacePda
      );
      console.log("Marketplace account:", marketplaceAccount);
    } catch (error) {
      console.error("Error:", error);
    }
  });
});
