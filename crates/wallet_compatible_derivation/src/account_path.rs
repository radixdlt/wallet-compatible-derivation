use crate::prelude::*;

/// A Radix Babylon BIP32 path use to derive accounts, e.g.:
/// `"m/44'/1022'/1'/525'/1460'/2'"`
///
/// It has 6 levels:
/// `m / purpose' / coin_type' / network' / 525 / 1460' / index'`
///
/// So the example path above has network `1`, being mainnet, and `2` being the third account.
///
/// The levels can more generally be described as:
/// `m / purpose' / coin_type' / network' / ACCOUNT_KIND / TX_SIGNING' / index'`
///
/// Or even more general:
/// `m / purpose' / coin_type' / network' / entity_kind / key_kind' / index'`
///
/// Account's are not the only `entity_kind` the Radix Wallet's use, the Personas
/// use a different derivation path, with a different value on `entity_kind`.
///
/// There are also other `key_kind`s used by the Radix Wallet, one for [ROLA][rola] (if set),
/// and another for reserved for an upcoming feature.
///
/// Since we used [SLIP10][slip10] derivation scheme, every level MUST be hardened,
/// which `'` denotes.
///
/// ```
/// extern crate wallet_compatible_derivation;
/// use wallet_compatible_derivation::prelude::*;
///
/// assert!("m/44'/1022'/1'/525'/1460'/1'".parse::<AccountPath>().is_ok());
/// ```
///
/// But we tend to prefer `H` notation over `'` to denote that a component is hardened, so
/// throughout docs and unit tests you will see `H`.
///
/// ```
/// extern crate wallet_compatible_derivation;
/// use wallet_compatible_derivation::prelude::*;
///
/// assert!("m/44H/1022H/1H/525H/1460H/1H".parse::<AccountPath>().is_ok());
/// ```
///
/// [slip10]: https://github.com/satoshilabs/slips/blob/master/slip-0010.md
/// [rola]: https://docs-babylon.radixdlt.com/main/frontend/rola.html
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

/// The derivation "purpose" of the HDPath as per [BIP44][bip].
/// N.B. the [`AccountPath`] is NOT strict BIP44, but we follow the
/// pattern of IOTA and other projects which also use SLIP10, but
/// chose to use a BIP44 base.
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

    /// The entity_kind path component, must be `ENTITY_KIND_ACCOUNT` for
    /// `AccountPath`.
    pub(crate) const IDX_ENTITY_KIND: usize = 3;

    /// The kind_kind path component, must be `TRANSACTION_SIGNING` for 
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
}