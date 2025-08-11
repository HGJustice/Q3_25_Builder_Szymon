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

  const [marketplacePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("marketplace")],
    program.programId
  );
  it("Listing initialization", async () => {
    const adminKey = anchor.web3.Keypair.generate();
    const platformFee = 250;
    await airdropSol(connection, adminKey.publicKey);

    const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), marketplacePda.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeMarketplace(platformFee)
      .accountsPartial({
        admin: adminKey.publicKey,
        marketplace: marketplacePda,
        treasury: treasuryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminKey])
      .rpc();

    let userKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, userKey.publicKey);

    const [userPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), userKey.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser("Simon", "London", {
        cook: {},
      })
      .accountsPartial({
        user: userKey.publicKey,
        userAccount: userPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKey])
      .rpc();

    const userAccount = await program.account.user.fetch(userPDA);
    const listingsCountBuffer = Buffer.allocUnsafe(8);
    listingsCountBuffer.writeBigUInt64LE(
      BigInt(userAccount.listingsCount.toString())
    );

    const [listingPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        userKey.publicKey.toBuffer(),
        listingsCountBuffer,
      ],
      program.programId
    );

    await program.methods
      .cookCreatesListing(
        "Pierogi",
        "potato and cheese dumplings, 8 dumplings in each portion",
        3,
        10,
        false,
        null
      )
      .accountsPartial({
        user: userKey.publicKey,
        userAccount: userPDA,
        userListing: listingPDA,
        marketplace: marketplacePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKey])
      .rpc();

    const listingAccount = await program.account.listing.fetch(listingPDA);
    const marketplace = await program.account.marketplace.fetch(marketplacePda);

    assert.equal(marketplace.totalListings.toNumber(), 1);
    assert.equal(listingAccount.listingId.toNumber(), 1);
    assert.equal(listingAccount.mealName, "Pierogi");

    const userAccount2 = await program.account.user.fetch(userPDA);
    const listingsCountBuffer2 = Buffer.allocUnsafe(8);
    listingsCountBuffer2.writeBigUInt64LE(
      BigInt(userAccount2.listingsCount.toString())
    );

    const [listingPDA2] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        userKey.publicKey.toBuffer(),
        listingsCountBuffer2,
      ],
      program.programId
    );

    await program.methods
      .cookCreatesListing("pizza", "pizza slices", 5, 15, false, null)
      .accountsPartial({
        user: userKey.publicKey,
        userAccount: userPDA,
        userListing: listingPDA2,
        marketplace: marketplacePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKey])
      .rpc();

    const listingAccount2 = await program.account.listing.fetch(listingPDA2);
    const marketplace2 = await program.account.marketplace.fetch(
      marketplacePda
    );

    assert.equal(marketplace2.totalListings.toNumber(), 2);
    assert.equal(listingAccount2.listingId.toNumber(), 2);
    assert.equal(listingAccount2.mealName, "pizza");
  });
  it("Listing creating and delisting", async () => {
    let userKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, userKey.publicKey);

    const [userPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), userKey.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser("Simon", "London", {
        cook: {},
      })
      .accountsPartial({
        user: userKey.publicKey,
        userAccount: userPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKey])
      .rpc();

    const userAccount = await program.account.user.fetch(userPDA);
    const listingsCountBuffer = Buffer.allocUnsafe(8);
    listingsCountBuffer.writeBigUInt64LE(
      BigInt(userAccount.listingsCount.toString())
    );

    const [listingPDA3] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        userKey.publicKey.toBuffer(),
        listingsCountBuffer,
      ],
      program.programId
    );

    await program.methods
      .cookCreatesListing(
        "nuggets",
        "fresh chicken and turkey nuggets",
        3,
        10,
        false,
        null
      )
      .accountsPartial({
        user: userKey.publicKey,
        userAccount: userPDA,
        userListing: listingPDA3,
        marketplace: marketplacePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKey])
      .rpc();

    const listingAccount3 = await program.account.listing.fetch(listingPDA3);
    const marketplace = await program.account.marketplace.fetch(marketplacePda);

    assert.equal(marketplace.totalListings.toNumber(), 3);
    assert.equal(listingAccount3.listingId.toNumber(), 1);
    assert.equal(listingAccount3.mealName, "nuggets");
  });
});
