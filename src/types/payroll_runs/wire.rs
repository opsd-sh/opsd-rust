use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePayrollRunRequest {
    pub(super) paye_scheme_id: String,
    pub(super) pay_schedule_id: String,
    pub(super) payment_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PayrollRun {
    pub(super) id: String,
    pub(super) paye_scheme_id: String,
    pub(super) pay_schedule_id: String,
    pub(super) payment_date: String,
    #[serde(flatten)]
    pub(super) status: super::PayrollRunStatus,
    pub(super) version: u64,
    pub(super) available_actions: Vec<crate::types::SubmissionAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) preparation_failure: Option<String>,
    pub(super) revision_id: Option<crate::types::PayrollRevisionId>,
    pub(super) employment_count: u64,
    #[serde(with = "time::serde::rfc3339::option")]
    pub(super) finalized_at: Option<OffsetDateTime>,
    pub(super) finalized_by_user_id: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PayrollRunDetails {
    #[serde(flatten)]
    pub(super) run: PayrollRun,
    pub(super) employment_ids: Vec<String>,
    pub(super) payments: Vec<super::PayrollPayment>,
    pub(super) calculation: Option<super::PayrollPreviewResponse>,
}
