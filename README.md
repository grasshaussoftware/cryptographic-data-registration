# cryptographic-data-registration
OPEN SOURCE GPL3.0- "VERITAS PROJECT" MMXXIII by DEUSOPUS - AD HOC - beta

## Whitepaper
https://drive.google.com/drive/folders/1OIWfF11SJt3TmEB7BoRdOdstNurHJDgx

## Welcome to Pear🍐
"think twice"
"got your pear?"

## Watch it in action on YouTube
https://youtu.be/a1LZxF1r2V4?si=4NpPcSX7Ryprgkug

# Pear Project

The Pear Project is a Rust application that facilitates the generation of cryptographic keys and QR tokens for users registering their global identity on the Avalanche-C blockchain.

## Features

- Generates RSA public/private key pairs based on the current Ethereum block hash as a seed.
- Creates mnemonic phrases for user seed generation.
- Converts mnemonic phrases and user data to QR tokens.
- Ensures sensitive information is not retained by zeroing memory.

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
Security

This application uses cryptographic RNG seeded with the current Ethereum block hash for RSA key generation and zeroizes memory for sensitive information.
License

GPL3 License
