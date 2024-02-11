use thiserror::Error as ThisError;

use crate::HDPathComponentValue;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(ThisError, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid BIP39 mnemonic")]
    InvalidMnemonic,

    #[error(
        "Mnemonic has too few words, only {expected} words mnemonics are supported, found: {found}"
    )]
    UnsupportedMnemonicTooFewWords { expected: usize, found: usize },

    #[error("Unsupported or unknown Network ID: '{0}'")]
    UnsupportedOrUnknownNetworkID(HDPathComponentValue),

    #[error("Unsupported or unknown Network ID: '{0}'")]
    UnsupportedOrUnknownNetworkIDFromStr(String),

    #[error("Invalid BIP32 HD path: '{0}'")]
    InvalidBIP32Path(String),

    #[error("Invalid Radix Account path (but valid BIP32): '{0}'")]
    InvalidAccountPath(String),

    #[error("Invalid Radix Account path, non hardened path component found.")]
    InvalidAccountPathNonHardenedPathComponent,

    #[error("Invalid Radix Account path, expected: {expected}, found {found}.")]
    InvalidAccountPathWrongDepth { expected: usize, found: usize },

    #[error("Invalid BIP32 Path, expected: {expected}, found {found}.")]
    InvalidDepthOfBIP32Path { expected: usize, found: usize },

    #[error("Invalid Radix Account path, invalid value at index: {index}, expected: {expected}, found {found}.")]
    InvalidAccountPathWrongValue {
        index: usize,
        expected: HDPathComponentValue,
        found: HDPathComponentValue,
    },

    #[error("Invalid Radix Account path, invalid value at index: {index} found {found}.")]
    InvalidAccountPathInvalidValue {
        index: usize,
        found: HDPathComponentValue,
    },
}
