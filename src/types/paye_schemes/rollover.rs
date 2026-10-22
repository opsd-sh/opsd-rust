use crate::types::{
    EmploymentAllowance, EmploymentId, HmrcPassword, PayeSchemeId, PayeSchemeRolloverId,
    SubmissionAttemptId, TaxBasis,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
/// Latest applicable new-year instruction for a continuing employment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloverTaxCodeNotice {
    pub employment_id: EmploymentId,
    pub tax_code: String,
    pub tax_basis: TaxBasis,
}
/// Start a rollover using the generation read from the scheme. After a lost
/// response, list existing rollovers instead of silently submitting a new one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloverPayeSchemeRequest {
    pub expected_generation: u64,
    pub sender_id: String,
    pub password: HmrcPassword,
    pub tax_year_start: i32,
    pub employment_allowance: EmploymentAllowance,
    /// Required. Empty means the operator checked and has no new-year notices.
    pub tax_code_notices: Vec<RolloverTaxCodeNotice>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverPayeSchemeResponse {
    pub id: PayeSchemeRolloverId,
    pub paye_scheme_id: PayeSchemeId,
    pub version: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub from_tax_year_start: i32,
    pub tax_year_start: i32,
    pub employment_allowance: EmploymentAllowance,
    pub tax_code_notices: Vec<RolloverTaxCodeNotice>,
    #[serde(flatten)]
    pub status: PayeSchemeRolloverStatus,
    pub preparation_failure: Option<String>,
    pub available_actions: Vec<RolloverAction>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PayeSchemeRolloverStatus {
    InProgress {
        progress: RolloverProgress,
    },
    Cancelled {
        final_fps: CancelledRolloverSubmission,
    },
    Complete {
        final_fps: AcceptedRolloverSubmission,
        eps: AcceptedRolloverSubmission,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum RolloverProgress {
    FinalFps {
        final_fps: RolloverSubmission,
    },
    NewYearEps {
        final_fps: AcceptedRolloverSubmission,
        eps: RolloverEpsProgress,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolloverEpsProgress {
    BeforeDispatch(EpsBeforeDispatch),
    Submission(RolloverSubmission),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EpsBeforeDispatch {
    AwaitingDispatch,
    PreparationFailed { message: String },
    CredentialsRequiredToSend { message: String },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverSubmission {
    pub attempt_id: SubmissionAttemptId,
    pub attempt_count: u32,
    pub status: RolloverSubmissionStatus,
    pub message: Option<String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloverSubmissionStatus {
    Sending,
    Pending,
    Reconciling,
    NotSent,
    Rejected,
    CredentialsRequired,
    PollingPaused,
    ReconciliationPaused,
    NeedsAttention,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AcceptedRolloverSubmission {
    Accepted {
        attempt_id: SubmissionAttemptId,
        attempt_count: u32,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CancelledRolloverSubmission {
    NotSent {
        attempt_id: SubmissionAttemptId,
        attempt_count: u32,
        message: Option<String>,
    },
    Rejected {
        attempt_id: SubmissionAttemptId,
        attempt_count: u32,
        message: Option<String>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum RolloverAction {
    Retry { attempt_id: SubmissionAttemptId },
    Cancel,
    ResumePolling { attempt_id: SubmissionAttemptId },
    ResumeReconciliation { attempt_id: SubmissionAttemptId },
    SendEps,
}
pub type RetryRolloverRequest = crate::types::RetrySubmissionRequest;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum ResumeRolloverRequest {
    Polling {
        expected_version: u64,
    },
    Reconciliation {
        expected_version: u64,
        password: HmrcPassword,
    },
    SendEps {
        expected_version: u64,
        sender_id: String,
        password: HmrcPassword,
    },
}
