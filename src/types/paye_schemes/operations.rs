use serde::{Deserialize, Serialize};

use super::{domain::*, wire};
use crate::types::{ParseError, PayeSchemeId};

/// Request accepted when creating a PAYE scheme.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreatePayeSchemeRequest",
    into = "wire::CreatePayeSchemeRequest"
)]
pub struct CreatePayeSchemeRequest {
    pub name: PayeSchemeName,
    pub employer_reference: EmployerReference,
    pub accounts_office_reference: AccountsOfficeReference,
}

/// Response returned after creating a PAYE scheme.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::PayeScheme", into = "wire::PayeScheme")]
pub struct CreatePayeSchemeResponse {
    pub scheme: PayeScheme,
}

/// Response returned when listing a business's PAYE schemes.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::PayeScheme>", into = "Vec<wire::PayeScheme>")]
pub struct ListPayeSchemesResponse {
    pub schemes: Vec<PayeScheme>,
}

impl TryFrom<wire::CreatePayeSchemeRequest> for CreatePayeSchemeRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreatePayeSchemeRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: PayeSchemeName::parse(&value.name)?,
            employer_reference: EmployerReference::parse(&value.employer_reference)?,
            accounts_office_reference: AccountsOfficeReference::parse(
                &value.accounts_office_reference,
            )?,
        })
    }
}

impl From<CreatePayeSchemeRequest> for wire::CreatePayeSchemeRequest {
    fn from(value: CreatePayeSchemeRequest) -> Self {
        Self {
            name: value.name.to_string(),
            employer_reference: value.employer_reference.to_string(),
            accounts_office_reference: value.accounts_office_reference.to_string(),
        }
    }
}

fn scheme_from_wire(value: wire::PayeScheme) -> Result<PayeScheme, ParseError> {
    Ok(PayeScheme {
        id: PayeSchemeId::parse(&value.id)?,
        name: PayeSchemeName::parse(&value.name)?,
        employer_reference: EmployerReference::parse(&value.employer_reference)?,
        accounts_office_reference: AccountsOfficeReference::parse(
            &value.accounts_office_reference,
        )?,
        created_at: value.created_at,
    })
}

fn scheme_into_wire(value: PayeScheme) -> wire::PayeScheme {
    wire::PayeScheme {
        id: value.id.to_string(),
        name: value.name.to_string(),
        employer_reference: value.employer_reference.to_string(),
        accounts_office_reference: value.accounts_office_reference.to_string(),
        created_at: value.created_at,
    }
}

impl TryFrom<wire::PayeScheme> for CreatePayeSchemeResponse {
    type Error = ParseError;

    fn try_from(value: wire::PayeScheme) -> Result<Self, Self::Error> {
        Ok(Self {
            scheme: scheme_from_wire(value)?,
        })
    }
}

impl From<CreatePayeSchemeResponse> for wire::PayeScheme {
    fn from(value: CreatePayeSchemeResponse) -> Self {
        scheme_into_wire(value.scheme)
    }
}

impl TryFrom<Vec<wire::PayeScheme>> for ListPayeSchemesResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::PayeScheme>) -> Result<Self, Self::Error> {
        Ok(Self {
            schemes: values
                .into_iter()
                .map(scheme_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListPayeSchemesResponse> for Vec<wire::PayeScheme> {
    fn from(value: ListPayeSchemesResponse) -> Self {
        value.schemes.into_iter().map(scheme_into_wire).collect()
    }
}
