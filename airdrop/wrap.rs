import bs58 from 'bs58'
import * as prompt from 'prompt-sync';


#[test]
fn base58_to_wallet() {
println!("lawalabdulrazaq:");
let stdin = io::stdin();
let base58 = stdin.lock().lines().next().unwrap().unwrap();

let wallet = bs58::decode(base58).into_vec().unwrap();
println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
let wallet: Vec<u8> =
vec![225,219,153,143,42,147,102,188,118,156,170,118,147,55,248,174,38,142,134,237,125,212,159,74,16,112,12,117,173,101,102,229,179,154,166,240,0,103,192,56,179,93,141,90,173,141,250,147,123,91,142,104,173,182,195,212,34,5,192,128,27,73,157,31];
let base58 = bs58::encode(wallet).into_string();
println!("{:?}", base58);
}