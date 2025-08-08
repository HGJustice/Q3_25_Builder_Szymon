import * as anchor from "@coral-xyz/anchor";

export async function airdropSol(connection, publicKey) {
  const airdropTx = await connection.requestAirdrop(
    publicKey,
    2 * anchor.web3.LAMPORTS_PER_SOL
  );
  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    signature: airdropTx,
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
  });
}
