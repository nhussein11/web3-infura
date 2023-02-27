pub mod cli {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(version = "1.0", author = "Author Name")]
    pub struct Commands {
        #[clap(subcommand)]
        pub subcmd: SubCommand,
    }

    #[derive(Parser, Debug)]
    pub enum SubCommand {
        #[clap(version = "1.0", author = "Author Name")]
        Ethereum(Ethereum),
        #[clap(version = "1.0", author = "Author Name")]
        Infura(Infura),
    }

    #[derive(Parser, Debug)]
    pub struct Ethereum {
        #[clap(subcommand)]
        pub subcmd: EthereumSubCommand,
    }

    #[derive(Parser, Debug)]
    pub enum EthereumSubCommand {
        #[clap(version = "1.0", author = "Author Name")]
        Balance(Balance),
        #[clap(version = "1.0", author = "Author Name")]
        BlockNumber(BlockNumber),
        #[clap(version = "1.0", author = "Author Name")]
        GasPrice(GasPrice),
    }

    #[derive(Parser, Debug)]
    pub struct Balance {
        #[clap(short, long)]
        pub address: String,
    }

    #[derive(Parser, Debug)]
    pub struct BlockNumber {}

    #[derive(Parser, Debug)]
    pub struct GasPrice {}

    #[derive(Parser, Debug)]
    pub struct Infura {
        #[clap(subcommand)]
        pub subcmd: InfuraSubCommand,
    }

    #[derive(Parser, Debug)]
    pub enum InfuraSubCommand {
        #[clap(version = "1.0", author = "Author Name")]
        Http(Http),
        #[clap(version = "1.0", author = "Author Name")]
        WebSocket(WebSocket),
    }

    #[derive(Parser, Debug)]
    pub struct Http {
        #[clap(short, long)]
        pub url: String,
    }

    #[derive(Parser, Debug)]
    pub struct WebSocket {
        #[clap(short, long)]
        pub url: String,
    }

    pub fn run() -> () {
        let args = Commands::parse();
        println!("{:?}", args);
    }
}
