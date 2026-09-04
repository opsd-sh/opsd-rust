use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePayeSchemeRequest {
    pub(super) name: String,
    pub(super) employer_reference: String,
    pub(super) accounts_office_reference: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PayeScheme {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) employer_reference: String,
    pub(super) accounts_office_reference: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
