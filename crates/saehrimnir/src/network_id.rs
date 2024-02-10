use radix_engine_common::network::NetworkDefinition;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetworkID {
    Mainnet,
    Stokenet,
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