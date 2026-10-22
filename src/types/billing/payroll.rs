use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// An active overdue-invoice restriction, independent of agreement status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PayrollSuspension {
    #[serde(with = "time::serde::rfc3339")]
    pub suspended_at: OffsetDateTime,
}

/// POST response. Subscribing starts monthly base fees; suspension is independent.
/// Use GET to read any independent suspension state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SubscribeToPayrollResponse {
    Subscribed {
        subscription_version: u64,
        #[serde(with = "time::serde::rfc3339")]
        subscribed_at: OffsetDateTime,
    },
}

/// Standalone payroll agreement and independent suspension. Paying debt does
/// not restart a cancelled agreement; cancellation does not clear suspension.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum GetPayrollSubscriptionResponse {
    NeverSubscribed {
        subscription_version: u64,
        suspension: Option<PayrollSuspension>,
    },
    Subscribed {
        subscription_version: u64,
        #[serde(with = "time::serde::rfc3339")]
        subscribed_at: OffsetDateTime,
        suspension: Option<PayrollSuspension>,
    },
    Cancelled {
        subscription_version: u64,
        #[serde(with = "time::serde::rfc3339")]
        subscribed_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        cancelled_at: OffsetDateTime,
        suspension: Option<PayrollSuspension>,
    },
}

/// GET/POST client payroll response. Suspension applies to the whole practice;
/// enabling a client starts monthly base fees without lifting suspension.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PracticePayrollResponse {
    NeverEnabled {
        subscription_version: u64,
        suspension: Option<PayrollSuspension>,
    },
    Enabled {
        subscription_version: u64,
        #[serde(with = "time::serde::rfc3339")]
        enabled_at: OffsetDateTime,
        suspension: Option<PayrollSuspension>,
    },
    Disabled {
        subscription_version: u64,
        #[serde(with = "time::serde::rfc3339")]
        enabled_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        disabled_at: OffsetDateTime,
        suspension: Option<PayrollSuspension>,
    },
}

/// Version read from the subscription status; prevents stale enable/cancel actions.
pub type PayrollSubscriptionAction = crate::types::VersionedActionRequest;
