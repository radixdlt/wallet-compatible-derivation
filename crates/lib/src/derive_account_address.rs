use crate::prelude::*;
use bip39::Mnemonic;

use radix_engine_common::{
    address::AddressBech32Encoder,
    crypto::{blake2b_256_hash, Ed25519PrivateKey, Ed25519PublicKey},
    types::ComponentAddress,
};

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
pub struct MnemonicID(String);

pub struct Account {
    pub network_id: NetworkID,
    pub private_key: Ed25519PrivateKey,
    pub public_key: Ed25519PublicKey,
    pub address: String,
    pub path: AccountPath,
    pub mnemonic_id: MnemonicID,
}

trait ToHex {
    fn to_hex(&self) -> String;
}
impl ToHex for Ed25519PrivateKey {
    fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}
impl ToHex for Ed25519PublicKey {
    fn to_hex(&self) -> String {
        hex::encode(self.to_vec())
    }
}

fn derive_address(public_key: &Ed25519PublicKey, network_id: NetworkID) -> String {
    let address_data = ComponentAddress::virtual_account_from_public_key(public_key.into());
    let address_encoder = AddressBech32Encoder::new(&network_id.network_definition());
    address_encoder
        .encode(&address_data.to_vec()[..])
        .expect("bech32 account address")
}

fn derive_ed25519_private_key(seed: &[u8], path: &slip10::path::BIP32Path) -> Ed25519PrivateKey {
    let key = slip10::derive_key_from_path(&seed, slip10::Curve::Ed25519, path).expect("Should never fail to derive Ed25519 Private key from seed for a valid BIP32Path - internal error, something wrong with SLIP10 Crate most likely");
    Ed25519PrivateKey::from_bytes(&key.key)
        .expect("Should always be able to create Ed25519PrivateKey from derived key.")
}

fn mnemonic_id(seed: &[u8]) -> MnemonicID {
    let components: Vec<HDPathComponentValue> = vec![PURPOSE, COINTYPE, harden(365)];
    let path = slip10::path::BIP32Path::from(components);
    let private_key = derive_ed25519_private_key(seed, &path);
    let public_key = private_key.public_key();
    let hash = blake2b_256_hash(&public_key.to_vec());
    let hex = hex::encode(hash);
    MnemonicID(hex)
}

pub fn derive_account(
    mnemonic: &Mnemonic,
    passphrase: impl AsRef<str>,
    path: &AccountPath,
) -> Result<Account, Error> {
    let network_id = path.network_id();
    let seed = mnemonic.to_seed(passphrase.as_ref());
    let mnemonic_id = mnemonic_id(&seed);
    let private_key = derive_ed25519_private_key(&seed, &path.0 .0);
    let public_key = private_key.public_key();
    let address = derive_address(&public_key, network_id);

    Ok(Account {
        network_id,
        private_key,
        public_key,
        address,
        path: path.clone(),
        mnemonic_id,
    })
}

#[cfg(test)]
mod tests {
    use bip39::Mnemonic;

    use crate::{derive_account_address::ToHex, prelude::*};

    fn test(
        mnemonic: Mnemonic,
        passphrase: impl AsRef<str>,
        network_id: NetworkID,
        path: impl AsRef<str>,
        private_key: impl AsRef<str>,
        public_key: impl AsRef<str>,
        mnemonic_id: impl AsRef<str>,
        address: impl AsRef<str>,
    ) {
        let path: AccountPath = path.as_ref().parse().unwrap();
        let account = derive_account(&mnemonic, passphrase.as_ref(), &path).unwrap();
        assert_eq!(account.private_key.to_hex(), private_key.as_ref());
        assert_eq!(account.public_key.to_hex(), public_key.as_ref());
        assert_eq!(account.mnemonic_id.to_string(), mnemonic_id.as_ref());
        assert_eq!(account.address, address.as_ref());
        assert_eq!(account.network_id, network_id);
        assert_eq!(account.path, path);
    }

    trait TestValue {
        fn test_0() -> Self;
    }
    impl TestValue for Mnemonic {
        fn test_0() -> Self {
            "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap()
        }
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_mainnet_index_0() {
        test(
            Mnemonic::test_0(),
            "radix",
            NetworkID::Mainnet,
            "m/44H/1022H/1H/525H/1460H/0H",
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_mainnet_index_1() {
        test(
            Mnemonic::test_0(),
            "radix",
            NetworkID::Mainnet,
            "m/44H/1022H/1H/525H/1460H/1H",
            "6b736e59d41c5ba47dc427ebee9990426441e01db4abee5c44192492c269d8e0",
            "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_stokenet_index_0() {
        test(
            Mnemonic::test_0(),
            "radix",
            NetworkID::Stokenet,
            "m/44H/1022H/2H/525H/1460H/0H",
            "4ec345585ce49c35424288dba67d0608eec3972ea4d7a54afeaf4b2cd7687e80",
            "18c7409458a82281711b668f833b0485e8fb58a3ceb8a728882bf6b83d3f06a9",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_stokenet_index_1() {
        test(
            Mnemonic::test_0(),
            "radix",
            NetworkID::Stokenet,
            "m/44H/1022H/2H/525H/1460H/1H",
            "f35325ea11511bfb16b0b846ae6d86ec9a91f978d4885463b57872627440ec1e",
            "26b3fd7f65f01ff8e418a56722fde9cc6fc18dc983e0474e6eb6c1cf3bd44f23",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr",
        );
    }
}
