/*
I'm aware that is not a good practice to do this kind of "unit tests" (spawing subprocess and
checking the output), but I'm doing this just to learn how to do it.

The idea is to have a test for each command, just showing that the command is working.
*/
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
            // Use invalid argument (use "gas-pri" instead of "gas-price")
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
            // Use invalid argument (use "addres" instead of "address")
            .arg("--addres")
            .arg("0x71C7656EC7ab88b098defB751B7401B5f6d8976F");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }

    #[test]
    fn get_balance_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("http")
            .arg("balance")
            .arg("--address")
            .arg("0x71C7656EC7ab88b098defB751B7401B5f6d8976F");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn try_to_get_balance_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("http")
            .arg("balance")
            // Use invalid argument (use "addres" instead of "address")
            .arg("--addres")
            .arg("0x71C7656EC7ab88b098defB751B7401B5f6d8976F");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }

    #[test]
    fn get_block_number_using_ws() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            .arg("block-number");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn try_to_get_block_number_using_ws() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("web-socket")
            // Use invalid argument (use "block-numbe" instead of "block-number")
            .arg("block-numbe");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }

    #[test]
    fn get_block_number_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding.arg("run").arg("--").arg("http").arg("block-number");

        let output = command.output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn try_to_get_block_number_using_http() {
        let mut binding = Command::new("cargo");

        let command = binding
            .arg("run")
            .arg("--")
            .arg("http")
            // Use invalid argument (use "block-numbe" instead of "block-number")
            .arg("block-numbe");

        let output = command.output().unwrap();
        assert!(!output.status.success());
    }
}
