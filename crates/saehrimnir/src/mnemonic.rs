use crate::prelude::*;

#[derive(Debug, Clone, derive_more::Display)]
pub struct Mnemonic(bip39::Mnemonic);

impl Mnemonic {
    pub fn to_seed(&self, passphrase: impl AsRef<str>) -> [u8; 64] {
        self.0.to_seed(passphrase.as_ref())
    }
}

impl FromStr for Mnemonic {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<bip39::Mnemonic>()
            .map_err(|_| Error::InvalidMnemonic)
            .map(|i| Self(i))
    }
}
