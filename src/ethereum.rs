
pub mod ethereum {
    use std::str::FromStr;

    use web3::{
        Web3,
        transports::WebSocket,
        types::{U256, H160}
    };

    pub async fn get_eth_balance(wss: Web3<WebSocket>, account_address: &String ) -> () {
        let account : H160 = H160::from_str(account_address).unwrap();
        let balance = wss.eth().balance(account, None).await.unwrap();
    
        let wei_conv: U256 = U256::exp10(18);

        println!(
            "ETH balance: {} ETH",
            balance.checked_div(wei_conv).unwrap()
        );
    }
}
