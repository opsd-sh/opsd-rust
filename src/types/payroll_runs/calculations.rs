use crate::types::{
    EmploymentId, NiCategory, NonNegativePence, PayrollPaymentDate, PayrollRunId, TaxBasis,
};
use serde::{Deserialize, Serialize};
/// Saved gross input. Null represents an incomplete legacy draft.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayrollPayment {
    pub employment_id: EmploymentId,
    pub gross_pay_pence: Option<NonNegativePence>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdatePayrollPaymentRequest {
    pub gross_pay_pence: NonNegativePence,
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
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculatedPayment {
    pub employment_id: EmploymentId,
    #[serde(flatten)]
    pub amounts: PaymentAmounts,
    pub inputs: CalculationInputs,
    pub ytd: PaymentAmounts,
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
