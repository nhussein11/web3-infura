mod cli;
pub mod ethereum;
pub mod infura;

use std::env;

use cli::cli::run_cli;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let api_key = &env::var("INFURA_API_KEY").unwrap();

    run_cli(api_key).await;
 
    Ok(())
}
