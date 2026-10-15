use crate::types::PayFrequency;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePayScheduleRequest {
    pub(super) paye_scheme_id: String,
    pub(super) name: String,
    pub(super) frequency: PayFrequency,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PaySchedule {
    pub(super) id: String,
    pub(super) paye_scheme_id: String,
    pub(super) name: String,
    pub(super) frequency: PayFrequency,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
