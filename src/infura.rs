pub mod infura {
    use web3::{
        transports::{Http, WebSocket},
        Result, Transport, Web3,
    };

    pub struct Web3Builder {
        transport: Box<dyn Transport>,
    }

    impl Web3Builder {
        pub fn new(transport: Box<dyn Transport>) -> Self {
            Self { transport }
        }

        pub fn build(self) -> Web3<Self::Transport> {
            Web3::new(self.transport)
        }
    }

    pub async fn connect_to_infura_by_websocket(ws_url: &String) -> Result<Web3<WebSocket>> {
        let websocket = WebSocket::new(ws_url).await?;
        Ok(Web3::new(websocket))
    }

    pub async fn connect_to_infura_by_http(http_url: &String) -> Result<Web3<Http>> {
        let http = Http::new(http_url)?;
        Ok(Web3::new(http))
    }
}
