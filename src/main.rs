use rsa::{ RsaPrivateKey, RsaPublicKey, pkcs8::{ EncodePublicKey, LineEnding } };
use bip39::{ Mnemonic, Language, MnemonicType };
use web3::{ transports::Http, Web3, types::BlockId, types::BlockNumber };
use qrcode::QrCode;
use qrcode::render::unicode;
use serde_json::json;
use std::io;
use std::thread::sleep;
use std::time::{ Duration };
use tokio; // for async runtime
use rand_core::{ SeedableRng };
use chrono::{ Utc, DateTime };
use sha2::{ Sha256, Digest };
use rand::{ Rng, rngs::StdRng };

// Avalanche C-Chain RPC URL

const AVAX_RPC_URL: &str =
    "https://avalanche-mainnet.infura.io/v3/f4824ede0b484d33a19f0d01c32c9de1";

async fn get_current_block_hash() -> web3::Result<[u8; 32]> {
    let http = Http::new(AVAX_RPC_URL)?;
    let web3 = Web3::new(http);

    if let Some(block) = web3.eth().block(BlockId::Number(BlockNumber::Latest)).await? {
        if let Some(hash) = block.hash {
            Ok(hash.0)
        } else {
            Err(web3::Error::Decoder("No hash found for the latest block".into()))
        }
    } else {
        Err(web3::Error::Decoder("Failed to fetch the latest block".into()))
    }
}

fn hash_to_hex(hash: [u8; 32]) -> std::string::String {
    let mut hex_string = String::new();
    for byte in hash.iter() {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string // Return the hexadecimal string
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // clear screen
        print!("{}[2J", 27 as char);

        async fn get_chronos() -> DateTime<Utc> {
            Utc::now()
        }

        let alpha_block_hash = get_current_block_hash().await.expect(
            "Failed to get current block hash"
        );
        let alpha_block = hash_to_hex(alpha_block_hash);

        // Generate a QR code for the smart contract address
        //https://docs.rs/qrcode/0.13.0/qrcode/
        //https://docs.rs/qrcode/0.13.0/qrcode/render/unicode/index.html
        // Print QR code to stdout
        //println!("Please scan this QR code with your phone to connect your Metamask wallet to the Avalanche-C blockchain.\n");

        //let connect = println!("Connect your Metamask wallet, {}", connect_code);
        // Print smart contract QR code to stdout for Metamask wallet connection
        //https://docs.rs/qrcode/0.13.0/qrcode/
        //https://docs.rs/qrcode/0.13.0/qrcode/render/unicode/index.html

        // Welcome message
        println!("Welcome to the Avalanche-C Global Identity Registration System.");
        println!(
            "This system will generate a unique public key for you and mint it as an NFT on the Avalanche-C blockchain. Your private key will be printed on the thermal printer for your records."
        );
        let alpha_chronos = get_chronos().await;
        println!("Current UTC time: {}", alpha_chronos);
        println!("Current block: {:?}", alpha_block);

        println!("Please press enter to continue...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        // clear screen
        print!("{}[2J", 27 as char);

        // Generate QR code for smart contract address

        // Print QR code to stdout

        //

        let title = "Avalanche-C Global Identity Registration System";

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

        println!(
            "Please enter your information below to register your global identity on the Avalanche-C blockchain.\n"
        );

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

        println!(
            "Please enter your '0x' DeFi public key address for increased entropy. It will appear on your public key QR token. Enter '0' to opt out:"
        );
        io::stdin().read_line(&mut defi).expect("Failed to read line");

        println!("One moment while we generate your public key and private key QR tokens...");

        // Initialize a random number generator
        let mut rng = StdRng::from_entropy();

        // Generate a random number, for example, a u64
        let random_number: u64 = rng.gen();

        // Hash the combined inputs
        let mut hasher = Sha256::new();

        // Convert alpha_chronos to a string representation or timestamp
        let alpha_chronos_str = alpha_chronos.to_rfc3339(); // or another appropriate format

        // Update the hasher with the alpha_block, alpha_chronos, defi, and random number
        hasher.update(alpha_block.trim().as_bytes());
        hasher.update(alpha_chronos_str.as_bytes());
        hasher.update(defi.trim().as_bytes());
        hasher.update(&random_number.to_be_bytes()); // Convert the random number to bytes and update the hasher

        // Finalize the hash and create the rng_seed
        let hash_result = hasher.finalize();
        let mut rng_seed = [0u8; 32];
        rng_seed.copy_from_slice(&hash_result.as_slice()[0..32]);

        // Use rng_seed as a high-entropy seed for your RNG
        let mut rng = rand_hc::Hc128Rng::from_seed(rng_seed);

        // Generate RSA keys using the seeded RNG
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        // Generate a new mnemonic
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mnemonic.phrase();

        // Convert mnemonic phrase to QR code in memory
        let code = QrCode::new(phrase).unwrap();
        let image = code.render::<unicode::Dense1x2>().build();

        // clear screen
        print!("{}[2J", 27 as char);

        // Print the current time
        let now = get_chronos().await;
        println!("Current time: {}", now);
        // Print the block hash that was current at the initiation of this process
        println!("Alpha block: {}", alpha_block);
        // Print the mnemonic phrase
        println!("Your seed phrase (mnemonic) is: {}", phrase);
        // Print the mnemonic phrase QR code to stdout
        println!("{}",image);

        // Print the timestamp

        println!(
            "This is your private key QR token. It is your proof of your identity.\n"
        );
        println!(
            "DO NOT photograph it or show it to anyone. It is your private key. Please scan it with a trusted QR code scanner to verify that it is correctly displaying your seed phrase.\n"
        );
        println!("Please press enter to continue...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        print!("{}[2J", 27 as char);

        println!("Private key QR token is printing to thermal printer. Keep it in a safe place.\n");
        // Print the "private key" information receipt to thermal printer
        // Print alpha_block, phrase, and image to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // The private key QR token is dropped and its memory is zeroed at this point
        drop(image);

        sleep(Duration::from_secs(10));
        // clear screen
        print!("{}[2J", 27 as char);

        // Print the seed phrase, time stamp, and QR code to the thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print the private key receipt to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Create a URL from the user's x.com username
        let user = format!("https://x.com/{}", user.trim());

        // Get the current time
        let zulu = get_chronos().await;

        // Get the current block hash
        let zulu_block_hash = get_current_block_hash().await.expect(
            "Failed to get current block hash"
        );
        let zulu_block = hash_to_hex(zulu_block_hash);

        // Export public key as PEM
        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;

        // Prepare JSON data
        let public_key =
            json!({
            "title": title,
            "alpha_chronos": alpha_chronos, 
            "alpha_block": alpha_block,   
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
            "user": user,
            "defi": defi,
            "zulu_chronos": zulu,
            "zulu_block": zulu_block,
            "public_key": public_key_pem
        });

        // Convert public_key to a string
        let id_data_string = serde_json
            ::to_string(&public_key)
            .expect("Unable to serialize id data");

        // Generate a QR code for the id_data string
        let code = QrCode::new(&id_data_string).expect("Unable to create QR code");

        // Render the QR code as a string
        let image = code.render::<unicode::Dense1x2>().quiet_zone(false).build();

        // Print the QR code to stdout
        // println!("Please photograph this QR code with your phone to save your public key and private key.\n");
        // println!("Your public key is your proof of your rights.\n");
        // println!("Your private key is your proof of your identity.\n");

        println!("{}\nThis is your public key QR token. It contains the identifying information you entered. It is your proof of your rights. You may show it to anyone.", image);
        println!("Please photograph it and save it to your phone and your cloud.");

        // Print the QR code to the thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print the public key receipt to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print QR code to stdout
        //println!("Please photograph this QR code with your phone to save your public key and private key.\n");
        //println!("Your public key is your proof of your rights.\n");
        //println!("Your private key is your proof of your identity.\n");
        println!("Please press enter to continue...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        // NFT creation scenario
        // mint your public key as an NFT on the Avalanche-C blockchain
        println!(
            "Your public key will now being minted as an NFT on the Avalanche-C blockchain. You will be charged a minimal gas fee. Follow the prompts in your Metamask wallet to complete the transaction.\n"
        );
        println!("Please press enter to continue...");
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        // Success
        println!(
            "Your public key has been minted as an NFT on the Avalanche-C blockchain and is available in your Metamask wallet."
        );
        println!("Registration is now complete.");
        println!("Your private data was successfully zeroed from memory and was not saved.");

        println!("Press enter to end...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");
        break;
    }

    Ok(())
}

// by grasshaussoftware c2023. all rights reserved.
// https://github.com/grasshaussoftware/cryptographic-data-registration
// contact: deusopus@duck.com
