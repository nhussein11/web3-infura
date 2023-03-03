#[cfg(test)]
mod tests {
    use web3::error::TransportError;
    use web3::Error;

    use crate::ethereum::ethereum::{
        get_eth_balance, get_eth_blocknumber, get_eth_gasprice, ETH_HTTP_URL, ETH_WS_URL,
    };
    use crate::infura::infura::{HttpBuilder, WebSocketBuilder};
    use std::env;

    #[tokio::test]
    async fn get_ethereum_balance() {
        let address = "0x71C7656EC7ab88b098defB751B7401B5f6d8976F";

        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        let web3s = HttpBuilder::new(http_url).build().unwrap();
        let eth_balance = get_eth_balance(&web3s, &address).await.unwrap();

        // The current balance of the address (at the time of writing this test) is 33 ETH
        assert_eq!(eth_balance, "33");
    }

    #[tokio::test]
    async fn try_to_get_ethereum_balance_with_invalid_address_with_http() {
        let address = "invalid address";

        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        let web3s = HttpBuilder::new(http_url).build().unwrap();

        let eth_balance = get_eth_balance(&web3s, &address).await.unwrap_err();
        let expected_error_message: String = "Invalid account address".to_string();

        match eth_balance {
            Error::InvalidResponse(error) => {
                assert_eq!(error, expected_error_message);
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn try_to_get_ethereum_balance_with_invalid_address_with_wss() {
        let address = "invalid address";

        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let ws_url = format!("{}{}", ETH_WS_URL, api_key);
        let web3s = WebSocketBuilder::new(ws_url).build().await.unwrap();

        let eth_balance = get_eth_balance(&web3s, &address).await.unwrap_err();
        let expected_error_message: String = "Invalid account address".to_string();

        match eth_balance {
            Error::InvalidResponse(error) => {
                assert_eq!(error, expected_error_message);
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn get_ethereum_block_number() {
        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        let web3s = HttpBuilder::new(http_url).build().unwrap();

        let block_number = get_eth_blocknumber(&web3s).await.unwrap();
        // The current block number (at the time of writing this test) is 16.734.004
        let block_number = block_number.replace(".", "");
        assert!(block_number.parse::<u64>().unwrap() > 16_000_000);
    }

    #[tokio::test]
    async fn get_ethereum_gas_price() {
        dotenv::dotenv().ok();
        let api_key = &env::var("INFURA_API_KEY").unwrap();
        let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
        let web3s = HttpBuilder::new(http_url).build().unwrap();

        let gas_price = get_eth_gasprice(&web3s).await.unwrap();

        // The current gas price (at the time of writing this test) is 24.937.932.747
        let gas_price = gas_price.replace(".", "");
        assert!(gas_price.parse::<u64>().unwrap() > 24_000_000_000);
    }

    #[tokio::test]
    async fn try_to_build_connection_with_invalid_http_url() {
        let invalid_http_url = "https://mainnet.infura.io/v3/invalid".to_string();
        let web3s = HttpBuilder::new(invalid_http_url).build().unwrap();

        let gas_price = get_eth_gasprice(&web3s).await.unwrap_err();
        let transport_error_expected_code = TransportError::Code(401);

        match gas_price {
            Error::Transport(transport_error) => {
                assert_eq!(transport_error_expected_code, transport_error);
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn try_to_build_connection_with_invalid_ws_url() {
        let invalid_ws_url = "wss://mainnet.infura.io/ws/v3/invalid".to_string();
        let web3s = WebSocketBuilder::new(invalid_ws_url)
            .build()
            .await
            .unwrap_err();

        let transport_error_expected_code = TransportError::Code(401);
        match web3s {
            Error::Transport(transport_error) => {
                assert_eq!(transport_error, transport_error_expected_code);
            }
            _ => unreachable!(),
        }
    }
}
