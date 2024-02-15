use radix_engine_common::network::NetworkDefinition;
use strum_macros::{Display, EnumString};

use crate::prelude::*;

/// The network on which an account can be used. For `Mainnet` the value `1` is used,
/// for `Stokenet` the value `2` is used.
///
/// See [Babylon-node repo][node] for more details.
///
/// [node]: https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L82-L98
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, enum_iterator::Sequence,
)]
pub enum NetworkID {
    /// The Radix mainnet.
    #[strum(ascii_case_insensitive)]
    Mainnet,

    /// A public facing testnet.
    #[strum(ascii_case_insensitive)]
    Stokenet,
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
}

impl TryFrom<HDPathComponentValue> for NetworkID {
    type Error = crate::Error;

    /// Tries to create a `NetworkID` from a path component, the value
    /// passed MUST be non-hardened / unhardened.
    /// 
    /// See `unhardened` function.
    fn try_from(value: HDPathComponentValue) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NetworkID::Mainnet),
            2 => Ok(NetworkID::Stokenet),
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
        }
    }

    /// A network definition used by this library to form bech32 encoded
    /// addresses.
    pub(crate) fn network_definition(&self) -> NetworkDefinition {
        match self {
            NetworkID::Mainnet => NetworkDefinition::mainnet(),
            NetworkID::Stokenet => NetworkDefinition::stokenet(),
        }
    }
}
