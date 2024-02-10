use clap::{Parser, Subcommand};
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
    /// The mnemonic you wanna use to derive accounts with.
    #[arg(short = 'm', long = "mnemonic")]
    mnemonic: String,

    /// An optional BIP39 passphrase.
    #[arg(short = 'p', long = "passphrase")]
    passphrase: Option<String>,

    /// The Network you want to derive accounts on.
    #[arg(short = 'n', long = "network", default_value_t = 1)]
    network: u32,

    /// The account index
    #[arg(short = 'i', long = "index", default_value_t = 0)]
    index: u32,
}

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
    // let account = cli
}
