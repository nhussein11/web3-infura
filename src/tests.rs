#[cfg(test)]
mod tests {
    use crate::ethereum::ethereum::{get_eth_balance, ETH_HTTP_URL};
    use crate::infura::infura::HttpBuilder;
    use std::env;

    #[tokio::test]
   async fn get_ethereum_balance() {
        let address = "0x71C7656EC7ab88b098defB751B7401B5f6d8976F";

        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        println!("http_url: {}", http_url);
        let web3s = HttpBuilder::new(http_url).build();
        let eth_balance = get_eth_balance(&web3s, &address).await;

        // The current balance of the address (at the time of writing this test) is 33 ETH
        assert_eq!(eth_balance, "33");
    }

   #[tokio::test]
   async fn try_to_get_ethereum_balance() {

   }
}
