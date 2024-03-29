use crate::prelude::*;

/// A Radix Babylon [BIP-32][bip32] path used to derive accounts, for example `m/44'/1022'/1'/525'/1460'/2'`.
///
/// This comes from the general derivation pattern for Radix addresses according to the [SLIP-10][slip10] 
/// derivation scheme. In the [SLIP-10][slip10] derivation scheme, every level must be hardened, which
/// is denoted by the `'` or `H` suffix. The official Radix wallet uses 6 levels:
///
/// ```text
/// m / purpose' / coin_type' / network' / entity_kind' / key_kind' / entity_index'
/// ```
///
/// The `AccountPath` struct is parametrized by Radix network id and account index, but fixes the other
/// constants in the path as follows:
///
/// ```text
/// m / 44' / 1022' / NETWORK_ID' / 525' / 1460' / ACCOUNT_INDEX'
/// ```
///
/// More generally:
/// * `purpose` is fixed as `44` as per [BIP-44][bip44].
/// * `coin_type` is fixed as `1022` for Radix as per [SLIP-0044][slip44].
/// * `network` is the Radix network id (1 for `mainnet`, 2 for `stokenet`, ...).
/// * `entity_kind` is the type of Radix entity which keys are being generated for. Possible values include:
///   * 525 - Pre-allocated [accounts][account].
///   * 618 - Pre-allocated [identities][identity], which are used for [ROLA][rola] for personas.
/// * `key_kind` is the type of key. Possible values include:
///   * 1460 - Transaction Signing (the default).
///   * 1678 - Authentication Signing such as [ROLA][rola]. This is used if a separate key is
///     created for ROLA and stored in account metadata. 
/// * `entity_index` is the 0-based index of the particular entity which is being derived.
///
/// See `test_asciisum` for the source of the `entity_kind` and `key_kind` numbers.
///
/// ```
/// extern crate wallet_compatible_derivation;
/// use wallet_compatible_derivation::prelude::*;
///
/// assert!("m/44'/1022'/1'/525'/1460'/1'".parse::<AccountPath>().is_ok());
/// assert!("m/44H/1022H/1H/525H/1460H/1H".parse::<AccountPath>().is_ok());
/// ```
///
/// [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
/// [bip44]: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
/// [slip10]: https://github.com/satoshilabs/slips/blob/master/slip-0010.md
/// [slip44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
/// [rola]: https://docs.radixdlt.com/docs/rola-radix-off-ledger-auth
/// [account]: https://docs.radixdlt.com/docs/account
/// [identity]: https://docs.radixdlt.com/docs/identity
#[derive(
    Zeroize, ZeroizeOnDrop, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, derive_more::Display,
)]
pub struct AccountPath(pub(crate) BIP32Path<{ Self::DEPTH }>);

/// A HD Path component value, e.g. "1022" being the
/// coin type of Radix.
pub type HDPathComponentValue = u32;

const BIP32_HARDENED: HDPathComponentValue = 2147483648;

pub const fn harden(value: HDPathComponentValue) -> HDPathComponentValue {
    value + BIP32_HARDENED
}

pub const fn is_hardened(value: HDPathComponentValue) -> bool {
    value >= BIP32_HARDENED
}

/// Panics if `value` is not hardened.
pub const fn unhardened(value: HDPathComponentValue) -> HDPathComponentValue {
    assert!(is_hardened(value));
    value - BIP32_HARDENED
}

/// The derivation "purpose" of the HDPath as per [BIP-44][bip].
/// N.B. the [`AccountPath`] is NOT strict BIP-44, but we follow the
/// pattern of IOTA and other projects which also use SLIP-10, but
/// chose to use a BIP-44 base.
///
/// [bip]: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
pub const PURPOSE: HDPathComponentValue = harden(44);

/// The `cointype` of Radix DLT: `1022H`, as defined in SLIP44, see
/// merged PR: https://github.com/satoshilabs/slips/pull/1137
pub const COINTYPE: HDPathComponentValue = harden(1022);

/// The purpose of this key is to use it for Radix Accounts
/// (as oppose to Identities - used by Personas - which has
/// a different value).
const ENTITY_KIND_ACCOUNT: HDPathComponentValue = harden(525);

/// This key is used to control the entity - the Account, and
/// can sign transactions and change the state of the account.
const KEY_KIND_SIGN_TX: HDPathComponentValue = harden(1460);

/// The index of an account, e.g. `0` being the first
/// account derived for some Mnemonic at some network,
/// and `1` being the second. This value is HARDENED
/// when used in an AccountPath (as required by SLIP10).
pub type EntityIndex = u32;

impl AccountPath {
    /// Read the `network_id` of this AccountPath.
    pub fn network_id(&self) -> NetworkID {
        NetworkID::try_from(unhardened(self.0.clone().components()[Self::IDX_NETWORK_ID])).expect("Should not have been possible to instantiate an Account Path with an invalid Network ID.")
    }

    /// Read the accounts `index` of this AccountPath.
    pub fn account_index(&self) -> HDPathComponentValue {
        unhardened(self.0.clone().components()[Self::IDX_ACCOUNT_INDEX])
    }
}

impl AccountPath {
    /// The required depth, number of path components/levels of all account paths.
    pub const DEPTH: usize = 6;

    /// The index of `44'`
    pub(crate) const IDX_PURPOSE: usize = 0;

    /// The cointype of `1022'`, with the same value used in Olympia version of Radix.
    /// Being officially recorded in [SLIP44][slip] on 2021-07-16.
    ///
    /// [slip]: https://github.com/satoshilabs/slips/pull/1137
    pub(crate) const IDX_COINTYPE: usize = 1;

    /// The id of the network this account can be used on,
    /// see [`NetworkID`].
    pub(crate) const IDX_NETWORK_ID: usize = 2;

    /// The `entity_kind` path component, must be `ENTITY_KIND_ACCOUNT` for
    /// `AccountPath`.
    pub(crate) const IDX_ENTITY_KIND: usize = 3;

    /// The `key_kind` path component, must be `TRANSACTION_SIGNING` for
    /// virtual account derivation.
    pub(crate) const IDX_KEY_KIND: usize = 4;

    /// The last path component, the index of the account.
    pub(crate) const IDX_ACCOUNT_INDEX: usize = 5;

    /// Crates a new `AccountPath` given the tuple (network, index).
    pub fn new(network_id: &NetworkID, index: EntityIndex) -> Self {
        let bip32_path = BIP32Path::<{ Self::DEPTH }>([
            PURPOSE,
            COINTYPE,
            network_id.hardened_hd_component_value(),
            ENTITY_KIND_ACCOUNT,
            KEY_KIND_SIGN_TX,
            harden(index),
        ]);

        bip32_path
            .try_into()
            .expect("Should have constructed a valid AccountPath from network_id and index.")
    }
}

impl TryFrom<BIP32Path<{ Self::DEPTH }>> for AccountPath {
    type Error = crate::Error;

    /// Tries to create a new `AccountPath` from a `BIP32Path`, by validating it,
    /// returning `Err` if it is invalid.
    fn try_from(value: BIP32Path<{ Self::DEPTH }>) -> Result<Self, Self::Error> {
        if !value.clone().into_iter().all(|c| is_hardened(c)) {
            return Err(Error::InvalidAccountPathNonHardenedPathComponent);
        }
        let components = value.clone().components();

        if components.len() != Self::DEPTH {
            return Err(Error::InvalidAccountPathWrongDepth {
                expected: Self::DEPTH,
                found: components.len(),
            });
        }
        let assert_with = |i, f: fn(HDPathComponentValue) -> bool| {
            if !f(components[i]) {
                Err(Error::InvalidAccountPathInvalidValue {
                    index: i,
                    found: components[i],
                })
            } else {
                Ok(())
            }
        };
        let assert_value = |i, v| {
            if components[i] != v {
                Err(Error::InvalidAccountPathWrongValue {
                    index: i,
                    expected: v,
                    found: components[i],
                })
            } else {
                Ok(())
            }
        };
        assert_value(Self::IDX_PURPOSE, PURPOSE)?;
        assert_value(Self::IDX_COINTYPE, COINTYPE)?;
        assert_with(Self::IDX_NETWORK_ID, |v| {
            NetworkID::all()
                .into_iter()
                .map(|n| n.hardened_hd_component_value())
                .any(|c| c == v)
        })?;
        assert_value(Self::IDX_ENTITY_KIND, ENTITY_KIND_ACCOUNT)?;
        assert_value(Self::IDX_KEY_KIND, KEY_KIND_SIGN_TX)?;
        // Nothing to validate at component index `IDX_ACCOUNT_INDEX` (5)
        Ok(Self(value))
    }
}

impl FromStr for AccountPath {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<BIP32Path<{ Self::DEPTH }>>()
            .and_then(|p| p.try_into())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn string_roundtrip() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        let path: AccountPath = s.parse().unwrap();
        assert_eq!(path.to_string(), s);
        assert_eq!(path.network_id(), NetworkID::Mainnet);
        assert_eq!(path.account_index(), 0);
    }


    #[test]
    fn test_asciisum() {
        let ascii_sum = |s: &str| s.chars().into_iter().fold(0, |acc, c| acc + c as u64);
        assert_eq!(ascii_sum("ACCOUNT"), 525);
        assert_eq!(ascii_sum("IDENTITY"), 618);
        assert_eq!(ascii_sum("TRANSACTION_SIGNING"), 1460);
        assert_eq!(ascii_sum("AUTHENTICATION_SIGNING"), 1678);
        assert_eq!(ascii_sum("GETID"), 365);
    }
}
