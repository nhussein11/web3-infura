pub mod infura;

use std::env;
use std::str::FromStr;

// TODO: fix web3 dependency imports
use web3::types::{H160, U256};

// import infura crate
use infura::infura::connect_to_infura;
#[tokio::main]
async fn main() -> web3::Result<()> {
  
    dotenv::dotenv().ok();
    let ws_url = &env::var("INFURA_WSS").unwrap();
    let web3s = connect_to_infura(ws_url).await?;
    let account = H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap());
    
    let balance = web3s.eth().balance(account.unwrap(), None).await?;
    
    println!("Wei balance of {:?}: {:?} wei", account.unwrap(), balance);

    let wei_conv: U256 = U256::exp10(18);

    println!(
        "ETH balance of {:?}: {} ETH",
        account.unwrap(),
        balance.checked_div(wei_conv).unwrap()
    );

    Ok(())
    
}


