//! `saehrimnir` is a library for generating Radix Babylon Accounts.
//!
//! It derives key pairs for derivation paths using a BIP39 mnemonic
//! and an optional passphrase. You can generate up to `2_147_483_648`
//! accounts (UInt32::MAX / 2) - a seemingly endless amount of accounts,
//! acting as an inspiration for the name `saehrimnir`.
//!
//! `saehrimnir` is also the library which powers the binary `bacon` packaged
//! in the same git repo.
///
/// ```
/// extern crate saehrimnir;
/// use saehrimnir::prelude::*;
///
/// let path = AccountPath::new(NetworkID::Mainnet, 0);
///
/// let mnemonic: Mnemonic = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();
///
/// let account = derive_account(&mnemonic, "radix", &path).unwrap();
///
/// assert_eq!(account.private_key.to_hex(), "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003");
/// assert_eq!(account.address, "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8");
/// ```
///
mod account_path;
mod bip32_path;
mod derive_account_address;
mod error;
mod network_id;

pub mod prelude {
    pub use crate::account_path::*;
    pub use crate::bip32_path::*;
    pub use crate::derive_account_address::*;
    pub use crate::error::*;
    pub use crate::network_id::*;

    #[allow(unused_imports)]
    pub use bip39::Mnemonic;

    pub(crate) use std::str::FromStr;
}

pub use prelude::*;
