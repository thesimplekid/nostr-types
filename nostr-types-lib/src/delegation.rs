use super::{EventKind, PrivateKey, PublicKey, PublicKeyHex, Signature, SignatureHex, Unixtime};
use crate::Error;
use serde::de::Error as DeError;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

/// Delegation information for an Event
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventDelegation {
    /// The event was not delegated
    NotDelegated,

    /// The delegation was invalid (with reason)
    InvalidDelegation(String),

    /// The event was delegated and is valid (with pubkey of delegator)
    DelegatedBy(PublicKey),
}

/// Conditions of delegation
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct DelegationConditions {
    /// If the delegation is only for a given event kind
    pub kind: Option<EventKind>,

    /// If the delegation is only for events created after a certain time
    pub created_after: Option<Unixtime>,

    /// If the delegation is only for events created before a certain time
    pub created_before: Option<Unixtime>,
}

impl DelegationConditions {
    /// Convert to string form
    pub fn as_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(kind) = self.kind {
            parts.push(format!("kind={}", u64::from(kind)));
        }
        if let Some(created_after) = self.created_after {
            parts.push(format!("created_at>{}", created_after.0));
        }
        if let Some(created_before) = self.created_before {
            parts.push(format!("created_at<{}", created_before.0));
        }
        parts.join("&")
    }

    /// Convert from string from
    pub fn try_from_str(s: &str) -> Result<DelegationConditions, Error> {
        let mut output: DelegationConditions = Default::default();

        let parts = s.split('&');
        for part in parts {
            if let Some(kindstr) = part.strip_prefix("kind=") {
                let event_num = kindstr.parse::<u64>()?;
                let event_kind: EventKind = From::from(event_num);
                output.kind = Some(event_kind);
            }
            if let Some(timestr) = part.strip_prefix("created_at>") {
                let time = timestr.parse::<i64>()?;
                output.created_after = Some(Unixtime(time));
            }
            if let Some(timestr) = part.strip_prefix("created_at<") {
                let time = timestr.parse::<i64>()?;
                output.created_before = Some(Unixtime(time));
            }
        }
        Ok(output)
    }

    #[allow(dead_code)]
    pub(crate) fn mock() -> DelegationConditions {
        DelegationConditions {
            kind: Some(EventKind::Repost),
            created_after: Some(Unixtime(1677700000)),
            created_before: None,
        }
    }

    /// Generate the signature part of a Delegation tag
    pub fn generate_signature(
        &self,
        pubkey: PublicKeyHex,
        private_key: PrivateKey,
    ) -> Result<SignatureHex, Error> {
        let input = format!("nostr:delegation:{}:{}", pubkey, self.as_string());
        let signature = private_key.sign(input.as_bytes())?;
        Ok(signature.into())
    }

    /// Verify the signature part of a Delegation tag
    pub fn verify_signature(
        &self,
        pubkey_delegater: &PublicKey,
        pubkey_delegatee: &PublicKey,
        signature: Signature,
    ) -> Result<(), Error> {
        let input = format!(
            "nostr:delegation:{}:{}",
            pubkey_delegatee.as_hex_string(),
            self.as_string()
        );
        pubkey_delegater.verify(input.as_bytes(), signature)
    }
}

impl Serialize for DelegationConditions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_string())
    }
}

impl<'de> Deserialize<'de> for DelegationConditions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DelegationConditionsVisitor)
    }
}

struct DelegationConditionsVisitor;

impl Visitor<'_> for DelegationConditionsVisitor {
    type Value = DelegationConditions;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A string")
    }

    fn visit_str<E>(self, v: &str) -> Result<DelegationConditions, E>
    where
        E: DeError,
    {
        DelegationConditions::try_from_str(v).map_err(|e| E::custom(format!("{e}")))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Tag;

    test_serde! {DelegationConditions, test_delegation_conditions_serde}

    #[test]
    fn test_sign_delegation_verify_delegation_signature() {
        let dc = DelegationConditions::try_from_str(
            "kind=1&created_at>1674834236&created_at<1677426236",
        )
        .unwrap();
        let delegator_private_key = PrivateKey::try_from_hex_string(
            "ee35e8bb71131c02c1d7e73231daa48e9953d329a4b701f7133c8f46dd21139c",
        )
        .unwrap();
        let delegator_public_key = delegator_private_key.public_key();
        let delegatee_public_key = PublicKey::try_from_hex_string(
            "477318cfb5427b9cfc66a9fa376150c1ddbc62115ae27cef72417eb959691396",
        )
        .unwrap();

        let signature = dc
            .generate_signature(
                PublicKeyHex::from(delegatee_public_key),
                delegator_private_key,
            )
            .unwrap();

        // signature is changing, validate by verify method
        let sig = Signature::try_from(signature).unwrap();
        let verify_result = dc.verify_signature(&delegator_public_key, &delegatee_public_key, sig);
        assert!(verify_result.is_ok());
    }

    #[test]
    fn test_delegation_tag_parse_and_verify() {
        let tag_str = "[\"delegation\",\"1a459a8a6aa6441d480ba665fb8fb21a4cfe8bcacb7d87300f8046a558a3fce4\",\"kind=1&created_at>1676067553&created_at<1678659553\",\"369aed09c1ad52fceb77ecd6c16f2433eac4a3803fc41c58876a5b60f4f36b9493d5115e5ec5a0ce6c3668ffe5b58d47f2cbc97233833bb7e908f66dbbbd9d36\"]";
        let dt = serde_json::from_str::<Tag>(tag_str).unwrap();
        if let Tag::Delegation {
            pubkey,
            conditions,
            sig,
        } = dt
        {
            assert_eq!(
                conditions.as_string(),
                "kind=1&created_at>1676067553&created_at<1678659553"
            );

            let delegatee_public_key = PublicKey::try_from_hex_string(
                "bea8aeb6c1657e33db5ac75a83910f77e8ec6145157e476b5b88c6e85b1fab34",
            )
            .unwrap();

            let verify_result = conditions.verify_signature(
                &PublicKey::try_from_hex_string(pubkey.as_str()).unwrap(),
                &delegatee_public_key,
                Signature::try_from(sig).unwrap(),
            );
            assert!(verify_result.is_ok());
        } else {
            panic!("Incorrect tag type")
        }
    }

    #[test]
    fn test_from_str() {
        let str = "kind=1&created_at>1000000&created_at<2000000";
        let dc = DelegationConditions::try_from_str(str).unwrap();
        assert_eq!(dc.as_string(), str);
    }

    #[test]
    fn test_as_string() {
        let dc = DelegationConditions {
            kind: Some(EventKind::TextNote),
            created_before: Some(Unixtime(2000000)),
            created_after: Some(Unixtime(1000000)),
        };
        assert_eq!(
            dc.as_string(),
            "kind=1&created_at>1000000&created_at<2000000"
        );
    }
}
