use clap::Args;
use saehrimnir::prelude::*;

use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Args, Zeroize, ZeroizeOnDrop)]
pub(crate) struct Config {
    /// The mnemonic you wanna use to derive accounts with.
    #[arg(short = 'm', long = "mnemonic", value_parser = Mnemonic24Words::from_str)]
    pub(crate) mnemonic: Mnemonic24Words,

    /// An optional BIP39 passphrase.
    #[arg(short = 'p', long = "passphrase", default_value_t = String::new())]
    pub(crate) passphrase: String,

    /// The Network you want to derive accounts on.
    #[arg(short = 'n', long = "network_id", value_parser = NetworkID::from_str, default_value_t = NetworkID::Mainnet)]
    pub(crate) network_id: NetworkID,

    /// The account index
    #[arg(short = 'i', long = "index", default_value_t = 0)]
    pub(crate) index: u32,
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

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString},
        mem,
        ops::Range,
    };

    use super::*;
    use saehrimnir::prelude::*;

    #[test]
    fn zeroize_config() {
        let mut config = Config {
            mnemonic: Mnemonic24Words::from_str("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote").unwrap(),
            passphrase: "radix".to_owned(),
            network_id: NetworkID::Mainnet,
            index: 0,
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
        assert_eq!(again_back_passphrase_string, "");
    }
}
