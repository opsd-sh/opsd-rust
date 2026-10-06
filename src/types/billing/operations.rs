use serde::{Deserialize, Serialize};

use super::wire;

/// Live payment setup status for a business or practice payer.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    from = "wire::GetBillingStatusResponse",
    into = "wire::GetBillingStatusResponse"
)]
pub struct GetBillingStatusResponse {
    /// Whether Stripe currently has a default payment method.
    /// This does not guarantee that a future charge will succeed.
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
