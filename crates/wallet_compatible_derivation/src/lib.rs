//! `wallet_compatible_derivation` is a library for generating Radix Babylon Accounts.
//!
//! It derives key pairs for derivation paths using a BIP-39 mnemonic
//! and an optional passphrase. You can generate up to `2_147_483_648`
//! accounts (UInt32::MAX / 2) - a seemingly endless amount of accounts,
//! acting as an inspiration for the name `wallet_compatible_derivation`.
//!
//! `wallet_compatible_derivation` is also the library which powers the `wallet_compatible_derivation_cli` binary crate
//! in the same package (git repo).
//!
//! ```
//! extern crate wallet_compatible_derivation;
//! use wallet_compatible_derivation::prelude::*;
//!
//! // Create an hierarchical deterministic derivation path.
//! let path = AccountPath::new(
//!	    &NetworkID::Mainnet, // Mainnet or Stokenet (testnet)
//!	    0 // Account Index, 0 is first.
//! );
//!
//! // 24 word BIP-39 English mnemonic
//! let mnemonic: Mnemonic24Words = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();
//!
//! // Derive Babylon Radix account...
//! let account = Account::derive(&mnemonic, "radix", &path);
//!
//! // ... containing the Account Address
//! assert_eq!(account.address, "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8");
//!
//! // ... and its private key, public key, ....
//! assert_eq!(account.private_key.to_hex(), "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003");
//!
//! // continue with next index, `1` to derive next account
//! let path_of_next_account = AccountPath::new(
//! 	&NetworkID::Mainnet,
//! 	1 // `1` comes after `0` (breaking news!)
//! );
//! let second_account = Account::derive(
//! 	&mnemonic,
//! 	"radix",
//! 	&path_of_next_account
//! );
//! // Next address...
//! assert_eq!(second_account.address, "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69");
//! ```
//!
mod account;
mod account_path;
mod bip32_path;
mod derive_account_address;
mod derive_key_pair;
mod error;
mod factor_source_id;
mod mnemonic_24words;
mod network_id;
mod to_hex;

pub mod prelude {
    pub use crate::account::*;
    pub use crate::account_path::*;
    pub use crate::bip32_path::*;

    pub use crate::error::*;
    pub use crate::factor_source_id::*;
    pub use crate::mnemonic_24words::*;
    pub use crate::network_id::*;
    pub use crate::to_hex::*;

    pub(crate) use crate::derive_account_address::*;
    pub(crate) use crate::derive_key_pair::*;
    pub(crate) use std::str::FromStr;
    pub(crate) use zeroize::{Zeroize, ZeroizeOnDrop};
}

pub use prelude::*;
