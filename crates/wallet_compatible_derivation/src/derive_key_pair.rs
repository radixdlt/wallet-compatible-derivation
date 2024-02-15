use ed25519_dalek::{PublicKey, SecretKey};

/// Derives an Ed255519 key pair on [`Curve25519`][curve],
/// using the hierarchal deterministic BIP32 derivation `path`,
/// and the `seed` of a hierarchal deterministic tree.
///
/// [curve]: https://en.wikipedia.org/wiki/Curve25519
pub(crate) fn derive_ed25519_key_pair(
    seed: &[u8],
    path: &slip10::path::BIP32Path,
) -> (SecretKey, PublicKey) {
    let key = slip10::derive_key_from_path(&seed, slip10::Curve::Ed25519, path).expect("Should never fail to derive Ed25519 Private key from seed for a valid BIP32Path - internal error, something wrong with SLIP10 Crate most likely");
    // Ed25519PrivateKey::from_bytes(&key.key)
    //     .expect("Should always be able to create Ed25519PrivateKey from derived key.")
    let private_key = SecretKey::from_bytes(&key.key)
        .expect("Should always be able to create Ed25519PrivateKey from derived key.");
    let public_key: PublicKey = (&private_key).into();
    (private_key, public_key)
}
