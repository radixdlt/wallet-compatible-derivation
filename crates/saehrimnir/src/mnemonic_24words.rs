use crate::prelude::*;

/// A guaranteed 24 words long BIP39 mnemonic.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub struct Mnemonic24Words(bip39::Mnemonic);

impl TryFrom<bip39::Mnemonic> for Mnemonic24Words {
    type Error = crate::Error;

    fn try_from(value: bip39::Mnemonic) -> Result<Self> {
        if value.word_count() != Self::WORD_COUNT {
            return Err(Error::UnsupportedMnemonicTooFewWords {
                expected: Self::WORD_COUNT,
                found: value.word_count(),
            });
        }
        Ok(Self(value))
    }
}

impl Mnemonic24Words {
    pub const WORD_COUNT: usize = 24;
    pub fn to_seed(&self, passphrase: impl AsRef<str>) -> [u8; 64] {
        self.0.to_seed(passphrase.as_ref())
    }
}

impl FromStr for Mnemonic24Words {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<bip39::Mnemonic>()
            .map_err(|_| Error::InvalidMnemonic)
            .and_then(|m| m.try_into())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn word_count_of_12_disallowed() {
        let intermediary: bip39::Mnemonic = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
            .parse()
            .unwrap();
        assert_eq!(
            Mnemonic24Words::try_from(intermediary),
            Err(Error::UnsupportedMnemonicTooFewWords {
                expected: 24,
                found: 12
            })
        );
    }

    #[test]
    fn word_count_of_24_works() {
        let s = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote";
        assert_eq!(s.parse::<Mnemonic24Words>().unwrap().to_string(), s);
    }
}
