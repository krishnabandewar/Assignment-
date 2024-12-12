# Secure Token Wallet

## Features
- Send tokens
- Receive tokens
- Check token balances

## Setup
1. Install Rust and DFX SDK.
2. Build and deploy the wallet:
cargo build --target wasm32-unknown-unknown --release
 dfx deploy wallet

## Usage
- Use `send_token`, `receive_token`, and `check_balance` functions.