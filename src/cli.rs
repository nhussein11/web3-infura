pub mod cli {

    use crate::{
        ethereum::ethereum::{get_eth_balance, get_eth_blocknumber, get_eth_gasprice},
        infura,
    };

    use infura::infura::{ConnectionBuilder, HttpBuilder, WebSocketBuilder};

    use clap::Parser;
    use web3::{transports, Transport as Web3TransportEnum, Web3};

    #[derive(Parser, Debug)]
    #[clap(version = "1.0", author = "Author Name")]
    pub struct Commands {
        #[clap(value_enum, default_value = "http")]
        pub transport: MyTransport,

        #[clap(subcommand)]
        pub ethereum_subcommands: EthereumSubcommands,
    }
    #[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
    pub enum MyTransport {
        Http = 1,
        WebSocket = 2,
    }

    #[derive(Parser, Debug)]
    pub enum EthereumSubcommands {
        #[clap(version = "1.0", author = "Author Name")]
        Balance(Balance),
        #[clap(version = "1.0", author = "Author Name")]
        BlockNumber,
        #[clap(version = "1.0", author = "Author Name")]
        GasPrice,
    }

    #[derive(Parser, Debug)]
    pub struct Balance {
        #[clap(short, long)]
        pub address: String,
    }

    pub async fn run_cli(api_key: &String) {
        let args = Commands::parse();

        // if args.transport == MyTransport::Http {
        //     let http_url = format!("https://mainnet.infura.io/v3/{}", api_key);
        //     let web3s = HttpBuilder::new(http_url).build();
        // } else {
        //     let ws_url = format!("wss://mainnet.infura.io/ws/v3/{}", api_key);
        //     let web3s = WebSocketBuilder::new(ws_url).build().await;
        // }

        // let web3s = match args.transport {
        //     MyTransport::Http => {
        //         let http_url = format!("https://mainnet.infura.io/v3/{}", api_key);
        //         let web3s = HttpBuilder::new(http_url).build();
        //         Box::new(web3s)
        //     }
        //     MyTransport::WebSocket => {
        //         let ws_url = format!("wss://mainnet.infura.io/ws/v3/{}", api_key);
        //         let web3s = WebSocketBuilder::new(ws_url).build().await;
        //         Box::new(web3s)
        //     }
        // };

        let http_url = format!("https://mainnet.infura.io/v3/{}", api_key);
        let web3s = HttpBuilder::new(http_url).build();
        match args.ethereum_subcommands {
            EthereumSubcommands::Balance(balance) => {
                get_eth_balance(&web3s, &balance.address).await;
            }
            EthereumSubcommands::BlockNumber => {
                get_eth_blocknumber(&web3s).await;
            }
            EthereumSubcommands::GasPrice => {
                get_eth_gasprice(&web3s).await;
            }
        };
    }
}
