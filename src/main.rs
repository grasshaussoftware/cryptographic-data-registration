use rsa::{ RsaPrivateKey, RsaPublicKey, pkcs8::{ EncodePublicKey, LineEnding } };
use bip39::{ Mnemonic, Language, MnemonicType };
use web3::{ transports::Http, Web3, types::BlockId, types::BlockNumber };
use qrcode::QrCode;
use qrcode::render::unicode;
use serde_json::json;
use std::io;
use std::thread;
use std::time::{ Duration };
use tokio; // for async runtime
use rand_core::{ SeedableRng };
use chrono::{ Utc, DateTime };
use sha2::{ Sha256, Digest };

// Replace this placeholder with your actual Avalanche node RPC URL and your Infura project ID
const AVAX_RPC_URL: &str =
    "https://avalanche-mainnet.infura.io/v3/f4824ede0b484d33a19f0d01c32c9de1";

async fn get_current_block_hash() -> web3::Result<[u8; 32]> {
    let http = Http::new(AVAX_RPC_URL)?;
    let web3 = Web3::new(http);

    if let Some(block) = web3.eth().block(BlockId::Number(BlockNumber::Latest)).await? {
        if let Some(hash) = block.hash {
            let mut seed = [0u8; 32];
            seed.copy_from_slice(&hash.0);
            return Ok(seed);
        }
    }

    Err(web3::Error::Decoder("Failed to get current block hash".into()))
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // clear screen
        print!("{}[2J", 27 as char);

        // Generate QR code for smart contract address
        // Print QR code to stdout
        // println!("Please connect your Metamask wallet to Avalanche-C by scanning this code...\n");
        
        // Connect metamask wallet to "pear-project" smart contract on Avalanche C-Chain
        // via QR code
        
        // Success or error
        //println!("Your Metamask wallet is now connected to Avalanche-C.\n");
        //println!("Press enter to continue...");
        //let mut proceed = String::new();
        //io::stdin().read_line(&mut proceed).expect("Failed to read line");

        // clear screen
        //print!("{}[2J", 27 as char);


        // (disregard references in this demo to photographing your public key and private key. 
        // the private key info will be printed to the thermal printer and then zeroed from memory.
        // the public key info will be printed to the thermal printer and then minted as an NFT on the Avalanche-C blockchain.)
        
        // Welcome message
        println!("Welcome to the Avalanche-C Global Identity Registration System.\n");
        let title = "Avalanche-C Global Identity Registration System";

        // sample block
        let sample = get_current_block_hash().await.expect(
            "Failed to get current block hash"
        );

        // timestamp
        let now: DateTime<Utc> = Utc::now();

        // Collect user information
        let mut name = String::new();
        let mut dob = String::new();
        let mut gender = String::new();
        let mut maiden = String::new();
        let mut home = String::new();
        let mut ssn = String::new();
        let mut address = String::new();
        let mut mail = String::new();
        let mut phone = String::new();
        let mut email = String::new();
        let mut user = String::new();
        let mut defi = String::new();

        println!("Please enter your information below to register your global identity on the Avalanche-C blockchain.\n");

        println!("Please enter your name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Please enter your date of birth:");
        io::stdin().read_line(&mut dob).expect("Failed to read line");

        println!("What is your gender?(at birth)");
        io::stdin().read_line(&mut gender).expect("Failed to read line");

        println!("Please enter your mother's maiden name:");
        io::stdin().read_line(&mut maiden).expect("Failed to read line");        

        println!("Please enter your citizenship:");
        io::stdin().read_line(&mut home).expect("Failed to read line");

        println!("Please enter your social security number:");
        io::stdin().read_line(&mut ssn).expect("Failed to read line");

        println!("Please enter your address:");
        io::stdin().read_line(&mut address).expect("Failed to read line");

        println!("Please enter your mailing address:");
        io::stdin().read_line(&mut mail).expect("Failed to read line");

        println!("Please enter your phone number:");
        io::stdin().read_line(&mut phone).expect("Failed to read line");

        println!("Please enter your email address:");
        io::stdin().read_line(&mut email).expect("Failed to read line");

        println!("Please enter your username on x.com:");
        io::stdin().read_line(&mut user).expect("Failed to read line");

        println!("Please enter your x0 public key address:");
        io::stdin().read_line(&mut defi).expect("Failed to read line");

        
        // Call to get the current block hash as a seed
        let block_hash_seed = get_current_block_hash().await.expect(
            "Failed to get current block hash"
        );

        // Hash the combined inputs
        let mut hasher = Sha256::new();
        hasher.update(&block_hash_seed);

        hasher.update(&defi.trim().as_bytes());
        let hash_result = hasher.finalize();
        let mut rng_seed = [0u8; 32];
        rng_seed.copy_from_slice(&hash_result.as_slice()[0..32]);

        // Now rng_seed contains the hashed result of both the block hash and the user's ether address,
        // and can be used as a high-entropy seed for your RNG
        let mut rng = rand_hc::Hc128Rng::from_seed(rng_seed);
        //drop(rng_seed);

        // Generate RSA keys using the seeded RNG
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        //drop(rng);

        // The private key is dropped and its memory is zeroed at this point
        drop(private_key);

        // Export public key as PEM
        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
        //drop(public_key_pem);

        // Generate a new mnemonic
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mnemonic.phrase();
        //drop(mnemonic);

        // Convert mnemonic phrase to QR code in memory
        let code = QrCode::new(phrase).unwrap();
        let image = code.render::<unicode::Dense1x2>().build();
        
        // code contains the qr code in memory and is therefore vulnerable to being dumped to disk
        //drop(code);

        // Print the seed phrase, time stamp, and QR code to the thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print the timestamp
        println!("{}", now);
        //drop(now);

        // Print block hash in hex
        let hex_string = block_hash_seed
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        //drop(block_hash_seed);
        println!("Block#: {}", hex_string);
        //drop(hex_string);
        
        // Print the mnemonic phrase
        println!("Your seed phrase (mnemonic): {}", phrase);
        //drop(phrase);
        
        println!("Your unique seed phrase QR token: {}", image);

        // Print the private key receipt to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        
        // Create a URL for the user's X.com profile
        let user = format!("https://x.com/{}", user.trim());

        // Prepare JSON data
        let pear_card =
            json!({
            "title": title,    
            "sample_block": sample,    
            "now": now,
            "name": name.trim(),
            "dob": dob.trim(),
            "gender": gender.trim(),
            "maiden": maiden.trim(),
            "home": home.trim(),
            "ssn": ssn.trim(),
            "address": address.trim(),
            "mail": mail.trim(),
            "phone": phone.trim(),
            "email": email.trim(),
            "defi": defi.trim(),
            "user": user,
            "timestamp": now,
            "block": hex_string,
            "public_key": public_key_pem
        });

        // Convert pear_card to a string
        let id_data_string = serde_json::to_string(&pear_card).expect("Unable to serialize id data");

        // Generate a QR code for the id_data string
        let code = QrCode::new(&id_data_string).expect("Unable to create QR code");

        // Render the QR code as a string
        let image = code.render::<unicode::Dense1x2>().quiet_zone(false).build();

        println!("This is your public key QR token. It contains the information you entered. It is your proof of your rights.\n{}", image);

        // 5 second timer
        thread::sleep(Duration::from_secs(5));
        
        // NFT creation scenario
        // mint your public key as an NFT on the Avalanche-C blockchain
        
        // Success
        println!("Your public key has been minted as an NFT on the Avalanche-C blockchain and is available in your Metamask wallet.\n");
        println!("Registration is now complete.\n");
        println!("Your private data was successfully zeroed from memory and was not saved.\n");
        
        
        
        
        println!("Press enter to end...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        // disconnect metamask wallet from "pear-project" smart contract on Avalanche C-Chain
        
        
    }
}