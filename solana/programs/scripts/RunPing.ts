import { Keypair } from "@solana/web3.js";
import * as fs from 'fs';

const web3 = require("@solana/web3.js");

async function main() {

    const CONNECTION_URL = "http://localhost:8899";

    const PAYER_KEYPAIR_FILE = "/home/andrei/work/src/public/solana/programs/keypairs/payer.json";
    const payerSecret = JSON.parse(fs.readFileSync(PAYER_KEYPAIR_FILE).toString()) as number[];
    const payerSecretKey = Uint8Array.from(payerSecret);
    const payerKeyPair = Keypair.fromSecretKey(payerSecretKey);
    console.log("Payer PubKey: ", payerKeyPair.publicKey);

    const PROGRAM_KEYPAIR_FILE = "/home/andrei/work/src/public/solana/programs/keypairs/ping.json";
    const programSecret = JSON.parse(fs.readFileSync(PROGRAM_KEYPAIR_FILE).toString()) as number[];
    const programSecretKey = Uint8Array.from(programSecret);
    const programKeyPair = Keypair.fromSecretKey(programSecretKey);
    console.log("Program PubKey: ", payerKeyPair.publicKey);

    const connection = new web3.Connection(CONNECTION_URL);

    const transaction = new web3.Transaction({
        feePayer: payerKeyPair.publicKey,
    });

    let keys = [{ pubkey: payerKeyPair.publicKey, isSigner: true, isWritable: true }];

    transaction.add(
        new web3.TransactionInstruction({
            keys: keys,
            programId: programKeyPair.publicKey,
        }),
    );

    console.log("Sending transaction...");
    const txHash = await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payerKeyPair],
    );

    console.log("Transaction sent with hash:", txHash);
}
main();