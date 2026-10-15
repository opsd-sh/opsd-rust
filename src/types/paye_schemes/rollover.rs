use crate::types::{EmploymentAllowance, EmploymentId, PayeSchemeRolloverId, TaxBasis};
use serde::{Deserialize, Serialize};
/// Latest applicable new-year instruction for a continuing employment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloverTaxCodeNotice {
    pub employment_id: EmploymentId,
    pub tax_code: String,
    pub tax_basis: TaxBasis,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloverPayeSchemeRequest {
    pub tax_year_start: i32,
    pub employment_allowance: EmploymentAllowance,
    /// Required. Empty means the operator checked and has no new-year notices.
    pub tax_code_notices: Vec<RolloverTaxCodeNotice>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayeSchemeRolloverStatus {
    Closing,
    Completed,
}
/// Scheme rollover result returned by the server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloverPayeSchemeResponse {
    pub id: PayeSchemeRolloverId,
    pub from_tax_year_start: i32,
    pub tax_year_start: i32,
    pub employment_allowance: EmploymentAllowance,
    pub status: PayeSchemeRolloverStatus,
    pub tax_code_notices: Vec<RolloverTaxCodeNotice>,
}
