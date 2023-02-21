pub mod infura {
    use web3::{
        transports::{WebSocket, Http},
        Web3,
        Result
    };

    pub async fn connect_to_infura_by_websocket(ws_url: &String) -> Result<Web3<WebSocket>> {
        let websocket = WebSocket::new(ws_url).await?;
        Ok(Web3::new(websocket))
    }

    pub async fn connect_to_infura_by_http(http_url: &String) -> Result<Web3<Http>> {
        let http = Http::new(http_url)?;
        Ok(Web3::new(http))
    }
}
