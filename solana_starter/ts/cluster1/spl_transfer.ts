import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wallet/turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { tokenAmount } from "@metaplex-foundation/umi";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("B89XFWfoEyTGtH1z6VMCwgPDpsEyWh6cQQjPpeU8v6t");

// Recipient address
const to = new PublicKey("D66kg6NQbxtSTQHvNzWhPAkGcoRK4uEAJkqXZ81hvSAA");

const token_decimals = 1_000_000n; // 6 decimals for the token

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ata_from = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const ata_to = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(
            connection,
            keypair,
            ata_from.address,
            ata_to.address,
            keypair.publicKey,
            10n * token_decimals
        );
        console.log(`Transfer tx signature: ${tx}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();