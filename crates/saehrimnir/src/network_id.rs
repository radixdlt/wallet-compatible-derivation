use radix_engine_common::network::NetworkDefinition;
use strum_macros::{Display, EnumString};
use zeroize::Zeroize;

use crate::prelude::*;

#[derive(
    ZeroizeOnDrop,
    Zeroize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    enum_iterator::Sequence,
)]
pub enum NetworkID {
    #[strum(ascii_case_insensitive)]
    Mainnet,
    #[strum(ascii_case_insensitive)]
    Stokenet,
}

impl NetworkID {
    pub fn all() -> Vec<NetworkID> {
        enum_iterator::all::<NetworkID>().collect::<Vec<_>>()
    }
}

impl TryFrom<HDPathComponentValue> for NetworkID {
    type Error = crate::Error;

    fn try_from(value: HDPathComponentValue) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NetworkID::Mainnet),
            2 => Ok(NetworkID::Stokenet),
            _ => Err(Error::UnsupportedOrUnknownNetworkID(value)),
        }
    }
}

impl NetworkID {
    pub fn hardened_hd_component_value(&self) -> HDPathComponentValue {
        match self {
            NetworkID::Mainnet => harden(1),
            NetworkID::Stokenet => harden(2),
        }
    }

    pub fn network_definition(&self) -> NetworkDefinition {
        match self {
            NetworkID::Mainnet => NetworkDefinition::mainnet(),
            NetworkID::Stokenet => NetworkDefinition::stokenet(),
        }
    }
}
