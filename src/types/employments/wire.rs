use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreateEmploymentRequest {
    pub(super) employee_id: String,
    pub(super) paye_scheme_id: String,
    pub(super) pay_schedule_id: String,
    pub(super) monthly_gross_pay_pence: crate::types::NonNegativePence,
    pub(super) start_date: crate::types::EmploymentDate,
    pub(super) ni_category: crate::types::NiCategory,
    pub(super) starter_information: crate::types::StarterInformation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Employment {
    pub(super) id: String,
    pub(super) employee_id: String,
    pub(super) paye_scheme_id: String,
    pub(super) pay_schedule_id: String,
    #[serde(flatten)]
    pub(super) payroll_settings: super::EmploymentPayrollSettings,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
