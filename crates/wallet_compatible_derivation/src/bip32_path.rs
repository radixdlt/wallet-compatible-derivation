use crate::prelude::*;
use itertools::Itertools as _;

/// A [BIP-32][bip] hierarchical deterministic derivation path of depth `N`,
/// with which we can build a Radix Wallet compatible `AccountPath`.
///
/// [bip]: https://github.com/iqlusioninc/crates/tree/main/bip32
#[derive(Zeroize, ZeroizeOnDrop, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BIP32Path<const N: usize>(pub(crate) [HDPathComponentValue; N]);

impl<const N: usize> TryFrom<slip10::path::BIP32Path> for BIP32Path<N> {
    type Error = crate::Error;

    fn try_from(value: slip10::path::BIP32Path) -> Result<Self> {
        let components = components_from(&value);
        let depth = &components.len() as &usize;
        TryInto::<[HDPathComponentValue; N]>::try_into(components)
            .map_err(|_| Error::InvalidDepthOfBIP32Path {
                expected: N,
                found: *depth,
            })
            .map(|cs| Self(cs))
    }
}

impl<const N: usize> std::fmt::Display for BIP32Path<N> {
    /// Formats a `BIP32Path` with `N` many levels into a string joining each
    /// level with `/`, and printing `H` if it was hardened, as per BIP-32 standard
    /// notation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_bip32_string())
    }
}

impl<const N: usize> BIP32Path<N> {
    /// Formats a `BIP32Path` with `N` many levels into a string joining each
    /// level with `/`, and printing `H` if it was hardened, as per BIP-32 standard
    /// notation.
    pub fn to_bip32_string(&self) -> String {
        let tail = self
            .clone()
            .into_iter()
            .map(|c| unhardened(c))
            .map(|v| format!("{}H", v))
            .join("/");
        format!("m/{}", tail)
    }

    pub(crate) fn inner(&self) -> slip10::path::BIP32Path {
        slip10::path::BIP32Path::from_str(&self.to_bip32_string())
            .expect("Should only have valid BIP-32 path")
    }

    /// Returns each path component, layer, of the BIP-32 path as a vector.
    pub fn components(&self) -> Vec<HDPathComponentValue> {
        self.clone()
            .into_iter()
            .collect::<Vec<HDPathComponentValue>>()
    }
}

impl<const N: usize> FromStr for BIP32Path<N> {
    type Err = crate::Error;

    /// Tries to parse a BIP-32 string into a BIP32Path.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        slip10::path::BIP32Path::from_str(s)
            .map_err(|_| Error::InvalidBIP32Path(s.to_string()))
            .and_then(|p| p.try_into())
    }
}

/// The `slip10::path::BIP32Path` type does not impl Iterator, 
/// nor does it expose a `as_vec` method, so we need to build 
/// that ourselves.
fn components_from(path: &slip10::path::BIP32Path) -> Vec<u32> {
    let mut vec = Vec::<HDPathComponentValue>::new();
    let mut components = path.clone();
    let len = path.depth();
    for _ in 0..len {
        vec.push(components.pop().expect("Should have asserted depth."));
    }
    vec.into_iter().rev().collect_vec()
}

impl<const N: usize> IntoIterator for BIP32Path<N> {
    type Item = HDPathComponentValue;

    type IntoIter = std::array::IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;
    type SUT = BIP32Path<6>;

    #[test]
    fn string_roundtrip() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        let path: SUT = s.parse().unwrap();
        assert_eq!(path.to_string(), s);
    }

    #[test]
    fn inner_roundtrip() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        let path: SUT = s.parse().unwrap();
        let i = "m/44'/1022'/1'/525'/1460'/0'";
        assert_eq!(path.inner().to_string(), i);
        let path2: SUT = i.parse().unwrap();
        assert_eq!(path2, path);
    }
}
