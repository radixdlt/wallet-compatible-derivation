use crate::prelude::*;

use radix_engine_common::crypto::{Ed25519PrivateKey, Ed25519PublicKey};

pub struct Account {
    pub network_id: NetworkID,
    pub private_key: Ed25519PrivateKey,
    pub public_key: Ed25519PublicKey,
    pub address: String,
    pub path: AccountPath,
    pub factor_source_id: FactorSourceID,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Factor Source ID: {}", self.factor_source_id)?;
        writeln!(f, "Address: {}", self.address)?;
        writeln!(f, "Network: {}", self.network_id)?;
        writeln!(f, "HD Path: {}", self.path)?;
        writeln!(f, "PrivateKey: {}", self.private_key.to_hex())?;
        writeln!(f, "PublicKey: {}", self.public_key.to_hex())?;
        Ok(())
    }
}
