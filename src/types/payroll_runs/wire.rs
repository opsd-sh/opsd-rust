use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePayrollRunRequest {
    pub(super) paye_scheme_id: String,
    pub(super) payment_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PayrollRun {
    pub(super) id: String,
    pub(super) paye_scheme_id: String,
    pub(super) payment_date: String,
    pub(super) status: String,
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
}
