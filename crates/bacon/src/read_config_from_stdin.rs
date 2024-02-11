use crate::config::Config;
use inquire::{CustomType, Password, Select};
use saehrimnir::prelude::*;

pub(crate) fn read_config_from_stdin() -> Result<Config> {
    let mnemonic = CustomType::<Mnemonic24Words>::new("Input mnemonic: ")
        .with_formatter(&|m| format!("{}", m))
        .with_error_message("Please type a valid mnemonic")
        .with_help_message("Only English 24 word mnemonics are supported.")
        .prompt()
        .map_err(|_| Error::InvalidMnemonic)?;

    let passphrase = Password::new("Passphrase (can be empty):")
        .prompt()
        .unwrap();

    let network: NetworkID = Select::new("Choose Network", NetworkID::all())
        .prompt()
        .expect("Should not be possible to select in invalid network id");

    let start = CustomType::<HDPathComponentValue>::new("Account index start: ")
        .with_formatter(&|i| format!("{}", i))
        .with_error_message("Only non negative integers <= 2,147,483,648 are allowed")
        .with_help_message("Normally you want to start at index `0`.")
        .prompt()
        .expect("Should not be possible to input an invalid u32");

    let count = CustomType::<u8>::new("Number of accounts to derive: ")
        .with_formatter(&|i| format!("#{}", i))
        .with_error_message("Only non negative integers <= 255 are allowed")
        .with_help_message("If you need more than 255 to be derived, let us know!.")
        .prompt()
        .expect("Should not be possible to input an invalid u8");

    Ok(Config {
        mnemonic,
        passphrase,
        network,
        start,
        count,
    })
}
