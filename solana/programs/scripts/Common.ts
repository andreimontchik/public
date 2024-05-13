import { Keypair } from "@solana/web3.js";
import * as fs from 'fs';

export function loadKeyPair(keypairFile: string): Keypair {
    const payerSecret = JSON.parse(fs.readFileSync(keypairFile).toString()) as number[];
    const payerSecretKey = Uint8Array.from(payerSecret);
    return Keypair.fromSecretKey(payerSecretKey);
}

// Flexible class that takes properties and imbues them to the object instance
export class Assignable {
    constructor(properties) {
        Object.keys(properties).map((key) => {
            return (this[key] = properties[key]);
        });
    }
}

export class FailOnDivisionPayload extends Assignable {
    dividend: number;
    divisor: number;
    remainder: number;
}

// Borsh needs a schema describing the payload
export const failOnDivisionPayloadSchema = new Map([
    [
        FailOnDivisionPayload,
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

