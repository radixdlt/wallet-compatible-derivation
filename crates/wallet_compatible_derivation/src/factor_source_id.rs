use crate::prelude::*;
use radix_common::prelude::*;

/// A safe to use hex encoding of the hash of a public key at a special node in your BIP-39 Seed,
/// This ID is used to identify that two accounts have been derived from the same mnemonic.
/// Since it is the hash of a public key it does not reveal any secrets.
/// And the public key is not the public key of any account signing key, a
/// special derivation path which is different from that of accounts have been used
/// to derive this key pair.
#[derive(Zeroize, ZeroizeOnDrop, Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display("{}", self.to_hex())]
pub struct FactorSourceID([u8; 32]);

impl ToHex for FactorSourceID {
    fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl FactorSourceID {
    /// Creates a SAFE to use ID from a hierarchal deterministic tree's `seed`, by
    /// deriving a special public key at a non-leaf (non account) node in the tree,
    /// and then hashing that public key, using the `blake2b_256_hash` algorithm.
    pub(crate) fn from_seed(seed: &[u8]) -> Self {
        let components: Vec<HDPathComponentValue> = vec![PURPOSE, COINTYPE, harden(365)];
        let path = slip10::path::BIP32Path::from(components);
        let (private_key, public_key) = derive_ed25519_key_pair(seed, &path);
        drop(private_key);
        let hash = blake2b_256_hash(&public_key.as_bytes());
        Self(hash.into_bytes())
    }
}
