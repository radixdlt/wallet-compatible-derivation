use itertools::Itertools as _;

use crate::prelude::*;

#[derive(ZeroizeOnDrop, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BIP32Path(Vec<HDPathComponentValue>);

impl From<slip10::path::BIP32Path> for BIP32Path {
    fn from(value: slip10::path::BIP32Path) -> Self {
        Self(components_from(&value))
    }
}

impl std::fmt::Display for BIP32Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_bip32_string())
    }
}

impl BIP32Path {
    pub fn to_bip32_string(&self) -> String {
        let tail = self
            .clone()
            .into_iter()
            .map(|c| unhardened(c))
            .map(|v| format!("{}H", v))
            .join("/");
        format!("m/{}", tail)
    }
}

impl FromStr for BIP32Path {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        slip10::path::BIP32Path::from_str(s)
            .map(|p| p.into())
            .map_err(|_| Error::InvalidBIP32Path(s.to_string()))
    }
}

impl BIP32Path {
    pub(crate) fn inner(&self) -> slip10::path::BIP32Path {
        slip10::path::BIP32Path::from_str(&self.to_bip32_string())
            .expect("Should only have valid BIP32 path")
    }
    pub fn components(&self) -> Vec<HDPathComponentValue> {
        self.clone()
            .into_iter()
            .collect::<Vec<HDPathComponentValue>>()
    }
}

fn components_from(path: &slip10::path::BIP32Path) -> Vec<u32> {
    let mut vec = Vec::<HDPathComponentValue>::new();
    let mut components = path.clone();
    let len = path.depth();
    for _ in 0..len {
        vec.push(components.pop().expect("Should have asserted depth."));
    }
    vec.into_iter().rev().collect_vec()
}

impl IntoIterator for BIP32Path {
    type Item = HDPathComponentValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn string_roundtrip() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        let path: BIP32Path = s.parse().unwrap();
        assert_eq!(path.to_string(), s);
    }

    #[test]
    fn inner_roundtrip() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        let path: BIP32Path = s.parse().unwrap();
        let i = "m/44'/1022'/1'/525'/1460'/0'";
        assert_eq!(path.inner().to_string(), i);
        let path2: BIP32Path = i.parse().unwrap();
        assert_eq!(path2, path);
    }
}
