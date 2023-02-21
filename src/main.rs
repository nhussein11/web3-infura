pub mod infura;
pub mod ethereum;

use std::env;

use infura::infura::connect_to_infura;
use ethereum::ethereum::get_eth_balance;

#[tokio::main]
async fn main() -> web3::Result<()> {
  
    dotenv::dotenv().ok();
    let infura_wss = &env::var("INFURA_WSS").unwrap();
    let account_address = &env::var("ACCOUNT_ADDRESS").unwrap();
    
    let web3s = connect_to_infura(infura_wss).await?;
    get_eth_balance(web3s, account_address).await;
    Ok(())
}
