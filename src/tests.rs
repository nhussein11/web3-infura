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
    fn try_to_get_gas_price_using_ws() {
        let mut binding = Command::new("cargo");
        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            // Use invalid argument
            .arg("gas-pri");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }

    #[test]
    fn get_gas_price_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding.arg("run").arg("--").arg("http").arg("gas-price");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn try_to_get_gas_price_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("http")
            // Use invalid argument
            .arg("gas-pri");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }

    #[test]
    fn get_balance_using_ws() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            .arg("balance")
            .arg("--address")
            .arg("0x71C7656EC7ab88b098defB751B7401B5f6d8976F");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn try_to_get_balance_using_ws() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            .arg("balance")
            // Use invalid argument
            .arg("--addres")
            .arg("0x71C7656EC7ab88b098defB751B7401B5f6d8976F");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }
}
