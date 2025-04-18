use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

/// Your private key file path.
const PRIVATE_KEY_PATH: &str = "PRIVATE_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed contract address.
const STYLUS_CONTRACT_ADDRESS: &str = "STYLUS_CONTRACT_ADDRESS";
const USER_ADDRESS: &str = "USER_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Log configuration (without exposing sensitive data)
    println!("Configuration loaded from environment variables");
    println!("RPC URL configured: {}", if env::var(RPC_URL).is_ok() { "Yes" } else { "No" });
    println!("Contract address configured: {}", if env::var(STYLUS_CONTRACT_ADDRESS).is_ok() { "Yes" } else { "No" });
    println!("User address configured: {}", if env::var(USER_ADDRESS).is_ok() { "Yes" } else { "No" });

    let priv_key_path = env::var(PRIVATE_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIVATE_KEY_PATH))?;
    let rpc_url = env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let contract_address = env::var(STYLUS_CONTRACT_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_CONTRACT_ADDRESS))?;
    let user_address_str = env::var(USER_ADDRESS).map_err(|_| eyre!("No {} env var set", USER_ADDRESS))?;
    let user_address: Address = user_address_str.parse().map_err(|e| eyre!("Failed to parse user address: {}", e))?;

    abigen!( //abigen! macro is used to generate type-safe bindings to the VendingMachine contract based on its ABI
        VendingMachine,
        r#"[
            function giveCupcakeTo(address user_address) external returns (bool)
            function getCupcakeBalanceFor(address user_address) external view returns (uint256)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = contract_address.parse()?;

    let privkey = read_secret_from_file(&priv_key_path)?;
    println!("Private key loaded from file successfully");

    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let vending_machine = VendingMachine::new(address, client);

    let balance = vending_machine.get_cupcake_balance_for(user_address).call().await?;
    println!("User cupcake balance = {:?}", balance);

    let tx_receipt = vending_machine.give_cupcake_to(user_address).send().await?.await?;
    match tx_receipt {
        Some(receipt) => {
            if receipt.status == Some(1.into()) {
                println!("Successfully gave cupcake to user via a tx");
            } else {
                println!("Failed to give cupcake to user, tx failed");
            }
        }
        None => {
            println!("Failed to get transaction receipt");
        }
    }

    let balance = vending_machine.get_cupcake_balance_for(user_address).call().await?;
    println!("New user cupcake balance = {:?}", balance);

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath).map_err(|e| eyre!("Failed to open private key file: {}", e))?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret).map_err(|e| eyre!("Failed to read private key: {}", e))?;
    
    // Validate that we have a non-empty key
    let trimmed = secret.trim().to_string();
    if trimmed.is_empty() {
        return Err(eyre!("Private key file is empty"));
    }
    
    Ok(trimmed)
}
