mod tests; 
mod ethereum;
mod infura;
#[path = "cli/cli.rs"] mod cli;
use crate::cli::cli::run_cli;

use std::env;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let api_key = &env::var("INFURA_API_KEY").unwrap();

    run_cli(api_key).await;
 
    Ok(())
}
