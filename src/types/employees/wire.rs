use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct CreateEmployeeRequest {
    pub(super) title: Option<String>,
    pub(super) passport_number: Option<String>,
    pub(super) date_of_birth: crate::types::DateOfBirth,
    pub(super) sex: crate::types::EmployeeSex,
    pub(super) address: crate::types::EmployeeAddress,
    pub(super) national_insurance_number: Option<String>,

    pub(super) forenames: Vec<String>,
    pub(super) surname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct UpdateEmployeeRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::types::FieldUpdate::is_unchanged"
    )]
    pub(super) title: crate::types::FieldUpdate<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::types::FieldUpdate::is_unchanged"
    )]
    pub(super) passport_number: crate::types::FieldUpdate<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) date_of_birth: Option<crate::types::DateOfBirth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) sex: Option<crate::types::EmployeeSex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) address: Option<crate::types::EmployeeAddress>,
    #[serde(
        default,
        skip_serializing_if = "crate::types::FieldUpdate::is_unchanged"
    )]
    pub(super) national_insurance_number: crate::types::FieldUpdate<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) forenames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) surname: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Employee {
    pub(super) title: Option<String>,
    pub(super) passport_number: Option<String>,
    pub(super) date_of_birth: crate::types::DateOfBirth,
    pub(super) sex: crate::types::EmployeeSex,
    pub(super) address: crate::types::EmployeeAddress,
    pub(super) national_insurance_number: Option<String>,

    pub(super) id: String,
    pub(super) forenames: Vec<String>,
    pub(super) surname: String,
    #[serde(with = "time::serde::rfc3339")]
    pub(super) created_at: OffsetDateTime,
}
