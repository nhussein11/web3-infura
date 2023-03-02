#[cfg(test)]
mod tests {
    use web3::error::TransportError;
    use web3::{Error};

    use crate::ethereum::ethereum::{get_eth_balance, ETH_HTTP_URL};
    use crate::infura::infura::HttpBuilder;
    use std::env;
    #[tokio::test]
    async fn get_ethereum_balance() {
        let address = "0x71C7656EC7ab88b098defB751B7401B5f6d8976F";

        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        let web3s = HttpBuilder::new(http_url).build();
        let eth_balance = get_eth_balance(&web3s, &address).await.unwrap();

        // The current balance of the address (at the time of writing this test) is 33 ETH
        assert_eq!(eth_balance, "33");
    }

    #[tokio::test]
    async fn try_to_get_ethereum_balance_with_invalid_balance() {
        let address = "invalid address";

        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        let web3s = HttpBuilder::new(http_url).build();

        let eth_balance = get_eth_balance(&web3s, &address).await.unwrap_err();
        let expected_error: Error = Error::InvalidResponse(r#"Invalid account address"#.to_string());

        assert!(matches!(eth_balance, expected_error));
    }

    #[tokio::test]
    async fn try_to_get_ethereum_balance_with_invalid_url() {
        let address = "0x71C7656EC7ab88b098defB751B7401B5f6d8976F";

        let http_url = "https://mainnet.infura.io/v3/invalid";
        let web3s = HttpBuilder::new(http_url.to_string()).build();

        let eth_balance = get_eth_balance(&web3s, &address).await.unwrap_err();
        
        let transport_error_expected_code = TransportError::Code(2312);
        let expected_error: Error = Error::Transport(transport_error_expected_code);
        
        // check if expected error is equal to the actual error
        assert!(matches!(expected_error, eth_balance)); // TODO: check this line
    }
}
