//! Public audit history, with safe diagnostics and no credentials or raw XML.
use super::{SubmissionRejectionReason, SubmissionState};
use crate::types::SubmissionAttemptId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionAttempt {
    pub id: SubmissionAttemptId,
    pub attempt_number: u32,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
    pub transaction_id: String,
    pub environment: SubmissionEnvironment,
    #[serde(flatten)]
    pub status: SubmissionAttemptStatus,
    pub initial_outcome: Option<RecordedOutcome<InitialSubmissionOutcome>>,
    pub business_outcome: Option<RecordedOutcome<BusinessSubmissionOutcome>>,
    pub cleanup: Option<SubmissionCleanupStatus>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmissionAttemptStatus {
    Unaccepted(SubmissionState),
    Accepted(AcceptedSubmissionStatus),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AcceptedSubmissionStatus {
    Accepted,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedOutcome<T> {
    #[serde(flatten)]
    pub outcome: T,
    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum InitialSubmissionOutcome {
    Acknowledged,
    NotSent,
    Rejected { reason: SubmissionRejectionReason },
    OutcomeUnknown { message: String },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum BusinessSubmissionOutcome {
    Accepted,
    Rejected { reason: SubmissionRejectionReason },
    ResubmissionRequested,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionEnvironment {
    Test,
    Live,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SubmissionCleanupStatus {
    Pending {
        #[serde(with = "time::serde::rfc3339")]
        next_action_at: OffsetDateTime,
    },
    Paused {
        #[serde(with = "time::serde::rfc3339")]
        paused_at: OffsetDateTime,
    },
    Complete {
        #[serde(with = "time::serde::rfc3339")]
        completed_at: OffsetDateTime,
    },
}
