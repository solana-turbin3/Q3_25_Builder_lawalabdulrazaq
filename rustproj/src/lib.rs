#[cfg(test)]
mod tests {
    use bs58;
    use solana_program::instruction::Instruction;
    use solana_sdk::instruction::AccountMeta;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;
    use std::str::FromStr;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    };
    use solana_program::hash::hash;
    use solana_program::system_program;


    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();

        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");

        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn airdrop() {
        // Import our keypair
        let _keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // we'll establish a connection to Solana devnet using the const we defined above
        let _client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match _client.request_airdrop(&_keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    // fn transfer_sol() {
    //     // Load your devnet keypair from file
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    //     // Generate a signature from the keypair
    //     let pubkey = keypair.pubkey();
    //     let message_bytes = b"I verify my Solana Keypair!";
    //     let sig = keypair.sign_message(message_bytes);
    //     let sig_hashed = hash(sig.as_ref());

    //     // Verify the signature using the public key
    //     match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
    //     true => println!("Signature verified"),
    //     false => println!("Verification failed"),
    //     }

    //     let to_pubkey = Pubkey::from_str("FL83sqP8mGmBcCcr9ogApux8rCZefgLCPHZoc7kTsru5").unwrap();
    //     let rpc_client = RpcClient::new(RPC_URL);
    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .expect("Failed to get recent blockhash");

    //     // Create a transfer instruction
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
    //         Some(&keypair.pubkey()),
    //         &vec![&keypair],
    //         recent_blockhash,
    //     );
    //     // Send the transaction
    //     let signature = rpc_client
    //         .send_and_confirm_transaction(&transaction)
    //         .expect("Failed to send transaction");
    //     println!(
    //         "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
    //         signature
    //     );



    // }
    fn transfer_sol() {
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Generate a signature from the keypair
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());

        // Verify the signature using the public key
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("FL83sqP8mGmBcCcr9ogApux8rCZefgLCPHZoc7kTsru5").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("Turbin-wallet.json")
            .expect("Couldn't find wallet file");

        // Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();

        // Get the PDA (Program Derived Address)
        // Derive the 'account' PDA (prereq_pda)
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

        // Derive the 'authority' PDA
        let authority_seed = &[b"collection", collection.as_ref()];
        let (authority, _auth_bump) = Pubkey::find_program_address(authority_seed, &turbin3_prereq_program);

        // the submit_rs instruction discriminator is
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        // Define the accounts metadata
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true), // user signer
            AccountMeta::new(prereq_pda, false), // PDA account
            AccountMeta::new(mint.pubkey(), true), // mint keypair
            AccountMeta::new(collection, false), // collection
            AccountMeta::new_readonly(authority, false), // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];

        // get recent block hash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        
        // Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        // Send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );


    }
}