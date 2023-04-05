use crate::Error;
use bech32::{FromBase32, ToBase32};
use derive_more::{AsMut, AsRef, Deref, Display, From, FromStr, Into};
use serde::de::{Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An event identifier, constructed as a SHA256 hash of the event fields according to NIP-01
#[derive(
    AsMut, AsRef, Clone, Copy, Debug, Deref, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct Id(pub [u8; 32]);

impl Id {
    /// Render into a hexadecimal string
    ///
    /// Consider converting `.into()` an `IdHex` which is a wrapped type rather than a naked `String`
    pub fn as_hex_string(&self) -> String {
        hex::encode(self.0)
    }

    /// Create from a hexadecimal string
    pub fn try_from_hex_string(v: &str) -> Result<Id, Error> {
        let vec: Vec<u8> = hex::decode(v)?;
        Ok(Id(vec
            .try_into()
            .map_err(|_| Error::WrongLengthHexString)?))
    }

    /// Export as a bech32 encoded string ("note")
    pub fn as_bech32_string(&self) -> String {
        bech32::encode("note", self.0.to_vec().to_base32(), bech32::Variant::Bech32).unwrap()
    }

    /// Import from a bech32 encoded string ("note")
    pub fn try_from_bech32_string(s: &str) -> Result<Id, Error> {
        let data = bech32::decode(s)?;
        if data.0 != "note" {
            Err(Error::WrongBech32("note".to_string(), data.0))
        } else {
            let decoded = Vec::<u8>::from_base32(&data.1)?;
            if decoded.len() != 32 {
                Err(Error::InvalidId)
            } else {
                match <[u8; 32]>::try_from(decoded) {
                    Ok(array) => Ok(Id(array)),
                    _ => Err(Error::InvalidId),
                }
            }
        }
    }

    // Mock data for testing
    #[allow(dead_code)]
    pub(crate) fn mock() -> Id {
        Id::try_from_hex_string("5df64b33303d62afc799bdc36d178c07b2e1f0d824f31b7dc812219440affab6")
            .unwrap()
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self.0))
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IdVisitor)
    }
}

struct IdVisitor;

impl Visitor<'_> for IdVisitor {
    type Value = Id;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a hexadecimal string representing 32 bytes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Id, E>
    where
        E: serde::de::Error,
    {
        let vec: Vec<u8> = hex::decode(v).map_err(|e| serde::de::Error::custom(format!("{e}")))?;

        Ok(Id(vec.try_into().map_err(|e: Vec<u8>| {
            E::custom(format!(
                "Id is not 32 bytes long. Was {} bytes long",
                e.len()
            ))
        })?))
    }
}

/// An event identifier, constructed as a SHA256 hash of the event fields according to NIP-01, as a hex string
///
/// You can convert from an `Id` into this with `From`/`Into`.  You can convert this back to an `Id` with `TryFrom`/`TryInto`.
#[derive(
    AsMut,
    AsRef,
    Clone,
    Debug,
    Deref,
    Deserialize,
    Display,
    Eq,
    From,
    FromStr,
    Hash,
    Into,
    PartialEq,
    Serialize,
)]
pub struct IdHex(String);

impl IdHex {
    // Mock data for testing
    #[allow(dead_code)]
    pub(crate) fn mock() -> IdHex {
        From::from(Id::mock())
    }

    /// Try from &str
    pub fn try_from_str(s: &str) -> Result<IdHex, Error> {
        Self::try_from_string(s.to_owned())
    }

    /// Try from String
    pub fn try_from_string(s: String) -> Result<IdHex, Error> {
        if s.len() != 64 {
            return Err(Error::InvalidId);
        }
        let vec: Vec<u8> = hex::decode(&s)?;
        if vec.len() != 32 {
            return Err(Error::InvalidId);
        }
        Ok(IdHex(s))
    }

    /// As &str
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Into String
    pub fn into_string(self) -> String {
        self.0
    }

    /// Prefix of
    pub fn prefix(&self, mut chars: usize) -> IdHexPrefix {
        if chars > 64 {
            chars = 64;
        }
        IdHexPrefix(self.0.get(0..chars).unwrap().to_owned())
    }
}

impl TryFrom<&str> for IdHex {
    type Error = Error;

    fn try_from(s: &str) -> Result<IdHex, Error> {
        IdHex::try_from_str(s)
    }
}

impl From<Id> for IdHex {
    fn from(i: Id) -> IdHex {
        IdHex(i.as_hex_string())
    }
}

impl From<IdHex> for Id {
    fn from(h: IdHex) -> Id {
        // could only fail if IdHex is invalid
        Id::try_from_hex_string(&h.0).unwrap()
    }
}

/// An event identifier prefix, constructed as a SHA256 hash of the event fields according to NIP-01, as a hex string
#[derive(
    AsMut,
    AsRef,
    Clone,
    Debug,
    Deref,
    Deserialize,
    Display,
    Eq,
    From,
    FromStr,
    Hash,
    Into,
    PartialEq,
    Serialize,
)]
pub struct IdHexPrefix(String);

impl IdHexPrefix {
    // Mock data for testing
    #[allow(dead_code)]
    pub(crate) fn mock() -> IdHexPrefix {
        IdHexPrefix("a872bee017".to_owned())
    }

    /// Try from &str
    pub fn try_from_str(s: &str) -> Result<IdHexPrefix, Error> {
        Self::try_from_string(s.to_owned())
    }

    /// Try from String
    pub fn try_from_string(s: String) -> Result<IdHexPrefix, Error> {
        if s.len() > 64 {
            return Err(Error::InvalidIdPrefix);
        }
        if s.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err(Error::InvalidPublicKeyPrefix);
        }
        // let vec: Vec<u8> = hex::decode(&s)?;
        // if vec.len() > 32 {
        //     return Err(Error::InvalidIdPrefix);
        // }
        Ok(IdHexPrefix(s))
    }

    /// As &str
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Into String
    pub fn into_string(self) -> String {
        self.0
    }

    /// Matches a PublicKeyhex
    pub fn matches(&self, id: &IdHex) -> bool {
        id.0.starts_with(&self.0)
    }
}

impl From<IdHex> for IdHexPrefix {
    fn from(id: IdHex) -> IdHexPrefix {
        IdHexPrefix(id.0)
    }
}

impl TryFrom<&str> for IdHexPrefix {
    type Error = Error;

    fn try_from(s: &str) -> Result<IdHexPrefix, Error> {
        IdHexPrefix::try_from_str(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    test_serde! {Id, test_id_serde}
    test_serde! {IdHex, test_id_hex_serde}
    test_serde! {IdHexPrefix, test_id_hex_prefix_serde}

    #[test]
    fn test_id_bech32() {
        let bech32 = Id::mock().as_bech32_string();
        println!("{bech32}");
        assert_eq!(Id::mock(), Id::try_from_bech32_string(&bech32).unwrap());
    }
}
