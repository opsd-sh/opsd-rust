use serde::{Deserialize, Serialize};

use super::{domain::*, wire};
use crate::types::{EmployeeId, ParseError};

/// Request accepted when creating an employee.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreateEmployeeRequest",
    into = "wire::CreateEmployeeRequest"
)]
pub struct CreateEmployeeRequest {
    pub title: Option<String>,
    pub passport_number: Option<String>,
    pub date_of_birth: crate::types::DateOfBirth,
    pub sex: crate::types::EmployeeSex,
    pub address: crate::types::EmployeeAddress,
    pub national_insurance_number: Option<String>,

    pub forenames: EmployeeForenames,
    pub surname: EmployeeSurname,
}

/// Change employee details; the server enforces edits allowed by payroll history.
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::UpdateEmployeeRequest",
    into = "wire::UpdateEmployeeRequest"
)]
pub struct UpdateEmployeeRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::types::FieldUpdate::is_unchanged"
    )]
    pub title: crate::types::FieldUpdate<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::types::FieldUpdate::is_unchanged"
    )]
    pub passport_number: crate::types::FieldUpdate<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<crate::types::DateOfBirth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<crate::types::EmployeeSex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::types::EmployeeAddress>,
    #[serde(
        default,
        skip_serializing_if = "crate::types::FieldUpdate::is_unchanged"
    )]
    pub national_insurance_number: crate::types::FieldUpdate<String>,

    pub forenames: Option<EmployeeForenames>,
    pub surname: Option<EmployeeSurname>,
}

macro_rules! employee_response {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(try_from = "wire::Employee", into = "wire::Employee")]
        pub struct $name {
            pub employee: Employee,
        }

        impl TryFrom<wire::Employee> for $name {
            type Error = ParseError;

            fn try_from(value: wire::Employee) -> Result<Self, Self::Error> {
                Ok(Self {
                    employee: employee_from_wire(value)?,
                })
            }
        }

        impl From<$name> for wire::Employee {
            fn from(value: $name) -> Self {
                employee_into_wire(value.employee)
            }
        }
    };
}

employee_response!(
    CreateEmployeeResponse,
    "Response returned after creating an employee."
);
employee_response!(
    GetEmployeeResponse,
    "Response returned when getting an employee."
);
employee_response!(
    UpdateEmployeeResponse,
    "Response returned after changing an employee."
);

/// Response returned when listing a business's employees.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::Employee>", into = "Vec<wire::Employee>")]
pub struct ListEmployeesResponse {
    pub employees: Vec<Employee>,
}

impl TryFrom<wire::CreateEmployeeRequest> for CreateEmployeeRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreateEmployeeRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            passport_number: value.passport_number,
            date_of_birth: value.date_of_birth,
            sex: value.sex,
            address: value.address,
            national_insurance_number: value.national_insurance_number,

            forenames: EmployeeForenames::parse(value.forenames)?,
            surname: EmployeeSurname::parse(&value.surname)?,
        })
    }
}

impl From<CreateEmployeeRequest> for wire::CreateEmployeeRequest {
    fn from(value: CreateEmployeeRequest) -> Self {
        Self {
            title: value.title,
            passport_number: value.passport_number,
            date_of_birth: value.date_of_birth,
            sex: value.sex,
            address: value.address,
            national_insurance_number: value.national_insurance_number,

            forenames: value
                .forenames
                .as_slice()
                .iter()
                .map(ToString::to_string)
                .collect(),
            surname: value.surname.to_string(),
        }
    }
}

impl TryFrom<wire::UpdateEmployeeRequest> for UpdateEmployeeRequest {
    type Error = ParseError;

    fn try_from(value: wire::UpdateEmployeeRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            passport_number: value.passport_number,
            date_of_birth: value.date_of_birth,
            sex: value.sex,
            address: value.address,
            national_insurance_number: value.national_insurance_number,

            forenames: value.forenames.map(EmployeeForenames::parse).transpose()?,
            surname: value
                .surname
                .map(|surname| EmployeeSurname::parse(&surname))
                .transpose()?,
        })
    }
}

impl From<UpdateEmployeeRequest> for wire::UpdateEmployeeRequest {
    fn from(value: UpdateEmployeeRequest) -> Self {
        Self {
            title: value.title,
            passport_number: value.passport_number,
            date_of_birth: value.date_of_birth,
            sex: value.sex,
            address: value.address,
            national_insurance_number: value.national_insurance_number,

            forenames: value.forenames.map(|forenames| {
                forenames
                    .as_slice()
                    .iter()
                    .map(ToString::to_string)
                    .collect()
            }),
            surname: value.surname.map(|surname| surname.to_string()),
        }
    }
}

fn employee_from_wire(value: wire::Employee) -> Result<Employee, ParseError> {
    Ok(Employee {
        title: value.title,
        passport_number: value.passport_number,
        date_of_birth: value.date_of_birth,
        sex: value.sex,
        address: value.address,
        national_insurance_number: value.national_insurance_number,

        id: EmployeeId::parse(&value.id)?,
        forenames: EmployeeForenames::parse(value.forenames)?,
        surname: EmployeeSurname::parse(&value.surname)?,
        created_at: value.created_at,
    })
}

fn employee_into_wire(value: Employee) -> wire::Employee {
    wire::Employee {
        title: value.title,
        passport_number: value.passport_number,
        date_of_birth: value.date_of_birth,
        sex: value.sex,
        address: value.address,
        national_insurance_number: value.national_insurance_number,

        id: value.id.to_string(),
        forenames: value
            .forenames
            .as_slice()
            .iter()
            .map(ToString::to_string)
            .collect(),
        surname: value.surname.to_string(),
        created_at: value.created_at,
    }
}

impl TryFrom<Vec<wire::Employee>> for ListEmployeesResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::Employee>) -> Result<Self, Self::Error> {
        Ok(Self {
            employees: values
                .into_iter()
                .map(employee_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListEmployeesResponse> for Vec<wire::Employee> {
    fn from(value: ListEmployeesResponse) -> Self {
        value
            .employees
            .into_iter()
            .map(employee_into_wire)
            .collect()
    }
}
