use time::OffsetDateTime;

use crate::types::{EmployeeId, EmploymentId, PayeSchemeId};

/// Payroll relationship between an employee and a PAYE scheme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Employment {
    pub id: EmploymentId,
    pub employee_id: EmployeeId,
    pub paye_scheme_id: PayeSchemeId,
    pub created_at: OffsetDateTime,
}
