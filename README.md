# Stylus Cupcake Vending Machine

A simple Rust-based smart contract for the Arbitrum Stylus platform that implements a virtual cupcake vending machine. This project demonstrates how to build, test, and interact with Stylus smart contracts.

## Overview

The Cupcake Vending Machine is a smart contract that:
- Tracks cupcake balances for users
- Distributes cupcakes to users (with a 5-second cooldown period)
- Demonstrates basic Stylus contract patterns

## Prerequisites

- Rust 1.83.0 (specified in rust-toolchain.toml)
- An Arbitrum Stylus-compatible RPC endpoint
- A private key for transaction signing

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/stylus-cupcake-example.git
   cd stylus-cupcake-example
   ```

2. Set up environment variables by copying the sample file:
   ```bash
   cp .env-sample .env
   ```

3. Edit the `.env` file with your specific configuration:
   - `PRIV_KEY_PATH`: Path to your private key file
   - `RPC_URL`: URL of the Stylus RPC endpoint
   - `STYLUS_CONTRACT_ADDRESS`: Address of your deployed contract
   - `USER_ADDRESS`: Address of the user to interact with

## Building the Contract

```bash
cargo build --release
```

To generate the ABI:

```bash
cargo stylus export-abi
```

## Testing

The project supports unit testing using the Stylus SDK's testing framework:

```bash
cargo test
```

## Contract Functionality

### VendingMachine Contract

The contract implements two main functions:

1. `give_cupcake_to(address)`: Gives a cupcake to the specified address if they haven't received one in the last 5 seconds
2. `get_cupcake_balance_for(address)`: Returns the cupcake balance for the specified address

### Storage Layout

The contract uses two mappings:
- `cupcake_balances`: Maps user addresses to their cupcake count
- `cupcake_distribution_times`: Maps user addresses to timestamps of their last cupcake

## Interacting with the Contract

The project includes an example script to interact with the deployed contract:

```bash
cargo run --example vending_machine
```

This will:
1. Check the cupcake balance of the specified user
2. Attempt to give a cupcake to the user
3. Check the updated balance

## License

This project is dual-licensed under MIT or Apache-2.0 at your option.

## Contributing

Contributions are welcome! Please sign off on your commits using the Developer Certificate of Origin (DCO) as described in the DCO.txt file.
