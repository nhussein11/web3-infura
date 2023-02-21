pub mod infura {
    use web3::{
        transports::WebSocket,
        Web3,
        Result
    };

    pub async fn connect_to_infura(ws_url: &String) -> Result<Web3<WebSocket>> {
        let websocket = WebSocket::new(ws_url).await?;
        Ok(Web3::new(websocket))
    }
}
