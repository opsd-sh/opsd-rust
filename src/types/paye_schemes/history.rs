use crate::types::{
    PayeSchemeId, PayeSchemeRolloverId, SubmissionAttempt, SubmissionAttemptId, UserId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverHistoryResponse {
    pub rollover_id: PayeSchemeRolloverId,
    pub paye_scheme_id: PayeSchemeId,
    pub version: u64,
    pub current_attempt_id: Option<SubmissionAttemptId>,
    pub final_fps: RolloverStageHistory,
    pub new_year_eps: RolloverStageHistory,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverStageHistory {
    pub attempts: Vec<SubmissionAttempt>,
    pub actions: Vec<RolloverRecordedAction>,
    pub preparation_failures: Vec<RolloverPreparationFailure>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverRecordedAction {
    #[serde(flatten)]
    pub kind: RolloverRecordedActionKind,
    pub user_id: UserId,
    pub expected_version: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum RolloverRecordedActionKind {
    Retry { attempt_id: SubmissionAttemptId },
    Cancel { attempt_id: SubmissionAttemptId },
    ResumePolling { attempt_id: SubmissionAttemptId },
    ResumeReconciliation { attempt_id: SubmissionAttemptId },
    SendEps,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverPreparationFailure {
    #[serde(flatten)]
    pub operation: RolloverPreparationOperation,
    pub message: String,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub resolved_at: Option<OffsetDateTime>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum RolloverPreparationOperation {
    Retry { attempt_id: SubmissionAttemptId },
    PrepareEps,
}
