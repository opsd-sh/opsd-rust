use serde::{Deserialize, Serialize};

use super::{domain::Employment, wire};
use crate::types::{EmployeeId, EmploymentId, ParseError, PayeSchemeId};

/// Request accepted when creating an employment.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreateEmploymentRequest",
    into = "wire::CreateEmploymentRequest"
)]
pub struct CreateEmploymentRequest {
    pub employee_id: EmployeeId,
    pub paye_scheme_id: PayeSchemeId,
}

/// Response returned after creating an employment.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Employment", into = "wire::Employment")]
pub struct CreateEmploymentResponse {
    pub employment: Employment,
}

/// Response returned when listing a business's employments.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::Employment>", into = "Vec<wire::Employment>")]
pub struct ListEmploymentsResponse {
    pub employments: Vec<Employment>,
}

impl From<CreateEmploymentRequest> for wire::CreateEmploymentRequest {
    fn from(value: CreateEmploymentRequest) -> Self {
        Self {
            employee_id: value.employee_id.to_string(),
            paye_scheme_id: value.paye_scheme_id.to_string(),
        }
    }
}

impl TryFrom<wire::CreateEmploymentRequest> for CreateEmploymentRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreateEmploymentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            employee_id: EmployeeId::parse(&value.employee_id)?,
            paye_scheme_id: PayeSchemeId::parse(&value.paye_scheme_id)?,
        })
    }
}

fn employment_from_wire(value: wire::Employment) -> Result<Employment, ParseError> {
    Ok(Employment {
        id: EmploymentId::parse(&value.id)?,
        employee_id: EmployeeId::parse(&value.employee_id)?,
        paye_scheme_id: PayeSchemeId::parse(&value.paye_scheme_id)?,
        created_at: value.created_at,
    })
}

fn employment_into_wire(value: Employment) -> wire::Employment {
    wire::Employment {
        id: value.id.to_string(),
        employee_id: value.employee_id.to_string(),
        paye_scheme_id: value.paye_scheme_id.to_string(),
        created_at: value.created_at,
    }
}

impl TryFrom<wire::Employment> for CreateEmploymentResponse {
    type Error = ParseError;

    fn try_from(value: wire::Employment) -> Result<Self, Self::Error> {
        Ok(Self {
            employment: employment_from_wire(value)?,
        })
    }
}

impl From<CreateEmploymentResponse> for wire::Employment {
    fn from(value: CreateEmploymentResponse) -> Self {
        employment_into_wire(value.employment)
    }
}

impl TryFrom<Vec<wire::Employment>> for ListEmploymentsResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::Employment>) -> Result<Self, Self::Error> {
        Ok(Self {
            employments: values
                .into_iter()
                .map(employment_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListEmploymentsResponse> for Vec<wire::Employment> {
    fn from(value: ListEmploymentsResponse) -> Self {
        value
            .employments
            .into_iter()
            .map(employment_into_wire)
            .collect()
    }
}
