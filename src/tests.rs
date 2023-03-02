#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn get_gas_price_using_ws() {
        let mut binding = Command::new("cargo");
        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            .arg("gas-price");

        // get INFURA_API_KEY from environment variable

        dotenv::dotenv().ok();
        let infura_api_key = std::env::var("INFURA_API_KEY").unwrap();
        println!("INFURA_API_KEY: {}", infura_api_key);

        let output = command.output().unwrap();
        assert!(output.status.success());
        // assert!(false);
    }
}
