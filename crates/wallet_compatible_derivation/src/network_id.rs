use radix_common::prelude::NetworkDefinition;
use strum_macros::{Display, EnumString};
use std::borrow::Cow;

use crate::prelude::*;

/// The network on which an account can be used. For `Mainnet` the value `1` is used,
/// for `Stokenet` the value `2` is used.
///
/// See [Babylon-node repo][node] for more details.
///
/// [node]: https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L82-L98
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, enum_iterator::Sequence,Copy
)]
pub enum NetworkID {
    /// The Radix mainnet.
    #[strum(ascii_case_insensitive)]
    Mainnet = 0x01,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Stokenet = 0x02,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Enkinet = 0x21,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Hammunet = 0x22,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Dumunet = 0x25,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Mardunet = 0x24,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Gilganet = 0x20,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Nergalnet = 0x23
}
impl NetworkID {
    /// Returns a collection of all networks this software support.
    ///
    /// If you have the need for support for any other testnet, let us know.
    ///
    /// For more information about other possible networks, [see here][node]
    ///
    /// [node]: https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L82-L98
    pub fn all() -> Vec<NetworkID> {
        enum_iterator::all::<NetworkID>().collect::<Vec<_>>()
    }

    /// The raw representation of this network id, an `u8`.
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }

    /// Name, most not be changed, i.e. cannot capitalized, is used
    /// by app to validate against Gateway
    pub fn logical_name(&self) -> String {
        self.network_definition().logical_name.into_owned()
    }
}

impl TryFrom<HDPathComponentValue> for NetworkID {
    type Error = Error;

    /// Tries to create a `NetworkID` from a path component, the value
    /// passed MUST be non-hardened / unhardened.
    /// 
    /// See `unhardened` function.
    fn try_from(value: HDPathComponentValue) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NetworkID::Mainnet),
            2 => Ok(NetworkID::Stokenet),
            32 => Ok(NetworkID::Gilganet),
            33 => Ok(NetworkID::Enkinet),
            34 => Ok(NetworkID::Hammunet),
            35 => Ok(NetworkID::Nergalnet),
            36 => Ok(NetworkID::Mardunet),
            37 => Ok(NetworkID::Dumunet),

            _ => Err(Error::UnsupportedOrUnknownNetworkID(value)),
        }
    }
}

impl NetworkID {

    /// Returns `<self>H`, that is, the discriminant of the network id
    /// but hardened, as per SLIP10.
    pub fn hardened_hd_component_value(&self) -> HDPathComponentValue {
        match self {
            NetworkID::Mainnet => harden(1),
            NetworkID::Stokenet => harden(2),
            NetworkID::Enkinet =>harden(33),
            NetworkID::Hammunet =>harden(34),
            NetworkID::Dumunet =>harden(37),
            NetworkID::Mardunet =>harden(36),
            NetworkID::Gilganet =>harden(32),
            NetworkID::Nergalnet =>harden(35),
        }
    }

    /// A network definition used by this library to form bech32 encoded
    /// addresses.
    pub(crate) fn network_definition(&self) -> ScryptoNetworkDefinition {
        match self {
            NetworkID::Mainnet => NetworkDefinition::mainnet(),
            NetworkID::Stokenet => NetworkDefinition::stokenet(),
            NetworkID::Enkinet => ScryptoNetworkDefinition {
                id: NetworkID::Enkinet.discriminant(),
                logical_name: Cow::Borrowed("enkinet"),
                hrp_suffix: Cow::Borrowed("tdx_21_"),
            },
            NetworkID::Hammunet =>ScryptoNetworkDefinition {
                id: NetworkID::Hammunet.discriminant(),
                logical_name: Cow::Borrowed("hammunet"),
                hrp_suffix: Cow::Borrowed("tdx_22_"),
            },
            NetworkID::Dumunet => ScryptoNetworkDefinition {
                id: NetworkID::Dumunet.discriminant(),
                logical_name: Cow::Borrowed("dumunet"),
                hrp_suffix: Cow::Borrowed("tdx_25_"),
            },
            NetworkID::Mardunet =>ScryptoNetworkDefinition {
                id: NetworkID::Mardunet.discriminant(),
                logical_name: Cow::Borrowed("mardunet"),
                hrp_suffix: Cow::Borrowed("tdx_24_"),
            },
            NetworkID::Gilganet => ScryptoNetworkDefinition {
                id:  NetworkID::Gilganet.discriminant(),
                logical_name: Cow::Borrowed("gilganet"),
                hrp_suffix: Cow::Borrowed("tdx_32_"),
            },
            NetworkID::Nergalnet =>  ScryptoNetworkDefinition {
                id:  NetworkID::Nergalnet.discriminant(),
                logical_name: Cow::Borrowed("nergalnet"),
                hrp_suffix: Cow::Borrowed("tdx_24_"),
            },
        }
    }
}
