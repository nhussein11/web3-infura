pub mod ethereum {
    use std::{ops::Div, str::FromStr};

    use web3::{
        types::{H160, U256},
        Transport, Web3,
    };

    use num_format::{Locale, ToFormattedString};

    pub async fn get_eth_balance<T: Transport>(
        transport: &Web3<T>,
        account_address: &String,
    ) -> () {
        let account: H160 = H160::from_str(account_address).unwrap();

        let balance = transport.eth().balance(account, None).await.unwrap();
        let converted_balance = wei_to_eth(balance);
        println!("Balance: {} [ETH]", format_unit_integer(converted_balance));
    }

    pub async fn get_eth_blocknumber<T: Transport>(transport: &Web3<T>) -> () {
        let block_number = transport.eth().block_number().await.unwrap();
        // let block_number_formatted = format_unit_integer(block_number);
        println!("Block number: {}", block_number);
    }

    pub async fn get_eth_gasprice<T: Transport>(transport: &Web3<T>) -> () {
        let gas_price: U256 = transport.eth().gas_price().await.unwrap();
        println!("Gas price: {} [GWEI]", wei_to_gwei(gas_price));
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
    
    fn format_unit_integer<T>(value: T) -> String
    where
        T: Into<U256>,
    {
        let integer: U256 = value.into();
        integer.as_u64().to_formatted_string(&Locale::es_AR)
    }
}
