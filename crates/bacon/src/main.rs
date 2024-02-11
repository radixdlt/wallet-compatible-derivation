mod config;
use crate::config::Config;

use clap::{Parser, Subcommand};
use inquire::{CustomType, Password, Select};
use saehrimnir::prelude::*;

use pager::Pager;
use std::{ops::Range, thread, time};
use zeroize::Zeroize;

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
impl Commands {
    fn is_using_pager(&self) -> bool {
        match self {
            Commands::NoPager(_) => false,
            Commands::Pager => true,
        }
    }
}

fn read_config_from_stdin() -> Result<Config> {
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
        .with_formatter(&|i| format!("{}H", i))
        .with_error_message("Only non negative integers <= 2,147,483,648 are allowed")
        .with_help_message("Normally you want to start at index `0`.")
        .prompt()
        .expect("Should not be possible to input an invalid u32");

    let count = CustomType::<u8>::new("Number of accounts to derive: ")
        .with_formatter(&|i| format!("{}H", i))
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

fn paged() {
    Pager::new().setup();

    // Pager setup is a bit slow, if we don't add this terribly ugly hacky
    // sleep, the output of inquire is not shown.
    thread::sleep(time::Duration::from_millis(250));
}

fn main() {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Commands::Pager);
    let use_pager = command.is_using_pager();
    let mut config = match command {
        Commands::NoPager(c) => Ok(c),
        Commands::Pager => {
            paged();
            read_config_from_stdin()
        }
    }
    .expect("Valid config");

    let mut zeroized_accounts = true;

    let start = config.start;
    let count = config.count as u32;
    let end = start + count;
    for index in (Range { start, end }) {
        let account_path = AccountPath::new(&config.network, index);
        let mut account = Account::derive(&config.mnemonic, &config.passphrase, &account_path);
        print_account(&account);
        account.zeroize();
        zeroized_accounts &= account.is_zeroized();
    }

    config.zeroize();

    if use_pager && config.mnemonic.is_zeroized() && zeroized_accounts {
        print_secrets_safe()
    }

    drop(config);
}

const WIDTH: usize = 50;

fn print_account(account: &Account) {
    let delimiter = "✨".repeat(WIDTH);
    let header_delimiter = "🔮".repeat(WIDTH);
    let header = ["✅ CREATED ACCOUNT ✅", &header_delimiter].join("\n");
    let new_account_string =
        [delimiter.clone(), header, format!("{account}"), delimiter].join("\n");
    println!("\n{new_account_string}");
}

fn print_secrets_safe() {
    let delimiter = "🛡️ ".repeat(WIDTH);
    let safe = [
        "\n\n",
        &delimiter,
        "🔐 All sensitive data have been zeroized, your secrets are safe 🔐",
        &delimiter,
    ]
    .join("\n");
    println!("{safe}")
}
