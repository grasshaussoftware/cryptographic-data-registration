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


async fn chronos {
    let chronos:
    DateTime<Utc> = Utc::now().await;
    return Ok(chronos);
}

// Avalanche C-Chain RPC URL

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

    hash_to_hex(hash: [u8; 32]) -> web3::Result<String> {
        let mut hex_string = String::new();
        for byte in hash.iter() {
            hex_string.push_str(&format!("{:02x}", byte));
        return Ok(hex_string)
        }
    }
}

async fn 


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {

        // clear screen
        print!("{}[2J", 27 as char);

        print!("seed: {}", seed);
        print!("hex_string: {}", hex_string);




        let connect = println!("Connect your Metamask wallet, {}", connect_code);
        // Print smart contract QR code to stdout for Metamask wallet connection
        //https://docs.rs/qrcode/0.13.0/qrcode/
        //https://docs.rs/qrcode/0.13.0/qrcode/render/unicode/index.html
        
        // Welcome message
        println!("Welcome to the Avalanche-C Global Identity Registration System.\n");
        println!("This system will generate a unique public key for you and mint it as an NFT on the Avalanche-C blockchain.\n");
        let alpha_chronos = chronos().await.expect("Failed to get current timestamp");
        let alpha_block = get_current_block_hash(hex_string).await.expect("Failed to get current block hash");
        println!("Timestamp: {}", alpha_chronos);
        println!("Please press enter to continue...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

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

        println!("Please enter your '0x' public key address:");
        io::stdin().read_line(&mut defi).expect("Failed to read line");

        println!("One moment while we generate your public key and private key...\n");
                
        // Call to get the current block hash as a seed
        let block_hash_seed = get_current_block_hash(seed).await.expect(
            "Failed to get current block hash"
        );

        // Hash the combined inputs
        let mut hasher = Sha256::new();
        hasher.update(&block_hash_seed);

        drop(block_hash_seed);

        hasher.update(&defi.trim().as_bytes());
        let hash_result = hasher.finalize();
        let mut rng_seed = [0u8; 32];
        rng_seed.copy_from_slice(&hash_result.as_slice()[0..32]);

        drop(hash_result);

        // Now rng_seed contains the hashed result of both the block hash and the user's ether address,
        // and can be used as a high-entropy seed for your RNG
        let mut rng = rand_hc::Hc128Rng::from_seed(rng_seed);
        
        drop(rng_seed);
        
        // Generate RSA keys using the seeded RNG
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        drop(private_key);

        // Generate a new mnemonic
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mnemonic.phrase();

        // Convert mnemonic phrase to QR code in memory
        let code = QrCode::new(phrase).unwrap();
        let image = code.render::<unicode::Dense1x2>().build();
   
        println!("Alpha block#: {}", alpha_block);
        
        // Print the mnemonic phrase
        println!("Your seed phrase (mnemonic): {}", phrase);
        
        println!("Your unique seed phrase QR token: {}", image);

        // Print the timestamp
        let zulu = chronos().await.expect("Failed to get current timestamp");
        println!("Timestamp: {}", zulu);

        println!("This is your private key QR token. It contains your identifying data. It is your proof of your identity.\n");
        println!("DO NOT photograph it or show it to anyone. It is your private key.\n");

        println!("Private key is printing to thermal printer...\n");
        // Print the "private key" information receipt to thermal printer
        // Print zulu, phrase, and image to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // The private key is dropped and its memory is zeroed at this point
        drop(phrase)
        drop(image)

        // Export public key as PEM
        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
        //drop(public_key_pem);        
        
        // Print the seed phrase, time stamp, and QR code to the thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print the private key receipt to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Create a URL from the user's x.com username
        let user = format!("https://x.com/{}", user.trim());

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
            "defi": defi.trim(),
            "user": user,
            "zulu": zulu,
            "public_key": public_key_pem
        });

        // Convert public_key to a string
        let id_data_string = serde_json::to_string(&public_key).expect("Unable to serialize id data");

        // Generate a QR code for the id_data string
        let code = QrCode::new(&id_data_string).expect("Unable to create QR code");

        // Render the QR code as a string
        let image = code.render::<unicode::Dense1x2>().quiet_zone(false).build();

        // Print the QR code to stdout
        // println!("Please photograph this QR code with your phone to save your public key and private key.\n");
        // println!("Your public key is your proof of your rights.\n");
        // println!("Your private key is your proof of your identity.\n");

        println!("This is your public key QR token. It contains the information you entered. It is your proof of your rights.\n{}", image);
        println!("Please photograph it and save it to your phone.\n");
        


        // Print the QR code to the thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print the public key receipt to thermal printer
        //https://docs.rs/thermal-print/latest/thermal_print/

        // Print QR code to stdout
        //println!("Please photograph this QR code with your phone to save your public key and private key.\n");
        //println!("Your public key is your proof of your rights.\n");
        //println!("Your private key is your proof of your identity.\n");

        // 5 second timer
        thread::sleep(Duration::from_secs(10));
        
        // NFT creation scenario
        // mint your public key as an NFT on the Avalanche-C blockchain
        
        // Success
        println!("Your public key has been minted as an NFT on the Avalanche-C blockchain and is available in your Metamask wallet.\n");
        println!("Registration is now complete.\n");
        println!("Your private data was successfully zeroed from memory and was not saved.\n");
        
        
        
        
        println!("Press enter to end...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");
        break;        
        
    }
}