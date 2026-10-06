use super::BillingMonth;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// An active overdue-invoice restriction, independent of agreement status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PayrollSuspension {
    #[serde(with = "time::serde::rfc3339")]
    pub suspended_at: OffsetDateTime,
}

/// POST response. Subscribing alone does not start billing or clear suspension.
/// Use GET to read any independent suspension state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SubscribeToPayrollResponse {
    Subscribed {
        #[serde(with = "time::serde::rfc3339")]
        subscribed_at: OffsetDateTime,
        /// Null before first billable payroll; otherwise YYYY-MM.
        billing_start_month: Option<BillingMonth>,
    },
}

/// Standalone payroll agreement and independent suspension. Paying debt does
/// not restart a cancelled agreement; cancellation does not clear suspension.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum GetPayrollSubscriptionResponse {
    NeverSubscribed {
        suspension: Option<PayrollSuspension>,
    },
    Subscribed {
        #[serde(with = "time::serde::rfc3339")]
        subscribed_at: OffsetDateTime,
        billing_start_month: Option<BillingMonth>,
        suspension: Option<PayrollSuspension>,
    },
    Cancelled {
        #[serde(with = "time::serde::rfc3339")]
        subscribed_at: OffsetDateTime,
        billing_start_month: Option<BillingMonth>,
        #[serde(with = "time::serde::rfc3339")]
        cancelled_at: OffsetDateTime,
        suspension: Option<PayrollSuspension>,
    },
}

/// GET/POST client payroll response. Suspension applies to the whole practice;
/// enabling one client does not start billing or restore a suspended practice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PracticePayrollResponse {
    NeverEnabled {
        suspension: Option<PayrollSuspension>,
    },
    Enabled {
        #[serde(with = "time::serde::rfc3339")]
        enabled_at: OffsetDateTime,
        billing_start_month: Option<BillingMonth>,
        suspension: Option<PayrollSuspension>,
    },
    Disabled {
        #[serde(with = "time::serde::rfc3339")]
        enabled_at: OffsetDateTime,
        billing_start_month: Option<BillingMonth>,
        #[serde(with = "time::serde::rfc3339")]
        disabled_at: OffsetDateTime,
        suspension: Option<PayrollSuspension>,
    },
}
