use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{BillingMonth, BillingPageSize};
use crate::types::{BusinessId, InvoiceId};

/// Newest-first pagination. Pass next_before back as before to continue.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListInvoicesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<BillingMonth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<BillingPageSize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListInvoicesResponse {
    pub invoices: Vec<InvoiceSummary>,
    pub next_before: Option<BillingMonth>,
}

/// Saved invoice metadata, not a live Stripe balance.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvoiceSummary {
    pub id: InvoiceId,
    pub billing_month: BillingMonth,
    pub currency: String,
    /// Original charges before discounts and tax, not the remaining balance.
    pub subtotal_pence: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub status: InvoiceStatus,
}

/// Open does not imply that payment retries have been exhausted.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    PendingDelivery,
    AwaitingStatus,
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

/// Pass next_offset back as offset to continue. Defaults: offset 0, limit 25.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListInvoiceLinesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<BillingPageSize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListInvoiceLinesResponse {
    pub lines: Vec<InvoiceLine>,
    pub next_offset: Option<u32>,
}

/// Local detailed charges, including the originating client business for a
/// consolidated practice invoice. Amounts are GBP pence before discounts/tax.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvoiceLine {
    pub business_id: BusinessId,
    /// Current business name, not a historical snapshot.
    pub business_name: String,
    pub charge_kind: PayrollChargeKind,
    pub usage_month: BillingMonth,
    pub quantity: i64,
    pub unit_amount_pence: i64,
    pub amount_pence: i64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayrollChargeKind {
    BaseFee,
    PayrollEmployee,
}
