use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PracticeBusiness {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) practice_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePracticeRequest {
    pub(super) name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Practice {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PracticeMember {
    pub(super) id: String,
    pub(super) email: String,
    pub(super) email_verified: bool,
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct UpdatePracticeMemberRequest {
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePracticeInvitationRequest {
    pub(super) email: String,
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct OutgoingPracticeInvitation {
    pub(super) id: String,
    pub(super) email: String,
    pub(super) role: String,
    pub(super) status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) expires_at: OffsetDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PendingPracticeInvitation {
    pub(super) id: String,
    pub(super) practice_id: String,
    pub(super) practice_name: String,
    pub(super) invited_by_email: String,
    pub(super) role: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) expires_at: OffsetDateTime,
}
