use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreatePayeSchemeRequest {
    pub(super) company_utr: Option<String>,

    pub(super) name: String,
    pub(super) employer_reference: String,
    pub(super) accounts_office_reference: String,
    pub(super) tax_year_start: i32,
    pub(super) employment_allowance: crate::types::EmploymentAllowance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct PayeScheme {
    pub(super) company_utr: Option<String>,
    pub(super) rollover_generation: u64,

    pub(super) id: String,
    pub(super) name: String,
    pub(super) employer_reference: String,
    pub(super) accounts_office_reference: String,
    pub(super) tax_year_start: Option<i32>,
    pub(super) employment_allowance: Option<crate::types::EmploymentAllowance>,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
