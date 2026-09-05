use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct GetBillingStatusResponse {
    pub(super) payment_method_saved: bool,
}
