pub mod cli {

    use crate::{
        ethereum::ethereum::{get_eth_balance, get_eth_blocknumber, get_eth_gasprice},
        infura,
    };

    use infura::infura::{HttpBuilder, WebSocketBuilder};

    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(version = "1.0", author = "Author Name")]
    pub struct Commands {
        #[clap(subcommand)]
        pub ethereum_subcommands: EthereumSubcommands,
    }

    // #[derive(Parser, Debug)]
    // pub enum BlockchainSubcommands {
    //     #[clap(version = "1.0", author = "Author Name")]
    //     Ethereum(EthereumSubcommands),
    // }

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

    pub async fn run_cli(api_key: &String) -> () {
        let http_url = format!("https://mainnet.infura.io/v3/{}", api_key);
        let web3s = HttpBuilder::new(http_url).build();

        // let wss_url = format!("wss://mainnet.infura.io/ws/v3/{}", api_key);
        // let web3s = WebSocketBuilder::new(wss_url).build().await;

        let args = Commands::parse();
        println!("{:?}", args);

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
