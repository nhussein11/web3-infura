pub mod infura {
    use web3::{
        transports::{Http, WebSocket},
        Transport, Web3,
    };

    pub struct TransportBuilder<T: Transport> {
        transport: T,
    }

    impl<T: Transport> TransportBuilder<T> {
        pub fn new(transport: T) -> Self {
            TransportBuilder { transport }
        }

        pub fn build(self) -> Web3<T> {
            Web3::new(self.transport)
        }
    }

    pub struct WebSocketBuilder {
        url: String,
    }

    impl WebSocketBuilder {
        pub fn new(url: String) -> Self {
            WebSocketBuilder { url }
        }

        pub async fn build(self) -> Web3<WebSocket> {
            let transport = WebSocket::new(&self.url).await.unwrap();
            TransportBuilder::new(transport).build()
        }
    }

    pub struct HttpBuilder {
        url: String,
    }

    impl HttpBuilder {
        pub fn new(url: String) -> Self {
            HttpBuilder { url }
        }
        pub fn build(self) -> Web3<Http> {
            let transport = Http::new(&self.url).unwrap();
            TransportBuilder::new(transport).build()
        }
    }
}
