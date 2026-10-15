use time::OffsetDateTime;

use crate::types::{EmployeeId, EmploymentId, PayeSchemeId};

/// Payroll relationship between an employee and a PAYE scheme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Employment {
    pub id: EmploymentId,
    pub employee_id: EmployeeId,
    pub paye_scheme_id: PayeSchemeId,
    pub pay_schedule_id: crate::types::PayScheduleId,
    pub payroll_settings: super::EmploymentPayrollSettings,
    pub created_at: OffsetDateTime,
}
