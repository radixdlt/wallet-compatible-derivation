use clap::{Args, Parser, Subcommand};
use saehrimnir::prelude::*;

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
    command: Commands,
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
    #[arg(short = 'n', long = "network", value_parser = NetworkID::from_str, default_value_t = NetworkID::Mainnet)]
    network: NetworkID,

    /// The account index
    #[arg(short = 'i', long = "index", default_value_t = 0)]
    index: u32,
}

use inquire::{
    ui::{Color, RenderConfig, Styled},
    validator::{StringValidator, Validation},
    CustomType, Editor, Text,
};

use pager::Pager;
use std::str::FromStr;

fn description_render_config() -> RenderConfig {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}

fn read_config_from_stdin() -> Result<Config> {
    let mnemonic = CustomType::<Mnemonic>::new("Input mnemonic: ")
        .with_formatter(&|m| format!("{}", m))
        .with_error_message("Please type a valid mnemonic")
        .with_help_message("Only English 24 word mnemonics are supported.")
        .prompt()
        .map_err(|_| Error::InvalidMnemonic)?;

    Ok(Config {
        mnemonic,
        passphrase: "radix".to_owned(),
        network: NetworkID::Mainnet,
        index: 0,
    })
}

fn main() {
    let cli = Cli::parse();
    let config = match cli.command {
        Commands::NoPager(c) => {
            Pager::new().setup();
            Ok(c)
        }
        Commands::Pager => {
            Pager::new().setup();
            read_config_from_stdin()
        }
    };
    println!("{:?}", config);
}
