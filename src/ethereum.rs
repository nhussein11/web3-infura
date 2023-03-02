pub mod ethereum {
    use std::convert::TryInto;
    use std::fmt::Display;
    use std::str::FromStr;

    use web3::{
        types::{H160, U256},
        Error, Transport, Web3,
    };

    use num_format::{Locale, ToFormattedString};

    pub const ETH_HTTP_URL: &str = "https://mainnet.infura.io/v3/";
    pub const ETH_WS_URL: &str = "wss://mainnet.infura.io/ws/v3/";

    pub async fn get_eth_balance<T: Transport>(
        transport: &Web3<T>,
        account_address: &str,
    ) -> Result<String, Error> {
        let account = H160::from_str(account_address);
        if account.is_err() {
            return Err(Error::InvalidResponse(
                "Invalid account address".to_string(),
            ));
        }
        let account = account.unwrap();

        let balance = transport.eth().balance(account, None).await;
        match balance {
            Ok(balance) => {
                let converted_balance = wei_to_eth(balance);
                println!("Balance: {} [ETH]", format_unit_integer(converted_balance));
                Ok(format_unit_integer(converted_balance))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn get_eth_blocknumber<T: Transport>(transport: &Web3<T>) -> Result<String, Error> {
        let block_number = transport.eth().block_number().await;
        match block_number {
            Ok(block_number) => {
                let block_number_formatted = format_unit_integer(block_number);
                Ok(block_number_formatted)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn get_eth_gasprice<T: Transport>(transport: &Web3<T>) -> Result<String, Error> {
        let gas_price = transport.eth().gas_price().await;
        match gas_price {
            Ok(gas_price) => {
                let gas_price_formatted = format_unit_integer(gas_price);
                Ok(gas_price_formatted)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    // Helper functions
    fn wei_to_eth(wei: U256) -> U256 {
        let wei_conv: U256 = U256::exp10(18);
        wei.checked_div(wei_conv).unwrap()
    }
    fn wei_to_gwei(wei: U256) -> U256 {
        let wei_conv: U256 = U256::exp10(9);
        wei.checked_div(wei_conv).unwrap()
    }
    fn format_unit_integer<T: TryInto<u64> + Display + Copy>(integer: T) -> String {
        match integer.try_into() {
            Ok(n) => n.to_formatted_string(&Locale::es_AR),
            Err(_) => format!("Conversion to u64 failed for input: {}", integer),
        }
    }
}
