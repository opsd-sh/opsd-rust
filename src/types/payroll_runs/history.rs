use crate::types::{
    PayrollPaymentDate, PayrollRevisionId, PayrollRunId, SubmissionAttempt, SubmissionAttemptId,
    UserId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollRunHistoryResponse {
    pub payroll_run_id: PayrollRunId,
    pub version: u64,
    pub current_revision_id: Option<PayrollRevisionId>,
    pub current_attempt_id: Option<SubmissionAttemptId>,
    pub revisions: Vec<PayrollRevisionHistory>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollRevisionHistory {
    pub id: PayrollRevisionId,
    pub revision_number: u32,
    pub payment_date: PayrollPaymentDate,
    #[serde(with = "time::serde::rfc3339")]
    pub finalized_at: OffsetDateTime,
    pub finalized_by_user_id: UserId,
    pub attempts: Vec<SubmissionAttempt>,
    pub actions: Vec<PayrollRecordedAction>,
    pub preparation_failures: Vec<PayrollPreparationFailure>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayrollRecordedActionKind {
    Retry,
    Reopen,
    Cancel,
    ResumePolling,
    ResumeReconciliation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollRecordedAction {
    pub action: PayrollRecordedActionKind,
    pub attempt_id: SubmissionAttemptId,
    pub user_id: UserId,
    pub expected_version: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayrollPreparationOperation {
    Retry,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollPreparationFailure {
    pub attempt_id: SubmissionAttemptId,
    pub operation: PayrollPreparationOperation,
    pub message: String,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub resolved_at: Option<OffsetDateTime>,
}
