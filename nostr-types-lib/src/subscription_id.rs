use derive_more::{AsMut, AsRef, Deref, From, FromStr, Into};
use serde::{Deserialize, Serialize};

/// A random client-chosen string used to refer to a subscription
#[derive(
    AsMut, AsRef, Clone, Debug, Deref, Deserialize, Eq, From, FromStr, Into, PartialEq, Serialize,
)]
pub struct SubscriptionId(pub String);

impl SubscriptionId {
    // Mock data for testing
    #[allow(dead_code)]
    pub(crate) fn mock() -> SubscriptionId {
        SubscriptionId("lk234js09".to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    test_serde! {SubscriptionId, test_subscription_id_serde}
}
