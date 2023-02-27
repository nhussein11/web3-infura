mod cli;
pub mod ethereum;
pub mod infura;

use std::env;

use cli::cli::run_cli;
use ethereum::ethereum::{get_eth_balance, get_eth_blocknumber, get_eth_gasprice};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let api_key = &env::var("INFURA_API_KEY").unwrap();

    let account_address = &env::var("ACCOUNT_ADDRESS").unwrap();
    println!("Account Address: {}", account_address);

    run_cli(api_key).await;
    Ok(())
}
