use crate::prelude::*;

use ed25519_dalek::PublicKey;
use radix_common::prelude::*;

/// Creates a bech32m encoded Radix canonical address from an Ed25519 PublicKey and a
/// Radix `NetworkID`.
pub(crate) fn derive_address(public_key: &PublicKey, network_id: &NetworkID) -> String {
    let public_key = Ed25519PublicKey::try_from(public_key.to_bytes().as_slice()).expect("Should always be able to create a Radix Engine Ed25519PublicKey from Dalek Ed25519 public key");
    let address_data = ComponentAddress::preallocated_account_from_public_key(&public_key);
    let address_encoder = AddressBech32Encoder::new(&network_id.network_definition());
    address_encoder
        .encode(&address_data.to_vec()[..])
        .expect("bech32 account address")
}
