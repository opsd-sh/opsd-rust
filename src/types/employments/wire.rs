use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreateEmploymentRequest {
    pub(super) employee_id: String,
    pub(super) paye_scheme_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Employment {
    pub(super) id: String,
    pub(super) employee_id: String,
    pub(super) paye_scheme_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
