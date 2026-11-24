//! A continuing job and its balances from the previous payroll system.
use crate::types::{
    EmployeeId, EmploymentDate, EmploymentNiMethod, NiCategory, NonNegativePence,
    NormalWeeklyHoursBand, PayScheduleId, PayeSchemeId, PostgraduateLoan, StudentLoan,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportEmploymentRequest {
    pub employee_id: EmployeeId,
    pub paye_scheme_id: PayeSchemeId,
    pub pay_schedule_id: PayScheduleId,
    pub monthly_gross_pay_pence: NonNegativePence,
    pub start_date: EmploymentDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ni_method: Option<EmploymentNiMethod>,
    pub ni_category: NiCategory,
    pub normal_weekly_hours_band: NormalWeeklyHoursBand,
    pub student_loan: StudentLoan,
    pub postgraduate_loan: PostgraduateLoan,
    pub payroll_import: PayrollImport,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PayrollImport {
    /// Exact payroll ID used by the previous software in this PAYE scheme.
    pub payroll_id: String,
    pub tax_year_start: i32,
    /// Payday covered by the latest accepted figures for this employment.
    pub last_payment_date: EmploymentDate,
    pub tax_code: String,
    pub tax_treatment: ImportedTaxTreatment,
    /// This employment's year-to-date amounts, in pence.
    pub gross_pay_pence: i64,
    pub taxable_pay_pence: i64,
    pub income_tax_pence: i64,
    pub net_pay_pence: i64,
    /// Figures for the single NI category supplied on the employment.
    pub ni: ImportedNi,
    /// Required when the old software paid this director under AL this year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director_history: Option<DirectorHistory>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "basis", rename_all = "snake_case", deny_unknown_fields)]
pub enum ImportedTaxTreatment {
    Cumulative {
        previous_employment_taxable_pay_pence: i64,
        previous_employment_tax_paid_pence: i64,
    },
    NonCumulative {},
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedNi {
    pub gross_earnings_pence: i64,
    pub employee_ni_pence: i64,
    pub employer_ni_pence: i64,
    pub at_lel_pence: i64,
    pub above_lel_to_pt_pence: i64,
    pub above_pt_to_uel_pence: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectorHistory {
    pub appointment_week_reported: bool,
    pub annual_assessment_applied: bool,
    /// This year's NI figures from paydays before appointment.
    pub before_appointment: ImportedNi,
}

/// External payroll balances for one employment.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayrollImportOpening {
    pub tax_year_start: i32,
    pub payroll_id: String,
    pub last_payment_date: EmploymentDate,
    pub gross_pay_pence: i64,
    pub taxable_pay_pence: i64,
    pub income_tax_pence: i64,
    pub net_pay_pence: i64,
    pub ni: ImportedNi,
    pub director_history: Option<DirectorHistory>,
}

/// Supply the fields to correct. An explicit null clears director history.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PayrollImportAmendment {
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub payroll_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_payment_date: Option<EmploymentDate>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_pay_pence: Option<i64>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxable_pay_pence: Option<i64>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub income_tax_pence: Option<i64>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_pay_pence: Option<i64>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub ni: Option<ImportedNi>,
    #[serde(
        default,
        deserialize_with = "supplied",
        skip_serializing_if = "Option::is_none"
    )]
    pub director_history: Option<Option<DirectorHistory>>,
}

fn supplied<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}
