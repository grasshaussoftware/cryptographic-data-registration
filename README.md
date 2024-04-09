# cryptographic-data-registration
Pear Project - OPEN SOURCE - MPL2.0 & GPL3.0 - VERITAS MMXXIII DEUSOPUS

## Whitepaper
[link](https://github.com/grasshaussoftware/crypto-id-whitepaper)

## Welcome to Pear🍐
Private Electronic Access Register
Requires Infura API

## Explainer Video
Check out the [explainer video](https://www.youtube.com/watch?v=M51dwN7r5Ao) for a quick overview of the project.


# Pear 0.1.0

The Pear project is a Rust application that facilitates the generation of cryptographic keys and QR tokens for users registering their global identity on the Avalanche-C blockchain.

## Features

- Generates RSA public/private key pairs based on the current Avalanche-C chain block hash as a seed.
- Creates mnemonic phrases for user seed generation.
- Converts mnemonic phrases and user data to QR tokens.
- Ensures sensitive information is not retained by zeroing memory.
- Prints your private key info to a CSN-A2 thermal printer via embedded_hal.
- Mints your public key QR code token to an NFT on the Avalanche C-chain for permanency.

## Requirements

- Rust Programming Language
- Cargo Package Manager
- Access to an Ethereum RPC URL (example uses Infura)

## Dependencies

To install all dependencies required for the project, ensure you have Cargo installed and run:

```bash
cargo build
```

The dependencies for this project are listed as follows:

```toml

[dependencies]
rsa = { version = "0.9.3", features = ["pem"] }
zeroize = "1.6.0"
tiny-bip39 = "1.0.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
qrcode = "0.12.0"
rand = "0.8.3"
web3 = "0.15"
tokio = "0.2.25"
hex = "0.4"
rand_core = "0.6"
rand_hc = "0.3"
chrono = { version = "0.4", features = ["serde"] }
```

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/grasshaussoftware/cryptographic-data-registration.git
cd cryptographic-data-registration
cargo build --release
```
## Usage

To run the program, use the following command:

```bash

cargo run

```

Follow the interactive prompts to input your data and receive your generated keys and QR tokens.

Use the app "QR Code Scanner" for mobile to read the QR codes and verify the information embedded in them

## Use Cases

Could be used to prove the legitimacy of a multitude of personal identification such as birth certificates, proof of citizenship, license plates, passports, driver's licenses, car title, etc. the list goes on and on. Basicall any and all noteable documents or data that would otherwise need a witness or notary public.

## Security

This application uses cryptographic RNG seeded with the current Avalanche C-Chain block hash for RSA key generation and zeroizes memory for sensitive information.
## License

MPL2.0 & GPL3 License
