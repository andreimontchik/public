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

    if (process.argv.length !== 4) {
        console.error('Required two numeric input parameters `Dividend` and `Remainder`!');
        process.exit(1);
    }

    const dividend: number = parseInt(process.argv[2], 10);
    const remainder: number = parseInt(process.argv[3], 10);

    console.log('Dividend:', dividend);
    console.log('Remainder:', remainder);

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

    let keys = [{ pubkey: payerKeyPair.publicKey, isSigner: true, isWritable: true }];

    for (let i = 1; i <= dividend; i++) {
        const transaction = new web3.Transaction({
            feePayer: payerKeyPair.publicKey,
        });

        const payload = new Payload({
            dividend,
            divisor: i,
            remainder,
        });
        const payloadBuffer = Buffer.from(serialize(payloadSchema, payload));

        transaction.add(
            new web3.TransactionInstruction({
                keys: keys,
                programId: programKeyPair.publicKey,
                data: payloadBuffer,
            }),
        );

        try {
            console.log("Sending transaction with Payload: ", payload);
            const txHash = await web3.sendAndConfirmTransaction(
                connection,
                transaction,
                [payerKeyPair],
            );
            console.log(`Transaction is confirmed. Signature:`, txHash);
        } catch (error) {
            console.error(`Transaction is failed. Error: `, error.message);
        }
    }
}
main();