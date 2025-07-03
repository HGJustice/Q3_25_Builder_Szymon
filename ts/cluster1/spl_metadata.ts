import wallet from "/Users/szymonlyzwinski/Desktop/wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("74HciTQ34GtbKkKa16BuA2kEaNmfV8FV7yY5ea8q7b2v");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
  try {
    // Start here
    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint: mint,
      mintAuthority: signer,
    };
    let data: DataV2Args = {
      name: "PussyBoi",
      symbol: "Puss",
      uri: "https://devnet.irys.xyz/E2Sgc8ytxbwoEcN86M84xTCEETvHdZRFpr7c8s8C1v8p",
      sellerFeeBasisPoints: 1,
      creators: null, //array of creator objects which contains userWallet and % of roylaties
      collection: null, //link this token to a parent collection
      uses: null, //how many times a nft can be used, null means unlimited
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data: data,
      isMutable: true,
      collectionDetails: null, // is this piece the parent to a collection
    };
    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });
    let result = await tx.sendAndConfirm(umi);
    console.log(bs58.encode(result.signature));
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
