import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MealPrepMarketplace } from "../target/types/meal_prep_marketplace";
import { airdropSol } from "./utils";
import { assert, expect } from "chai";

describe("order testing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace
    .meal_prep_marketplace as Program<MealPrepMarketplace>;

  it("creates an order and changing cooks and customers order status", async () => {
    const cookKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, cookKey.publicKey);
    const customerKey = anchor.web3.Keypair.generate();
    await airdropSol(connection, customerKey.publicKey);

    const [cookPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), cookKey.publicKey.toBuffer()],
      program.programId
    );

    const [customerPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), customerKey.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser("cook", "London", {
        cook: {},
      })
      .accountsPartial({
        user: cookKey.publicKey,
        userAccount: cookPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([cookKey])
      .rpc();

    await program.methods
      .initializeUser("customer", "London", {
        customer: {},
      })
      .accountsPartial({
        user: customerKey.publicKey,
        userAccount: customerPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerKey])
      .rpc();

    const cookAccount = await program.account.user.fetch(cookPDA);
    const customerAccount = await program.account.user.fetch(customerPDA);

    assert.equal(cookAccount.username, "cook");
    assert.equal(customerAccount.username, "customer");

    const listingsCountBuffer = Buffer.allocUnsafe(8);
    listingsCountBuffer.writeBigUInt64LE(
      BigInt(cookAccount.listingsCount.toString())
    );

    const [listingPDA4] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        cookKey.publicKey.toBuffer(),
        listingsCountBuffer,
      ],
      program.programId
    );

    const [orderPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("order"),
        customerKey.publicKey.toBuffer(),
        listingPDA4.toBuffer(),
      ],
      program.programId
    );

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        customerKey.publicKey.toBuffer(),
        listingPDA4.toBuffer(),
      ],
      program.programId
    );

    const [marketplacePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace")],
      program.programId
    );

    await program.methods
      .cookCreatesListing(
        "chips",
        "fresh chips",
        new anchor.BN(1),
        new anchor.BN(10),
        false,
        null
      )
      .accountsPartial({
        user: cookKey.publicKey,
        userAccount: cookPDA,
        userListing: listingPDA4,
        marketplace: marketplacePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([cookKey])
      .rpc();

    const marketplaceAccount = await program.account.marketplace.fetch(
      marketplacePda
    );
    const cookAccount2 = await program.account.user.fetch(cookPDA);
    assert.equal(cookAccount2.listingsCount.toNumber(), 2);
    assert.equal(marketplaceAccount.totalListings.toNumber(), 3);

    const beforeBalance = await connection.getBalance(customerKey.publicKey);

    await program.methods
      .createOrder(new anchor.BN(1), new anchor.BN(5), null, "London")
      .accountsPartial({
        customer: customerKey.publicKey,
        cook: cookPDA,
        listing: listingPDA4,
        order: orderPDA,
        marketplace: marketplacePda,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerKey])
      .rpc();

    const marketplaceAccount2 = await program.account.marketplace.fetch(
      marketplacePda
    );
    const afterBalance = await connection.getBalance(customerKey.publicKey);
    expect(beforeBalance).to.be.greaterThan(afterBalance);
    assert.equal(marketplaceAccount2.totalOrders.toNumber(), 1);

    const orderAccountBefore = await program.account.order.fetch(orderPDA);
    assert(orderAccountBefore.cookOrderStatus, "confirmed");
    assert(orderAccountBefore.customerOrderStatus, "waiting");

    await program.methods
      .cookChangeOrderStatus(new anchor.BN(1), {
        ready: {},
      })
      .accountsPartial({
        user: cookKey.publicKey,
        cook: cookPDA,
        listing: listingPDA4,
        order: orderPDA,
      })
      .signers([cookKey])
      .rpc();

    await program.methods
      .customerChangeOrderStatus(new anchor.BN(1), {
        collected: {},
      })
      .accountsPartial({
        user: customerKey.publicKey,
        cook: cookPDA,
        listing: listingPDA4,
        order: orderPDA,
      })
      .signers([customerKey])
      .rpc();

    const orderAccountAfter = await program.account.order.fetch(orderPDA);
    assert(orderAccountAfter.cookOrderStatus, "ready");
    assert(orderAccountAfter.customerOrderStatus, "collected");
  });
});
