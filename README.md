# Web 3 - Infura :boom:

This project is a simple example of how to use Infura to connect to the Ethereum blockchain. It uses the Infura API to get the latest block number and the latest block hash. To get the infura API key, you need to create an account on [Infura](https://infura.io/). After creating an account, you can create a new project and get the API key.

As it is a simple example, I added only some of the available methods in the Infura API. You can find the full list of the available methods [here](https://docs.infura.io/infura/networks/ethereum/json-rpc-methods).

### Why did I create this project? :books:

The main reason is that I wanted to learn more about Rust and I thought that creating a CLI to get information about the Ethereum blockchain would be a good idea. I also wanted to learn more about the Rust ecosystem and how to use some of the available crates. I used the following crates in this project:

- [clap](https://crates.io/crates/clap): to parse the CLI arguments
- [dotenv](https://crates.io/crates/dotenv): to load the environment variables from the .env file
- [tokio](https://crates.io/crates/tokio): to make HTTP requests
- [web3](https://crates.io/crates/web3): to connect to the Ethereum blockchain
- [num-format](https://crates.io/crates/num-format): to format the numbers

### What's next? :100:

If I have more time, I will add more features to this project. My idea is to use the CLI to get information not only about the Ethereum blockchain but also about other blockchains available on Infura, that's why I separated the project into different modules, so it will be easier to add more features tn the future hypothetically. For instance, if I want to add a module that to get information about NEAR blockchain, probably I will create a new module called `near` and there I would add the necessary code to get the desired data.

Having that in mind, **_if you want to contribute to this project, obviously you are more than welcome_**. Feel free to add more features to the CLI or suggest some ideas for new features.

## How to run :runner:

1. Clone the project:

```bash
git clone https://github.com/nhussein11/web3-infura.git
```

2. Create a .env file and add the infura API key with the following format:

```bash
INFURA_API_KEY=<YOUR_API_KEY>
```

3. Run the CLI (to see some of the available commands, you can run the following command):

```bash
cargo run -- --help
```

At the moment, the CLI only supports the following commands:

- `balance` : get the balance of an address
- `block-number`: get the latest block number
- `get-price`: get the current price of Ether in USD

This CLI also provides the option to specify the transport protocol. The default protocol is HTTP. You can also use WebSockets. To use WebSockets, you need to add the `web-socket` flag when running the CLI.

## How to test :white_check_mark:

I wrote some unit test for this project. It's important to mention that some tests, like the ones that depends on real time variation, are approachs in fact.

- To run all of them, you just need to run the following command:

```bash
cargo test
```

- If you want to run a specific test, you can run the following command:

```bash
cargo test <test_name>
```

## Examples :crab:

Let's see some examples:

- Get the balance of the following address, using HTTP

```bash
cargo run -- balance --address 0x71C7656EC7ab88b098defB751B7401B5f6d8976F
```

- Get the balance of the following address, using WebSockets

```bash
cargo run -- web-socket balance --address 0x71C7656EC7ab88b098defB751B7401B5f6d8976F
```

- Get the latest block number, using HTTP

```bash
cargo run -- block-number
```

- Get the current gas price, using WebSockets

```bash
cargo run -- web-socket gas-price
```

And that's it. I hope you find this project useful. If you have any questions, feel free to contact me :rocket:
