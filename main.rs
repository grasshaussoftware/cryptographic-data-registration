// main.rs
use rsa::{ RsaPrivateKey, RsaPublicKey };
use rsa::pkcs8::{ EncodePublicKey, LineEnding };
use bip39::{ Mnemonic, Language, MnemonicType };
use std::{ io };
use std::time::{ Duration };
use qrcode::QrCode;
use qrcode::render::unicode;
use serde_json::json;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        //Attach your metamask wallet

        // Collect user information
        let mut name = String::new();
        let mut bitcoin = String::new();
        let mut screenname = String::new();
        let mut email = String::new();
        
        //let mut ethereum = String::new();

        println!("Please enter your name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Please enter your Ethereum public key:");
        io::stdin().read_line(&mut bitcoin).expect("Failed to read line");

        println!("Please enter your screenname:");
        io::stdin().read_line(&mut screenname).expect("Failed to read line");

        println!("Please enter your email:");
        io::stdin().read_line(&mut email).expect("Failed to read line");

        // Generate RSA keys
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), bits).expect(
            "Failed to generate a key"
        );
        let public_key = RsaPublicKey::from(&private_key);

        // Export public key as PEM

        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;

        // Generate a new mnemonic
        // If you're following the typical `bip39` API:
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        // `phrase` will give you the actual mnemonic phrase as a string.
        let phrase = mnemonic.phrase();

        // The private key is dropped and its memory is zeroed at this point
        drop(private_key);

        println!("Your seed phrase (mnemonic): {}", phrase);

        // Convert mnemonic phrase to QR code and display it
        let code = QrCode::new(phrase).unwrap();
        let image = code.render::<unicode::Dense1x2>().build();
        println!("Your unique seed phrase is displayed as a QR token: {}", image);

        // Wait for 10 seconds before proceeding
        println!("Please take a photo of your private token now BUT do not share it.");

        println!("Your public token will be displayed momentarily, please wait...");
        thread::sleep(Duration::from_secs(60));

        // Prepare JSON data
        let id_data =
            json!({
        "name": name.trim(),
        "screenname": screenname.trim(),
        "email": email.trim(),
        "bitcoin address":bitcoin.trim(),
        "public_key": public_key_pem
    });

        // Convert id_data to a string
        let id_data_string = serde_json::to_string(&id_data).expect("Unable to serialize id data");

        // Generate a QR code for the id_data string
        let code = QrCode::new(&id_data_string).expect("Unable to create QR code");

        // You can adjust `scale` to change the size of the modules (the "pixels" of the QR code).
        // A smaller scale will produce a smaller QR code.
        let image = code
            .render::<unicode::Dense1x2>()
            .quiet_zone(false)
            .module_dimensions(1, 1)
            .build();

        println!("This is your public key QR token. Please photograph it for your records. It is your proof of your rights.\n{}", image);
        println!("Your private key was successfully destroyed.");

        // Reset for the next person to use it
        println!("Registration Complete. Thank you and have a nice day.");

        // Prompt to register next user or exit
        println!("Press Enter to register your global identity on the Ethereum blockchain...");
        let mut proceed = String::new();
        io::stdin().read_line(&mut proceed).expect("Failed to read line");

        println!("Hello!");
    }
}

