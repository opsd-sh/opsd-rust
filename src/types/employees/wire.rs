use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreateEmployeeRequest {
    pub(super) forenames: Vec<String>,
    pub(super) surname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct UpdateEmployeeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) forenames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) surname: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Employee {
    pub(super) id: String,
    pub(super) forenames: Vec<String>,
    pub(super) surname: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
