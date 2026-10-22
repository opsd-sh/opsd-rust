use crate::types::{HmrcPassword, LateReportingReason};
use serde::{Deserialize, Serialize};
/// Finalise and submit a draft or reopened run. Read its state afterwards;
/// returning successfully does not establish HMRC business acceptance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizePayrollRunRequest {
    pub expected_version: u64,
    pub sender_id: String,
    pub password: HmrcPassword,
    pub late_reporting_reason: Option<LateReportingReason>,
}
pub type RetryPayrollRunRequest = crate::types::RetrySubmissionRequest;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum ResumePayrollRunRequest {
    Polling {
        expected_version: u64,
    },
    Reconciliation {
        expected_version: u64,
        password: HmrcPassword,
    },
}
