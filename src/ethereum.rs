pub mod ethereum {
    use std::str::FromStr;

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

        let wei_conv: U256 = U256::exp10(18);
        println!(
            "ETH balance: {} ETH",
            balance.checked_div(wei_conv).unwrap()
        );
    }

    pub async fn get_eth_blocknumber<T: Transport>(transport: &Web3<T>) -> () {
        let block_number = transport
            .eth()
            .block_number()
            .await
            .unwrap()
            .as_u64()
            .to_formatted_string(&Locale::es_AR);
        println!("ETH block number: {}", block_number);
    }

    pub async fn get_eth_gasprice<T: Transport>(transport: &Web3<T>) -> () {
        let gas_price = transport
            .eth()
            .gas_price()
            .await
            .unwrap()
            .as_u64()
            .to_formatted_string(&Locale::es_AR);
        println!("ETH gas price: {}", gas_price);
    }
}
