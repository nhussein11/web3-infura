pub mod ethereum;
pub mod infura;
mod cli;

use std::env;

use ethereum::ethereum::{get_eth_balance, get_eth_blocknumber, get_eth_gasprice};
use infura::infura::{HttpBuilder, WebSocketBuilder};
use cli::cli::run_cli;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let api_key = &env::var("INFURA_API_KEY").unwrap();
    let account_address = &env::var("ACCOUNT_ADDRESS").unwrap();

    // let wss_url = format!("wss://mainnet.infura.io/ws/v3/{}", api_key);
    // let web3s = WebSocketBuilder::new(wss_url).build().await;

    let http_url = format!("https://mainnet.infura.io/v3/{}", api_key);
    let web3s = HttpBuilder::new(http_url).build();

    get_eth_balance(&web3s, account_address).await;
    get_eth_blocknumber(&web3s).await;
    get_eth_gasprice(&web3s).await;
    run_cli();
    Ok(())
}

