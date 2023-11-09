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
use chrono::{Utc, DateTime};

// Replace this placeholder with your actual Ethereum node RPC URL and your Infura project ID
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

        //timestamp
        let now: DateTime<Utc> = Utc::now();
        
        // Collect user information
        let mut name = String::new();
        let mut maiden = String::new();
        let mut ether = String::new();
        let mut handle = String::new();

        println!("Please enter your name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Please enter your mother's maiden name:");
        io::stdin().read_line(&mut maiden).expect("Failed to read line");



        println!("Please enter your Ethereum public key:");
        io::stdin().read_line(&mut ether).expect("Failed to read line");

        println!("Please enter your handle on X.com:");
        io::stdin().read_line(&mut handle).expect("Failed to read line");

        let url = format!("https://x.com/{}", handle.trim());
        
        // Call to get the current Ethereum block hash as a seed
        let block_hash_seed = get_current_block_hash().await.expect(
            "Failed to get current block hash"
        );

        // Create a seeded RNG
        let mut rng = rand_hc::Hc128Rng::from_seed(block_hash_seed);

        // Generate RSA keys using the seeded RNG
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        // Export public key as PEM
        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;

        // Generate a new mnemonic
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mnemonic.phrase();

        // The private key is dropped and its memory is zeroed at this point
        drop(private_key);

        // Print the seed phrase, time stamp, and QR code to the thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print the mnemonic phrase
        println!("Your seed phrase (mnemonic): {}", phrase);
        // Print the timestamp
        println!("{}", now);
        // Convert mnemonic phrase to QR code and display it
        let code = QrCode::new(phrase).unwrap();
        let image = code.render::<unicode::Dense1x2>().build();
        println!("Your unique seed phrase QR token: {}", image);
        
        // Wait for 25 seconds before proceeding
        println!("Please take a photo of your seed phrase QR token, but do not share it.");
        thread::sleep(Duration::from_secs(25));

        // Prepare JSON data
        let id_data =
            json!({
            "name": name.trim(),
            "maiden": maiden.trim(),
            "ether": ether.trim(),
            "x.com": url,
            "timestamp": now,
            "public_key": public_key_pem
        });

        // Convert id_data to a string
        let id_data_string = serde_json::to_string(&id_data).expect("Unable to serialize id data");

        // Generate a QR code for the id_data string
        let code = QrCode::new(&id_data_string).expect("Unable to create QR code");

        // Render the QR code as a string
        let image = code.render::<unicode::Dense1x2>().quiet_zone(false).build();

        println!("This is your public key QR token. Please photograph it for your records. It is your proof of your rights.\n{}", image);
        
        println!("Your private key was successfully destroyed from memory as was not saved.");

        // Reset for the next person to use it
        println!("Registration Complete. Thank you and have a nice day.");

        // Prompt to register next user or exit
        println!("Press Enter to register your global identity on the Avalanche-C blockchain...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        println!("Hello!");
    }
}
