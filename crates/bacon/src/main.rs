mod config;
mod read_config_from_stdin;
use crate::config::Config;
use crate::read_config_from_stdin::*;

use clap::{Parser, Subcommand};

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

    /// If the PrivateKey of derived accounts is included in output.
    #[arg(short, long, default_value_t = false)]
    pub(crate) include_private_key: bool,
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

    let include_private_key = cli.include_private_key;
    let mut zeroized_accounts = true;

    let start = config.start;
    let count = config.count as u32;
    let end = start + count;
    for index in (Range { start, end }) {
        let account_path = AccountPath::new(&config.network, index);
        let mut account = Account::derive(&config.mnemonic, &config.passphrase, &account_path);
        print_account(&account, include_private_key);
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

fn print_account(account: &Account, include_private_key: bool) {
    let delimiter = "✨".repeat(WIDTH);
    let header_delimiter = "🔮".repeat(WIDTH);
    let header = ["✅ CREATED ACCOUNT ✅", &header_delimiter].join("\n");
    let account_string = account.to_string_include_private_key(include_private_key);
    let output = [
        delimiter.clone(),
        header,
        format!("{account_string}"),
        delimiter,
    ]
    .join("\n");
    println!("\n{output}");
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
