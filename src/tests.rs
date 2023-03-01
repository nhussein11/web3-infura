// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn get_gas_price_using_ws() {
        let mut binding = Command::new("cargo");
        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            .arg("gas-price");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn get_gas_price_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding.arg("run").arg("--").arg("http").arg("gas-price");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }
}
