use serde::{Deserialize, Serialize};

use super::{BillingPageSize, StripePaymentMethodId};

/// Pass next_starting_after back as starting_after to continue.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListPaymentMethodsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<StripePaymentMethodId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<BillingPageSize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListPaymentMethodsResponse {
    pub payment_methods: Vec<PaymentMethod>,
    pub next_starting_after: Option<StripePaymentMethodId>,
}

/// A live Stripe payment method. Presence does not guarantee successful payment.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentMethod {
    pub id: StripePaymentMethodId,
    /// Provider type, including non-card types; deliberately not a closed enum.
    #[serde(rename = "type")]
    pub payment_method_type: String,
    /// Stripe's customer invoice default, not a local delivery snapshot.
    pub is_default: bool,
    /// None for non-card methods.
    pub card: Option<PaymentMethodCard>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentMethodCard {
    pub brand: String,
    pub last4: String,
    pub expiry_month: u8,
    pub expiry_year: u16,
}

/// Select a saved method belonging to this payer. This does not charge it,
/// retry an invoice or clear a suspension.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetDefaultPaymentMethodRequest {
    pub payment_method_id: StripePaymentMethodId,
}
