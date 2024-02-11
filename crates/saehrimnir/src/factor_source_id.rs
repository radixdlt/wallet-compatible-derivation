use radix_engine_common::crypto::blake2b_256_hash;

use crate::prelude::*;

/// A safe to use hex encoding of the hash of a public key at a special node in your BIP39 Seed,
/// This ID is used to identify that two accounts have been derived from the same mnemonic.
/// Since it is the hash of a public key it does not reveal any secrets.
/// And the public key is not the public key of any account signing key, a
/// special derivation path which is different from that of accounts have been used
/// to derive this key pair.
#[derive(ZeroizeOnDrop, Clone, Debug, PartialEq, Eq, derive_more::Display)]
pub struct FactorSourceID(String);

impl FactorSourceID {
    pub(crate) fn from_seed(seed: &[u8]) -> Self {
        let components: Vec<HDPathComponentValue> = vec![PURPOSE, COINTYPE, harden(365)];
        let path = slip10::path::BIP32Path::from(components);
        let (private_key, public_key) = derive_ed25519_key_pair(seed, &path);
        drop(private_key);
        let hash = blake2b_256_hash(&public_key.as_bytes());
        let hex = hex::encode(hash);
        FactorSourceID(hex)
    }
}