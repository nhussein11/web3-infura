pub mod ethereum;
pub mod infura;

use std::env;

use ethereum::ethereum::{get_eth_balance, get_eth_blocknumber, get_eth_gasprice};
use infura::infura::{connect_to_infura_by_websocket, connect_to_infura_by_http};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let api_key = &env::var("INFURA_API_KEY").unwrap();
    let account_address = &env::var("ACCOUNT_ADDRESS").unwrap();

    // let celo_http = format!("https://celo-alfajores.infura.io/v3/{}", api_key);
    let infura_wss = format!("wss://mainnet.infura.io/ws/v3/{}", api_key);

    let web3s = connect_to_infura_by_websocket(&infura_wss).await?;

    // let web3s = connect_to_infura_by_http(&celo_http).await?;
    get_eth_balance(&web3s, account_address).await;
    get_eth_blocknumber(&web3s).await;
    get_eth_gasprice(&web3s).await;
    Ok(())
}
