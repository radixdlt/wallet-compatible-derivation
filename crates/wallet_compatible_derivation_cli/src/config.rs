use clap::Args;
use wallet_compatible_derivation::prelude::*;

use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A run configuration for the binary `wallet_compatible_derivation_cli`.
///
/// Contains secrets, thus it implements `Zeroize`.
///
/// As soon as this run configuration is no longer needed, it should be zeroized
/// and dropped.
#[derive(Debug, Args, Zeroize, ZeroizeOnDrop)]
pub(crate) struct Config {
    /// The mnemonic you wanna use to derive accounts with.
    #[arg(
        short = 'm',
        long = "mnemonic", 
        help = "The BIP-39 Mnemonic ('Seed Phrase') used to derive the accounts. Must be a 24 word English Mnemonic.", value_parser = Mnemonic24Words::from_str
    )]
    pub(crate) mnemonic: Mnemonic24Words,

    /// An optional BIP-39 passphrase.
    #[arg(short = 'p', long = "passphrase", help = "Advanced: An optional BIP-39 passphrase, use the empty string if you don't need one. Often referred to as 'the 25th word'. For extra security.", default_value_t = String::new())]
    pub(crate) passphrase: String,

    /// The Network you want to derive accounts on.
    #[arg(short = 'n', long = "network", help = "The ID of the Radix Network the derived accounts should be used with.", value_parser = NetworkID::from_str, default_value_t = NetworkID::Mainnet)]
    #[zeroize(skip)]
    pub(crate) network: NetworkID,

    /// The start account index
    #[arg(
        short = 's',
        long = "start",
        help = "The start account index to derive the first account at.",
        default_value_t = 0
    )]
    pub(crate) start: u32,

    /// The number of accounts to derive.
    #[arg(
        short = 'c',
        long = "count",
        help = "The number of accounts to derive, starting at `index`. Max 255.",
        default_value_t = 2
    )]
    pub(crate) count: u8,
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString},
        mem,
        ops::Range,
    };

    use super::*;

    #[test]
    fn zeroize_config() {
        let mut config = Config {
            mnemonic: Mnemonic24Words::from_str("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote").unwrap(),
            passphrase: "radix".to_owned(),
            network: NetworkID::Mainnet,
            start: 0,
            count: 1,
        };

        let mnemonic_view = &config.mnemonic as *const _ as *const u8;
        let mnemonic_range = Range {
            start: 0,
            end: mem::size_of::<Mnemonic24Words>() as isize,
        };
        for i in mnemonic_range.clone() {
            assert_eq!(unsafe { *mnemonic_view.offset(i) }, 0xff);
        }

        let passphrase_ptr = CString::new(config.passphrase.as_str())
            .unwrap()
            .as_c_str()
            .as_ptr();

        config.zeroize();

        for i in mnemonic_range.clone() {
            assert_eq!(unsafe { *mnemonic_view.offset(i) }, 0x00);
        }

        let again_back_passphrase_c_str = unsafe { CStr::from_ptr(passphrase_ptr) };
        let again_back_passphrase_c_string: CString =
            again_back_passphrase_c_str.try_into().unwrap();
        let again_back_passphrase_string: String = again_back_passphrase_c_string
            .to_string_lossy()
            .into_owned();
        assert_ne!(again_back_passphrase_string, "radix");
    }
}
