import wallet from "/Users/szymonlyzwinski/Desktop/wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    const image =
      "https://devnet.irys.xyz/Chzzu3VrUBWHwpuSUoMbnq8ebqrLs17D8TrebpRNNqKr";
    const metadata = {
      name: "Turbin3 Rug",
      symbol: "T3R",
      description: "a auto geenrated rug",
      image: image,
      attributes: [{ trait_type: "colour", value: "5" }],
      properties: {
        files: [
          {
            type: "image/png",
            uri: "image",
          },
        ],
      },
      creators: [],
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
