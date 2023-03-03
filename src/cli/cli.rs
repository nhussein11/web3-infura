pub mod cli {
    use crate::{
        ethereum::ethereum::{
            get_eth_balance, get_eth_blocknumber, get_eth_gasprice, ETH_HTTP_URL, ETH_WS_URL,
        },
        infura::infura::{HttpBuilder, WebSocketBuilder},
    };

    use clap::Parser;
    use web3::{Transport as Web3Transport, Web3};

    #[derive(Parser, Debug)]
    #[clap(version = "1.0", author = "nhussein11")]
    pub struct Commands {
        #[clap(value_enum, default_value = "http")]
        pub transport: Transport,

        #[clap(subcommand)]
        pub ethereum_subcommands: EthereumSubcommands,
    }
    #[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
    pub enum Transport {
        Http,
        WebSocket,
    }

    #[derive(Parser, Debug)]
    pub enum EthereumSubcommands {
        #[clap(version = "1.0", author = "nhussein11")]
        Balance(Balance),
        #[clap(version = "1.0", author = "nhussein11")]
        BlockNumber,
        #[clap(version = "1.0", author = "nhussein11")]
        GasPrice,
    }

    #[derive(Parser, Debug)]
    pub struct Balance {
        #[clap(short, long)]
        pub address: String,
    }

    pub async fn run_cli(api_key: &String) {
        let args = Commands::parse();

        match args.transport {
            Transport::Http => {
                let http_url = format!("{}{}", ETH_HTTP_URL, api_key);
                let web3s = HttpBuilder::new(http_url).build();
                run_ethereum_subcommands(args.ethereum_subcommands, &web3s).await;
            }
            Transport::WebSocket => {
                let ws_url = format!("{}{}", ETH_WS_URL, api_key);
                // TODO: Handle error
                let web3s = WebSocketBuilder::new(ws_url).build().await;
                match web3s {
                    Ok(web3s) => run_ethereum_subcommands(args.ethereum_subcommands, &web3s).await,
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }

    async fn run_ethereum_subcommands<T: Web3Transport>(
        args: EthereumSubcommands,
        web3s: &Web3<T>,
    ) {
        match args {
            EthereumSubcommands::Balance(balance) => {
                get_eth_balance(web3s, &balance.address).await;
            }
            EthereumSubcommands::BlockNumber => {
                get_eth_blocknumber(web3s).await;
            }
            EthereumSubcommands::GasPrice => {
                get_eth_gasprice(web3s).await;
            }
        };
    }
}
