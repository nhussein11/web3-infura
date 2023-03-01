# Web 3 - Infura

This project is a simple example of how to use Infura to connect to the Ethereum blockchain. It uses the Infura API to get the latest block number and the latest block hash. To get the infura API key, you need to create an account on [Infura](https://infura.io/). After creating an account, you can create a new project and get the API key.

As it is a simple example, I added only some of the available methods in the Infura API. You can find the full list of the available methods [here](https://docs.infura.io/infura/networks/ethereum/json-rpc-methods).

I also want to mention that this project is not a library, it is just a simple CLI that uses the Infura API to get some information about the Ethereum blockchain. If I have time, I will add more features to this project. My idea is to use the CLI to get information not only about the Ethereum blockchain but also about other blockchains available on Infura.

Having that in mind, if you want to contribute to this project, you are more than welcome. You can add more features to the CLI or you can suggest some ideas for new features.

## How to run

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
-  `balance` : get the balance of an address
-  `block-number`: get the latest block number
-  `get-price`: get the current price of Ether in USD

This CLI also provides the option to specify the transport protocol. The default protocol is HTTP. You can also use WebSockets. To use WebSockets, you need to add the `web-socket` flag when running the CLI.


### Examples

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




And that's it. I hope you find this project useful. If you have any questions, feel free to contact me :rocket: .