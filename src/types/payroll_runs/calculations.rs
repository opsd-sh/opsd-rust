use crate::types::{
    EmploymentId, NiCategory, NonNegativePence, PayrollPaymentDate, PayrollRunId, TaxBasis,
};
use serde::{Deserialize, Serialize};
/// Saved gross input. Null represents an incomplete legacy draft.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollPayment {
    pub normal_weekly_hours_band: crate::types::NormalWeeklyHoursBand,
    pub employment_id: EmploymentId,
    pub gross_pay_pence: Option<NonNegativePence>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum UpdatePayrollPaymentRequest {
    GrossPay {
        gross_pay_pence: NonNegativePence,
        #[serde(skip_serializing_if = "Option::is_none")]
        normal_weekly_hours_band: Option<crate::types::NormalWeeklyHoursBand>,
    },
    HoursBand {
        normal_weekly_hours_band: crate::types::NormalWeeklyHoursBand,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentAmounts {
    pub gross_pay_pence: i64,
    pub income_tax_pence: i64,
    pub employee_ni_pence: i64,
    pub employer_ni_pence: i64,
    pub net_pay_pence: i64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculationInputs {
    pub tax_code: String,
    pub tax_basis: TaxBasis,
    pub ni_category: NiCategory,
    pub previous_employment_taxable_pay_pence: i64,
    pub previous_employment_tax_paid_pence: i64,
    pub previous_ytd: PaymentAmounts,
    pub previous_ni_earnings_ytd: NiEarningsBands,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculatedPayment {
    pub employment_id: EmploymentId,
    #[serde(flatten)]
    pub amounts: PaymentAmounts,
    pub inputs: CalculationInputs,
    pub ytd: PaymentAmounts,
    pub ni_earnings: NiEarningsBands,
    pub ni_earnings_ytd: NiEarningsBands,
}
/// Live draft preview, or immutable calculation embedded in a finalised run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollPreviewResponse {
    pub payroll_run_id: PayrollRunId,
    pub payment_date: PayrollPaymentDate,
    pub tax_year_start: i32,
    pub tax_month: u8,
    pub payments: Vec<CalculatedPayment>,
    pub totals: PaymentAmounts,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NiEarningsBands {
    pub at_lel_pence: i64,
    pub above_lel_to_pt_pence: i64,
    pub above_pt_to_uel_pence: i64,
}
