use crate::prelude::*;

use ed25519_dalek::{PublicKey, SecretKey};

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

    /// Derives a simple [`Account`] using the `mnemonic` and BIP-39 `passphrase` (can be the empty string) using the hierarchical deterministic derivation path `path`.
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
// and from Gist
// https://gist.github.com/Sajjon/060c5747c6ffead12f78645b623a8164
#[cfg(test)]
mod tests {

    use crate::prelude::*;
    use std::ops::Range;

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
        assert_eq!(account_path.to_string(), path); // test Display
        assert_eq!(path.parse::<AccountPath>().unwrap(), account_path); // test FromStr
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

    // ========= SECURIFIED KEY SPACE =========
    // Higher part of the 2^31-1 key space
    // These values have been cross references
    // using this Python script:
    // https://gist.github.com/Sajjon/060c5747c6ffead12f78645b623a8164
    // Which is based on the SLIP10 reference implementation:
    // https://github.com/satoshilabs/slips/blob/master/slip-0010/testvectors.py
    // ========================================
    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_0() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            0,
            "m/44H/1022H/1H/525H/1460H/0H",
            "5b82120ec4763f8bacff71c8e529894fea1e735a5698ff400364a913f7b20c00",
            "f3d0210f6c2cecbdc977b7aae19d468a6c363e73a055bc877248f8318f0122e8",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12xek3geay25lktmk5zyplc7z7mg5xe8ldh48ta4mkcd9q0v0q6l8y6",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_1() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            1,
            "m/44H/1022H/1H/525H/1460H/1H",
            "da5e924c716a05b616940dd7828e3020de4dc09c371ab03966e00e95c68cb439",
            "df49129a10aa88c76837611a4ecda794ac5f650e4401037e1ff275e52bc784c5",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx128mh2ae9dsrwa0t8l37ayjrxxf0p84e6qm227ytxtcu447f5uw5m8w",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_minus_3_hardened() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            (2i32.pow(30) - 3) as u32,
            "m/44H/1022H/1H/525H/1460H/1073741821H",
            "d9e0394b67affb91b5acdc3ecf6786a6628892ffd605291c853568cbed498afa",
            "a484112bcd119488f13191a6ec57ff27606ea041537662730e60580cdb679616",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12yxslky3ye2rdtcv439s7l8hw2pm7sp6g3e537dsuk6558z66yteu5",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_minus_2_hardened() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            (2i32.pow(30) - 2) as u32,
            "m/44H/1022H/1H/525H/1460H/1073741822H",
            "5218159039d5c639ae4e0b6b351b821e3687aa44768230c4f06a13ae0c78715c",
            "2155707a3cebd7788dc83113174d30e2c29abae34f399c27a6caa8c6f5ae543e",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx1299mrqwvhy6cka9vsvjddqhttm9qckk08w32kp6nrnzrwaclqelp4x",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_minus_1_hardened() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            (2i32.pow(30) - 1) as u32,
            "m/44H/1022H/1H/525H/1460H/1073741823H",
            "5c5adfebe650684e3cc20e4dba49e1447d7ac12f63ae1bd8723554d0a95aaf38",
            "f4f43daaedc3603b3dc6b92a2014630a96ca2a20cc14d2dcaa71f49c30789689",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx129zmjv05ljhm3tc3f5nayvfgym69fu6zlajt6xp2jj900c5qt76m6v",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_hardened() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            2u32.pow(30),
            "m/44H/1022H/1H/525H/1460H/1073741824H", // Sargon securified notation: "/0S"
            "b0b9180f7c96778cffba7af2ef1ddf4705fca21b965e8a722ccf2ec403c35950",
            "e0293d4979bc303ea4fe361a62baf9c060c7d90267972b05c61eead9ef3eed3e",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx128znphf3gxek50qyxjcuels6xtulum3g46vhr43ryavj7zr53xxded",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_plus_1_hardened() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            2u32.pow(30) + 1,
            "m/44H/1022H/1H/525H/1460H/1073741825H", // Sargon securified notation: "/1S"
            "c1880587c727f2f01dfdf61d19b44283d311b31c12e8898b774b73e8067d25b1",
            "c6aaee6fa60d73a17989ce2a2a5db5a88cd696aef61d2f298262fae189dff04e",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12y8gd9dyz9mhg3jv5p9md5gvuzc34m0p90te0hx7aqgsvuy5g2p09s",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow30_plus_2_hardened() {
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            2u32.pow(30) + 2,
            "m/44H/1022H/1H/525H/1460H/1073741826H", // Sargon securified notation: "/2S"
            "837bc77bb29e4702be39c69fbade7d350bc23f6daddf68a64474984e899a97a3",
            "6a92b3338dc74a50e8b3fff896a7e0f43c42742544af52de20353675d8bc7907",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12xluhgaw3vcyskpsmswu279jlysmrdjuk23erjcx8s83kcgx3r4zvn",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_5() {
        let idx = 2u32.pow(31u32) - 5;
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            idx,
            "m/44H/1022H/1H/525H/1460H/2147483643H",
            "8371cdce66f0733cf1f8a07235825267e8e650f9bf194dfe82992c8ae77faa84",
            "9bce7e1a1d724b2013add0697e4133e2affc93b806793ee6709dfdc242738e19",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12x0z0sm5qpp9gmuah7nnpkkkk2zn2r8tvpd9w64097949mcs7jm960",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_4() {
        let idx = 2u32.pow(31u32) - 4;
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            idx,
            "m/44H/1022H/1H/525H/1460H/2147483644H", // Sargon securified notation: "/1073741820S"
            "361126bd7947254c49b83c23bbb557219cfa2ac5e5a4551501f18236ffa4eb17",
            "481b737f5baaf52520612e70858ffa72a3624d5a050da5748844ac14036c8b17",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12y27yrwuqmec5saaykp82098nykpeqentzt3syt4dfdyuq0ckkc07u",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_3() {
        let idx = 2u32.pow(31u32) - 3;
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            idx,
            "m/44H/1022H/1H/525H/1460H/2147483645H", // Sargon securified notation: "/1073741821S"
            "f63fe429c5723448dfb8d1f3eda88a659473b4c38960a09bb20efe546fac95ee",
            "b2819057da648f36eadb59f60b732d4ae7fb22a207acf214e0271d3c587afd54",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12x9mszdtxacj5trw78g2ndvc54wtxg9mxx982w2p8vnv7jes7nvc40",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_2() {
        let idx = 2u32.pow(31u32) - 2;
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            idx,
            "m/44H/1022H/1H/525H/1460H/2147483646H", // Sargon securified notation: "/1073741822S"
            "5a8b6327942ca8fc5b30fb5b0c1fa53e97362d514ff4f2c281060b9d51f7fc88",
            "932123e6c46af8ebde7a96bee4563e09bbf41b28eae9d6ba1c667a2f490a1fcf",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx12ysf5nesz5h3wk8aypyn83e9752mal8q545epwykq6nr8k8aavyu7d",
        );
    }

    #[test]
    fn derive_account_mnemonic_2_with_passphrase_mainnet_index_2pow31_minus_1() {
        let idx = 2u32.pow(31u32) - 1;
        test(
            Mnemonic24Words::test_2(),
            "",
            NetworkID::Mainnet,
            idx,
            "m/44H/1022H/1H/525H/1460H/2147483647H", // Sargon securified notation: "/1073741823S"
            "7eae6f235206329561b09fc2235d35e017c3f28b54fd3b4f6525e601257c4ce7",
            "87a2f84f826da0c62052fbe7b385ab78883c02d1fa5472c55a06aa529a0701e9",
            "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd",
            "account_rdx128258pxhges8rmva0a2egr0tzqd8x8clsl5d90a8qv3zqggc4jr2ss",
        );
    }
}
