import bs58 from 'bs58';
import promptSync from 'prompt-sync';

const prompt = promptSync();

// // Convert base58 string to wallet (byte array)
// function base58ToWallet() {
//     const base58 = prompt('Enter base58 string: ');
//     const wallet = bs58.decode(base58);
//     console.log('Byte array:', wallet);
// }

// Hardcoded base58 string
const base58 = 'your_base58_string_here';

// Convert base58 string to wallet (byte array)
function base58ToWallet() {
    const wallet = bs58.decode(base58);
    console.log('Byte array:', wallet);
}

// Convert wallet (byte array) to base58 string
function walletToBase58() {
    const wallet = Uint8Array.from([
        225,219,153,143,42,147,102,188,118,156,170,118,147,55,248,174,38,142,134,237,125,212,159,74,16,112,12,117,173,101,102,229,179,154,166,240,0,103,192,56,179,93,141,90,173,141,250,147,123,91,142,104,173,182,195,212,34,5,192,128,27,73,157,31
    ]);
    const base58 = bs58.encode(wallet);
    console.log('Base58 string:', base58);
}

// Example usage:
base58ToWallet();
walletToBase58();