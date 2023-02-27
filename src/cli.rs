pub mod cli {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(version = "1.0", author = "Author Name")]
    pub struct Commands {
        #[clap(subcommand)]
        pub ethsubcommands: EthereumSubcommands,
    }

    #[derive(Parser, Debug)]
    pub enum EthereumSubcommands {
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

    pub fn run_cli() -> () {
        let args = Commands::parse();
        println!("{:?}", args);
    }
}
