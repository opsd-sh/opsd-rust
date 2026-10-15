use crate::types::{
    EmploymentDate, NiCategory, NonNegativePence, PreviousEmploymentAmounts, StarterDeclaration,
    StarterInformation, TaxBasis, TaxCodeNotice,
};
use serde::{Deserialize, Serialize};
/// Stored employment defaults and the scheme's active-year tax settings.
/// Null fields preserve incomplete legacy records; creating a new employment requires complete inputs.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmploymentPayrollSettings {
    pub monthly_gross_pay_pence: Option<NonNegativePence>,
    pub start_date: Option<EmploymentDate>,
    pub tax_code: Option<String>,
    pub tax_basis: Option<TaxBasis>,
    pub tax_year_start: Option<i32>,
    pub reporting_starter_declaration: Option<StarterDeclaration>,
    pub previous_employment: Option<PreviousEmploymentAmounts>,
    pub ni_category: Option<NiCategory>,
    pub starter_information: Option<StarterInformation>,
}

/// Updates to defaults and original starter evidence. Null/omitted fields are unchanged.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateEmploymentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_gross_pay_pence: Option<NonNegativePence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<EmploymentDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ni_category: Option<NiCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starter_information: Option<StarterInformation>,
}
/// HMRC notice updates cannot also change starter evidence or the start date.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyEmploymentTaxCodeNotice {
    pub tax_code_notice: TaxCodeNotice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_gross_pay_pence: Option<NonNegativePence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ni_category: Option<NiCategory>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    untagged,
    expecting = "payroll settings or a tax code notice without starter fields"
)]
pub enum UpdateEmploymentRequest {
    Settings(UpdateEmploymentSettings),
    TaxCodeNotice(ApplyEmploymentTaxCodeNotice),
}
