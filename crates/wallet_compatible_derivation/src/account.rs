use crate::prelude::*;

use ed25519_dalek::{PublicKey, SecretKey};
use zeroize::Zeroize;

/// A tuple of keys and Radix Babylon Account address, for a
/// virtual account - an account that the Radix Public Ledger
/// knows nothing about (if you haven't used this account before that is).
#[derive(ZeroizeOnDrop, Zeroize, derive_more::Display)]
#[display("{}", self.to_string_include_private_key(false))]
pub struct Account {
    /// The network used to derive the `address`.
    #[zeroize(skip)]
    pub network_id: NetworkID,

    /// The private key controlling this account - assuming that you have
    /// not used this account before and change it to be controlled by
    /// a set of factors (MFA), thus controlled by an access controller.
    pub private_key: SecretKey,

    /// The public key of this account, derived from `private_key`, was used
    /// together with the `network_id` to derive the `address`.
    #[zeroize(skip)]
    pub public_key: PublicKey,

    /// A bech32 encoded Radix Babylon account address
    pub address: String,

    /// The value of the last HD path component, the account index.
    pub index: HDPathComponentValue,

    /// The HD path which was used to derive the keys.
    pub path: AccountPath,

    /// ID used to identify that two accounts have been derived from the same mnemonic - does not reveal any secrets.
    pub factor_source_id: FactorSourceID,
}

impl Account {
    pub fn to_string_include_private_key(&self, include_private_key: bool) -> String {
        let private_key_or_empty = if include_private_key {
            format!("\nPrivateKey: {}", self.private_key.to_hex())
        } else {
            "".to_owned()
        };
        format!(
            "
Factor Source ID: {}
Address: {}
Network: {}
Index: {}
HD Path: {}{}
PublicKey: {}
",
            self.factor_source_id,
            self.address,
            self.network_id,
            self.index,
            self.path,
            private_key_or_empty,
            self.public_key.to_hex()
        )
    }

    /// Derives a simple [`Account`] using the `mnemonic` and BIP39 `passphrase` (can be the empty string) using the hierarchical deterministic derivation path `path`.
    ///
    /// See [`Account`] for more details, but in short it is an Address + key pair.
    pub fn derive(
        mnemonic: &Mnemonic24Words,
        passphrase: impl AsRef<str>,
        path: &AccountPath,
    ) -> Self {
        let network_id = path.network_id();
        let seed = mnemonic.to_seed(passphrase.as_ref());
        let factor_source_id = FactorSourceID::from_seed(&seed);
        let (private_key, public_key) = derive_ed25519_key_pair(&seed, &path.0.inner());
        let address = derive_address(&public_key, &network_id);

        Self {
            network_id,
            private_key,
            public_key,
            address,
            index: path.clone().account_index(),
            path: path.clone(),
            factor_source_id,
        }
    }

    pub fn is_zeroized(&self) -> bool {
        self.private_key.to_bytes() == [0; 32]
    }
}

// Test vectors from Swift lib, see commit:
// https://github.com/radixdlt/babylon-wallet-ios/commit/f5f654a40b2afa48820919360f2e8d2f00ebe63e
#[cfg(test)]
mod tests {

    use crate::prelude::*;
    use std::ops::Range;
    use zeroize::Zeroize;

    #[test]
    fn to_string_include_private_key() {
        let path: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".parse().unwrap();
        let account = Account::derive(&Mnemonic24Words::test_0(), "", &path);
        let expected = "\nFactor Source ID: 6facb00a836864511fdf8f181382209e64e83ad462288ea1bc7868f236fb8033\nAddress: account_rdx128vge9xzep4hsn4pns8qch5uqld2yvx6f3gfff786du7vlk6w6e6k4\nNetwork: Mainnet\nIndex: 0\nHD Path: m/44H/1022H/1H/525H/1460H/0H\nPrivateKey: 7b21b62816c6349293abc3a8c37470f917ae621ada2eb8d5124250e83b78f7ef\nPublicKey: 6224937b15ec4017a036c0bd6999b7fa2b9c2f9452286542fd56f6a3fb6d33ed\n";

        assert_eq!(account.to_string_include_private_key(true), expected);
    }

    fn test(
        mnemonic: Mnemonic24Words,
        passphrase: impl AsRef<str>,
        network_id: NetworkID,
        index: HDPathComponentValue,
        path: impl AsRef<str>,
        private_key: impl AsRef<str>,
        public_key: impl AsRef<str>,
        factor_source_id: impl AsRef<str>,
        address: impl AsRef<str>,
    ) {
        let path = path.as_ref();
        let account_path = AccountPath::new(&network_id, index);
        assert_eq!(path.parse::<AccountPath>().unwrap(), account_path); // test FromStr
        assert_eq!(account_path.to_string(), path); // test Display
        let account = Account::derive(&mnemonic, passphrase.as_ref(), &account_path);
        assert_eq!(account.private_key.to_hex(), private_key.as_ref());
        assert_eq!(account.public_key.to_hex(), public_key.as_ref());
        assert_eq!(
            account.factor_source_id.to_string(),
            factor_source_id.as_ref()
        );
        assert_eq!(account.address, address.as_ref());
        assert_eq!(account.network_id, network_id);
        assert_eq!(account.path, account_path);
        assert_eq!(account.index, index);
    }

    #[test]
    fn zeroize_account_private_key_is_zeroized() {
        let mnemonic = Mnemonic24Words::new([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);
        let path: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".parse().unwrap();
        let mut account = Account::derive(&mnemonic, "", &path);

        let private_key_view = &account.private_key as *const _ as *const u8;
        let private_key_range = Range { start: 0, end: 32 };

        let mut key_bytes = Vec::<u8>::new();
        for i in private_key_range.clone() {
            key_bytes.push(unsafe { *private_key_view.offset(i) })
        }

        // Expected private key given the mnemonic and path, see unit test
        // "derive_account_mnemonic_1_without_passphrase_mainnet_index_0" below.
        let key_hex = "2bd55b473c972e32667582acd73653b67f7d56a74f9aab3f73126a7b7ad49de6";

        assert_eq!(account.private_key.to_hex(), key_hex);
        assert_eq!(hex::encode(key_bytes), key_hex);

        account.zeroize();

        // Assert that private key has been zeroized.
        for i in private_key_range {
            assert_eq!(unsafe { *private_key_view.offset(i) }, 0x00);
        }
    }

    #[test]
    fn derive_account_mnemonic_0_without_passphrase_mainnet_index_0() {
        test(
            Mnemonic24Words::test_0(),
            "",
            NetworkID::Mainnet,
            0,
            "m/44H/1022H/1H/525H/1460H/0H",
            "7b21b62816c6349293abc3a8c37470f917ae621ada2eb8d5124250e83b78f7ef",
            "6224937b15ec4017a036c0bd6999b7fa2b9c2f9452286542fd56f6a3fb6d33ed",
            "6facb00a836864511fdf8f181382209e64e83ad462288ea1bc7868f236fb8033",
            "account_rdx128vge9xzep4hsn4pns8qch5uqld2yvx6f3gfff786du7vlk6w6e6k4",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_without_passphrase_mainnet_index_1() {
        test(
            Mnemonic24Words::test_0(),
            "",
            NetworkID::Mainnet,
            1,
            "m/44H/1022H/1H/525H/1460H/1H",
            "e153431a8e55f8fde4d6c5377ea4f749fd28a6f196c7735ce153bd16bcbfcd6e",
            "a8d6fb3b7f3627b4589c2b663e8cc9b4d49df7013220ac0edd7e22e6cc608fa6",
            "6facb00a836864511fdf8f181382209e64e83ad462288ea1bc7868f236fb8033",
            "account_rdx129xapgx582768wrkd54mq0a8lhp8aqp5vkkc8u2jfavujktl0tatcs",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_with_passphrase_mainnet_index_0() {
        test(
            Mnemonic24Words::test_0(),
            "radix",
            NetworkID::Mainnet,
            0,
            "m/44H/1022H/1H/525H/1460H/0H",
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_with_passphrase_mainnet_index_1() {
        test(
            Mnemonic24Words::test_0(),
            "radix",
            NetworkID::Mainnet,
            1,
            "m/44H/1022H/1H/525H/1460H/1H",
            "6b736e59d41c5ba47dc427ebee9990426441e01db4abee5c44192492c269d8e0",
            "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_without_passphrase_stokenet_index_0() {
        test(
            Mnemonic24Words::test_0(),
            "",
            NetworkID::Stokenet,
            0,
            "m/44H/1022H/2H/525H/1460H/0H",
            "2e7def75661fcd8a8916866546a7713bc10fea728d46487f33e3fa09f538038c",
            "5fdfa89b784cc63fc90f67bd3481f6611a798a9581b414bf627f758075e95ca1",
            "6facb00a836864511fdf8f181382209e64e83ad462288ea1bc7868f236fb8033",
            "account_tdx_2_12x4rz8yh6t2qtpwdmzc2fvz9xvr00rvv37v7lk3eyh8re7z6r0xyw8",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_without_passphrase_stokenet_index_1() {
        test(
            Mnemonic24Words::test_0(),
            "",
            NetworkID::Stokenet,
            1,
            "m/44H/1022H/2H/525H/1460H/1H",
            "c24fe54ad3cff0ba2627935e11f75fae12c477828d96fdfe3a707defa1d5db57",
            "0c6cf91e9b669bf09aeff687c86f6158f8fdfb23d0034bd3cb3f95c4443e9324",
            "6facb00a836864511fdf8f181382209e64e83ad462288ea1bc7868f236fb8033",
            "account_tdx_2_12xwkvs77drhw7lxnw2aewrs264yhhkln7zzpejye66q6gt5mc2kphn",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_with_passphrase_stokenet_index_0() {
        test(
            Mnemonic24Words::test_0(),
            "radix",
            NetworkID::Stokenet,
            0,
            "m/44H/1022H/2H/525H/1460H/0H",
            "4ec345585ce49c35424288dba67d0608eec3972ea4d7a54afeaf4b2cd7687e80",
            "18c7409458a82281711b668f833b0485e8fb58a3ceb8a728882bf6b83d3f06a9",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
        );
    }

    #[test]
    fn derive_account_mnemonic_0_with_passphrase_stokenet_index_1() {
        test(
            Mnemonic24Words::test_0(),
            "radix",
            NetworkID::Stokenet,
            1,
            "m/44H/1022H/2H/525H/1460H/1H",
            "f35325ea11511bfb16b0b846ae6d86ec9a91f978d4885463b57872627440ec1e",
            "26b3fd7f65f01ff8e418a56722fde9cc6fc18dc983e0474e6eb6c1cf3bd44f23",
            "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
            "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_without_passphrase_mainnet_index_0() {
        test(
            Mnemonic24Words::test_1(),
            "",
            NetworkID::Mainnet,
            0,
            "m/44H/1022H/1H/525H/1460H/0H",
            "2bd55b473c972e32667582acd73653b67f7d56a74f9aab3f73126a7b7ad49de6",
            "cd0ace2fe890da0139d69d4414f146e5a36d4d76b65520d0d3d6967b1b57cb99",
            "3bf4636876a9c795486194d2eaff32790961ed9005e18a7ebe677f0947b54087",
            "account_rdx128dp80lfaywaqchg4fqymy76pqvl20mjmpw08839yfh4qz6us4ltaj",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_without_passphrase_mainnet_index_1() {
        test(
            Mnemonic24Words::test_1(),
            "",
            NetworkID::Mainnet,
            1,
            "m/44H/1022H/1H/525H/1460H/1H",
            "af64f29665576e01e3fb10f9836e4b0fa066efe7a88f867f917be00620386f0b",
            "bb09890daf2ed7a89bcd69eb56f56bc9208a37a147c1d9804db4f12d185a46a6",
            "3bf4636876a9c795486194d2eaff32790961ed9005e18a7ebe677f0947b54087",
            "account_rdx129vlwaav373ucq6jewq6z722de5yd4ulklguv87u0ql0hmw5redatp",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_mainnet_index_0() {
        test(
            Mnemonic24Words::test_1(),
            "foo",
            NetworkID::Mainnet,
            0,
            "m/44H/1022H/1H/525H/1460H/0H",
            "37947aece03dfbbe89672cb5b1caba88629625739750db7b8b0d8cb4bd5631f8",
            "111ae3183e7b93c0f751bbfbc8aba6888434d889e3805f8941669e3194721290",
            "883882e1d9d47b98090163bb4b369ae00349507693d856b1854de103dfe52793",
            "account_rdx12xg8ncs6xd8fr9t3gzx3sv3k8nmu8q4ekgxaahdlnxhn2rfrh04k2w",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_mainnet_index_1() {
        test(
            Mnemonic24Words::test_1(),
            "foo",
            NetworkID::Mainnet,
            1,
            "m/44H/1022H/1H/525H/1460H/1H",
            "431fc569aac0a7fe55c7537b9c46977c66eb50cd6383795ecef64a6fb2aa39aa",
            "e24df52deaa191fd247d1f0c10d55ff9251c1b7b50e61125bb419bd28e76b4c2",
            "883882e1d9d47b98090163bb4b369ae00349507693d856b1854de103dfe52793",
            "account_rdx12ydzkre4ujmn5mz2rddqt5mytl7ek52c7fgks48fusj32rfs0ns40n",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_without_passphrase_stokenet_index_0() {
        test(
            Mnemonic24Words::test_1(),
            "",
            NetworkID::Stokenet,
            0,
            "m/44H/1022H/2H/525H/1460H/0H",
            "b5ecb0b6b928a198cb1a6bb87b0b67a5ae675961ea4b835e9aad8629828600ab",
            "12d9d790ef471e11738ff7ba3f99d1ddc58d969c9a796848f8e4af01d294c263",
            "3bf4636876a9c795486194d2eaff32790961ed9005e18a7ebe677f0947b54087",
            "account_tdx_2_129t4rk8hyu9ekz9jgxcveprkm40dly5f4tc426sdqz7fa7mtgkmmff",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_without_passphrase_stokenet_index_1() {
        test(
            Mnemonic24Words::test_1(),
            "",
            NetworkID::Stokenet,
            1,
            "m/44H/1022H/2H/525H/1460H/1H",
            "2b2e6ce6abe0ab7ac7eb15d0809f4a44809ef979449bdd3550a5791a86e927ca",
            "2b1414b927a03ade597127bdaa90db93f60518795141ab5c451649f4997acddb",
            "3bf4636876a9c795486194d2eaff32790961ed9005e18a7ebe677f0947b54087",
            "account_tdx_2_128cplhpppm0295zxf9507tlng8zf539jv9rc2pmaymkft36qpt7slj",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_stokenet_index_0() {
        test(
            Mnemonic24Words::test_1(),
            "foo",
            NetworkID::Stokenet,
            0,
            "m/44H/1022H/2H/525H/1460H/0H",
            "a5f1c8d8433416b147c09ce6a5dd83bb77cab9d344ea9ea458d4a0c45b30ec7a",
            "810b03bf9c767f66e0e8caca015873c96bf7df0c5a28884f30a9a2837386cb7b",
            "883882e1d9d47b98090163bb4b369ae00349507693d856b1854de103dfe52793",
            "account_tdx_2_129kc6c9fhmsgstj4kv8ycc76z7nf36j46saav84lwt6ttdpeq44w6l",
        );
    }

    #[test]
    fn derive_account_mnemonic_1_with_passphrase_stokenet_index_1() {
        test(
            Mnemonic24Words::test_1(),
            "foo",
            NetworkID::Stokenet,
            1,
            "m/44H/1022H/2H/525H/1460H/1H",
            "df60dafc61032f3bb0bd48ef6ba4bed03b93f5f87277d47456655736f4be709f",
            "eb04aaa6721c86fd71f9e7e5173f7a176a11c9e9407f39bbe998bc3bb12f03e5",
            "883882e1d9d47b98090163bb4b369ae00349507693d856b1854de103dfe52793",
            "account_tdx_2_129peacgfcj99m8ty9s2z09u7n3dhf6ps0n6mlz5ttex7mnfrzyjtt5",
        );
    }
}
