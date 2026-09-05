use serde::{Deserialize, Serialize};

use super::wire;

/// Business-level payment setup status returned by the public API.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    from = "wire::GetBillingStatusResponse",
    into = "wire::GetBillingStatusResponse"
)]
pub struct GetBillingStatusResponse {
    /// Whether payment setup has been confirmed for the business.
    /// This is not a live check of the saved method or a promise that a future
    /// charge will succeed. A false value can mean confirmation is still pending.
    pub payment_method_saved: bool,
}

impl From<wire::GetBillingStatusResponse> for GetBillingStatusResponse {
    fn from(value: wire::GetBillingStatusResponse) -> Self {
        Self {
            payment_method_saved: value.payment_method_saved,
        }
    }
}

impl From<GetBillingStatusResponse> for wire::GetBillingStatusResponse {
    fn from(value: GetBillingStatusResponse) -> Self {
        Self {
            payment_method_saved: value.payment_method_saved,
        }
    }
}
