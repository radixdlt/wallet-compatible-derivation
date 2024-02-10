//! `saehrimnir` is a library for generating Radix Babylon Accounts.
//!
//! It derives key pairs for derivation paths using a BIP39 mnemonic
//! and an optional passphrase. You can generate up to `2_147_483_648`
//! accounts (UInt32::MAX / 2) - a seemingly endless amount of accounts,
//! acting as an inspiration for the name `saehrimnir`.
//!
//! `saehrimnir` is also the library which powers the `bacon` binary crate
//! in the same package (git repo).
//!
//! ```
//! extern crate saehrimnir;
//! use saehrimnir::prelude::*;
//!
//! // Create an hierarchical deterministic derivation path.
//! let path = AccountPath::new(
//!	    NetworkID::Mainnet, // Mainnet or Stokenet (testnet)
//!	    0 // Account Index, 0 is first.
//! );
//!
//! // 24 word BIP39 English mnemonic
//! let mnemonic: Mnemonic = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();
//!
//! // Derive Babylon Radix account...
//! let account = derive_account(&mnemonic, "radix", &path).unwrap();
//!
//! // ... containing the Account Address
//! assert_eq!(account.address, "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8");
//!
//! // ... and its private key, public key, ....
//! assert_eq!(account.private_key.to_hex(), "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003");
//! ```
//!
mod account_path;
mod bip32_path;
mod derive_account_address;
mod error;
mod mnemonic;
mod network_id;

pub mod prelude {
    pub use crate::account_path::*;
    pub use crate::bip32_path::*;
    pub use crate::derive_account_address::*;
    pub use crate::error::*;
    pub use crate::mnemonic::*;
    pub use crate::network_id::*;

    pub(crate) use std::str::FromStr;
}

pub use prelude::*;
