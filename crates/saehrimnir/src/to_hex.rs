use ed25519_dalek::{PublicKey, SecretKey};

pub trait ToHex {
    fn to_hex(&self) -> String;
}
impl ToHex for SecretKey {
    fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}
impl ToHex for PublicKey {
    fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }
}
