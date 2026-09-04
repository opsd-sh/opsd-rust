use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreateBusinessRequest {
    pub(super) name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Business {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct BusinessMember {
    pub(super) id: String,
    pub(super) email: String,
    pub(super) email_verified: bool,
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct UpdateBusinessMemberRequest {
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreateBusinessInvitationRequest {
    pub(super) email: String,
    pub(super) role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct OutgoingBusinessInvitation {
    pub(super) id: String,
    pub(super) email: String,
    pub(super) role: String,
    pub(super) status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) expires_at: OffsetDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PendingBusinessInvitation {
    pub(super) id: String,
    pub(super) business_id: String,
    pub(super) business_name: String,
    pub(super) invited_by_email: String,
    pub(super) role: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) expires_at: OffsetDateTime,
}
