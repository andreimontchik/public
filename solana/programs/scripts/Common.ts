import { Keypair } from "@solana/web3.js";
import * as fs from 'fs';

export function loadKeyPair(keypairFile: string): Keypair {
    const payerSecret = JSON.parse(fs.readFileSync(keypairFile).toString()) as number[];
    const payerSecretKey = Uint8Array.from(payerSecret);
    return Keypair.fromSecretKey(payerSecretKey);
}