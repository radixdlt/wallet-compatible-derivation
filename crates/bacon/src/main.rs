use clap::{Args, Parser, Subcommand};
use inquire::{CustomType, Password, Select};
use saehrimnir::prelude::*;

use pager::Pager;
use std::{str::FromStr, thread, time};

#[derive(Parser)]
#[command(name = "bacon", version)]
#[command(
about = "Babylon Account CreatiON.",
long_about = format!(r#"
Generate Radix Babylon accounts - private (and public) keys and addresses given a mnemonic, Network ID (Mainnet/Stokenet) and indices.
"#)
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    NoPager(Config),
    Pager,
}

#[derive(Debug, Args)]
struct Config {
    /// The mnemonic you wanna use to derive accounts with.
    #[arg(short = 'm', long = "mnemonic", value_parser = Mnemonic::from_str)]
    mnemonic: Mnemonic,

    /// An optional BIP39 passphrase.
    #[arg(short = 'p', long = "passphrase", default_value_t = String::new())]
    passphrase: String,

    /// The Network you want to derive accounts on.
    #[arg(short = 'n', long = "network_id", value_parser = NetworkID::from_str, default_value_t = NetworkID::Mainnet)]
    network_id: NetworkID,

    /// The account index
    #[arg(short = 'i', long = "index", default_value_t = 0)]
    index: u32,
}
impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mnemonic: {}", self.mnemonic)?;
        writeln!(f, "Passphrase: {}", self.passphrase)?;
        writeln!(f, "NetworkID: {}", self.network_id)?;
        writeln!(f, "Index: {}", self.index)?;
        Ok(())
    }
}

fn read_config_from_stdin() -> Result<Config> {
    let mnemonic = CustomType::<Mnemonic>::new("Input mnemonic: ")
        .with_formatter(&|m| format!("{}", m))
        .with_error_message("Please type a valid mnemonic")
        .with_help_message("Only English 24 word mnemonics are supported.")
        .prompt()
        .map_err(|_| Error::InvalidMnemonic)?;

    let passphrase = Password::new("Passphrase (can be empty):")
        .prompt()
        .unwrap();

    let network_id: NetworkID = Select::new("Choose Network ID", NetworkID::all())
        .prompt()
        .expect("Should not be possible to select in invalid network id");

    let index = CustomType::<HDPathComponentValue>::new("Account index: ")
        .with_formatter(&|i| format!("{}H", i))
        .with_error_message("Please type a valid non negative integer")
        .with_help_message("Only non negative integers <= 2,147,483,648 are allowed")
        .prompt()
        .expect("Should not be possible to input an invalid u32");

    Ok(Config {
        mnemonic,
        passphrase,
        network_id,
        index,
    })
}

fn paged() {
    Pager::new().setup();

    // Pager setup is a bit slow, if we don't add this terribly ugly hacky
    // sleep, the output of inquire is not shown.
    thread::sleep(time::Duration::from_millis(250));
}

fn main() {
    let cli = Cli::parse();
    let config = match cli.command.unwrap_or(Commands::Pager) {
        Commands::NoPager(c) => Ok(c),
        Commands::Pager => {
            paged();
            read_config_from_stdin()
        }
    }
    .expect("Valid config");

    println!("Input config: {}", config);
    let account_path = AccountPath::new(config.network_id, config.index);
    let account = derive_account(&config.mnemonic, config.passphrase, &account_path);
    println!("Account:\n{}", account);
}
