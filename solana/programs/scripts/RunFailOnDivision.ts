import { Keypair } from "@solana/web3.js";
import * as fs from 'fs';
import { serialize } from "borsh";
import { Buffer } from "buffer";

const web3 = require("@solana/web3.js");

// Flexible class that takes properties and imbues them to the object instance
class Assignable {
    constructor(properties) {
        Object.keys(properties).map((key) => {
            return (this[key] = properties[key]);
        });
    }
}

class Payload extends Assignable {
    dividend: number;
    divisor: number;
    remainder: number;
}

// Borsh needs a schema describing the payload
const payloadSchema = new Map([
    [
        Payload,
        {
            kind: "struct",
            fields: [
                ["dividend", "u8"],
                ["divisor", "u8"],
                ["remainder", "u8"],
            ],
        },
    ],
]);

async function main() {

    const CONNECTION_URL = "http://localhost:8899";

    const PAYER_KEYPAIR_FILE = "/home/andrei/work/src/public/solana/programs/keypairs/payer.json";
    const payerSecret = JSON.parse(fs.readFileSync(PAYER_KEYPAIR_FILE).toString()) as number[];
    const payerSecretKey = Uint8Array.from(payerSecret);
    const payerKeyPair = Keypair.fromSecretKey(payerSecretKey);
    console.log("Payer: ", payerKeyPair.publicKey);

    const PROGRAM_KEYPAIR_FILE = "/home/andrei/work/src/public/solana/programs/keypairs/fail-on-division.json";
    const programSecret = JSON.parse(fs.readFileSync(PROGRAM_KEYPAIR_FILE).toString()) as number[];
    const programSecretKey = Uint8Array.from(programSecret);
    const programKeyPair = Keypair.fromSecretKey(programSecretKey);
    console.log("Program: ", programKeyPair.publicKey);

    const connection = new web3.Connection(CONNECTION_URL);

    const transaction = new web3.Transaction({
        feePayer: payerKeyPair.publicKey,
    });

    let keys = [{ pubkey: payerKeyPair.publicKey, isSigner: true, isWritable: true }];

    const payload = new Payload({
        dividend: 7,
        divisor: 3,
        remainder: 2,
    });
    const payloadBuffer = Buffer.from(serialize(payloadSchema, payload));

    transaction.add(
        new web3.TransactionInstruction({
            keys: keys,
            programId: programKeyPair.publicKey,
            data: payloadBuffer,
        }),
    );

    console.log("Sending transaction...");
    const txHash = await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payerKeyPair],
    );
    console.log("Transaction Confirmed. Signature:", txHash);
}
main();