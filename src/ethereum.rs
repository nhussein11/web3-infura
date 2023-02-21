pub mod ethereum {
    use std::str::FromStr;

    use web3::{
        Transport,
        types::{H160, U256},
        Web3,
    };

    pub async fn get_eth_balance<T: Transport>(transport: Web3<T>, account_address: &String) -> () {
        let account: H160 = H160::from_str(account_address).unwrap();
        let balance = transport.eth().balance(account, None).await.unwrap();

        let wei_conv: U256 = U256::exp10(18);

        println!(
            "ETH balance: {} ETH",
            balance.checked_div(wei_conv).unwrap()
        );
    }
}
