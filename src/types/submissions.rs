//! Submission states returned by the API. No retries are performed automatically.
use super::SubmissionAttemptId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Transient Government Gateway password. Debug output is redacted; JSON
/// serialization deliberately exposes the value for the authenticated request.
#[derive(Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct HmrcPassword(String);
impl HmrcPassword {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
impl std::fmt::Debug for HmrcPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HmrcPassword([REDACTED])")
    }
}
impl<'de> Deserialize<'de> for HmrcPassword {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)
            .map(Self)
            .map_err(|_| serde::de::Error::custom("password must be a string"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SubmissionState {
    Sending,
    Pending,
    Reconciling,
    NotSent { message: String },
    CredentialsRequired { message: String },
    PollingPaused { message: String },
    ReconciliationPaused { message: String },
    NeedsAttention { message: String },
    Rejected { reason: SubmissionRejectionReason },
}
/// Safe, operator-facing diagnostics. Codes may expand as HMRC errors are classified.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionRejectionReason {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionProgress {
    pub attempt_id: SubmissionAttemptId,
    pub attempt_count: u32,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub initial_outcome_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub business_outcome_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub state: SubmissionState,
}
/// Supply the version from the response the operator acted on. Do not fetch a
/// newer version and silently retry after a conflict.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionedActionRequest {
    pub expected_version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetrySubmissionRequest {
    pub expected_version: u64,
    pub sender_id: String,
    pub password: HmrcPassword,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum SubmissionAction {
    Retry { attempt_id: SubmissionAttemptId },
    Reopen { attempt_id: SubmissionAttemptId },
    Cancel,
    ResumePolling { attempt_id: SubmissionAttemptId },
    ResumeReconciliation { attempt_id: SubmissionAttemptId },
}
mod history;
pub use history::*;
