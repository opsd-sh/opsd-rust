use serde::{Deserialize, Serialize};

use super::{domain::*, wire};
use crate::types::{EmploymentId, ParseError, PayeSchemeId, PayrollRunId, UserId};

/// Request accepted when creating a payroll run.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreatePayrollRunRequest",
    into = "wire::CreatePayrollRunRequest"
)]
pub struct CreatePayrollRunRequest {
    pub paye_scheme_id: PayeSchemeId,
    pub payment_date: PayrollPaymentDate,
}

macro_rules! run_response {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(try_from = "wire::PayrollRun", into = "wire::PayrollRun")]
        pub struct $name {
            pub run: PayrollRun,
        }

        impl TryFrom<wire::PayrollRun> for $name {
            type Error = ParseError;

            fn try_from(value: wire::PayrollRun) -> Result<Self, Self::Error> {
                Ok(Self {
                    run: run_from_wire(value)?,
                })
            }
        }

        impl From<$name> for wire::PayrollRun {
            fn from(value: $name) -> Self {
                run_into_wire(value.run)
            }
        }
    };
}

run_response!(
    CreatePayrollRunResponse,
    "Response returned after creating a payroll run."
);

/// Response returned when listing a business's payroll runs.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::PayrollRun>", into = "Vec<wire::PayrollRun>")]
pub struct ListPayrollRunsResponse {
    pub runs: Vec<PayrollRun>,
}

macro_rules! details_response {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(try_from = "wire::PayrollRunDetails", into = "wire::PayrollRunDetails")]
        pub struct $name {
            pub details: PayrollRunDetails,
        }

        impl TryFrom<wire::PayrollRunDetails> for $name {
            type Error = ParseError;

            fn try_from(value: wire::PayrollRunDetails) -> Result<Self, Self::Error> {
                Ok(Self {
                    details: details_from_wire(value)?,
                })
            }
        }

        impl From<$name> for wire::PayrollRunDetails {
            fn from(value: $name) -> Self {
                details_into_wire(value.details)
            }
        }
    };
}

details_response!(
    GetPayrollRunResponse,
    "Response returned when getting a payroll run."
);
details_response!(
    FinalizePayrollRunResponse,
    "Response returned after finalizing a payroll run."
);

impl TryFrom<wire::CreatePayrollRunRequest> for CreatePayrollRunRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreatePayrollRunRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            paye_scheme_id: PayeSchemeId::parse(&value.paye_scheme_id)?,
            payment_date: PayrollPaymentDate::parse(&value.payment_date)?,
        })
    }
}

impl From<CreatePayrollRunRequest> for wire::CreatePayrollRunRequest {
    fn from(value: CreatePayrollRunRequest) -> Self {
        Self {
            paye_scheme_id: value.paye_scheme_id.to_string(),
            payment_date: value.payment_date.to_string(),
        }
    }
}

fn run_from_wire(value: wire::PayrollRun) -> Result<PayrollRun, ParseError> {
    Ok(PayrollRun {
        id: PayrollRunId::parse(&value.id)?,
        paye_scheme_id: PayeSchemeId::parse(&value.paye_scheme_id)?,
        payment_date: PayrollPaymentDate::parse(&value.payment_date)?,
        status: PayrollRunStatus::parse(&value.status)?,
        employment_count: value.employment_count,
        finalized_at: value.finalized_at,
        finalized_by_user_id: value
            .finalized_by_user_id
            .map(|id| UserId::parse(&id))
            .transpose()?,
        created_at: value.created_at,
    })
}

fn run_into_wire(value: PayrollRun) -> wire::PayrollRun {
    wire::PayrollRun {
        id: value.id.to_string(),
        paye_scheme_id: value.paye_scheme_id.to_string(),
        payment_date: value.payment_date.to_string(),
        status: value.status.to_string(),
        employment_count: value.employment_count,
        finalized_at: value.finalized_at,
        finalized_by_user_id: value.finalized_by_user_id.map(|id| id.to_string()),
        created_at: value.created_at,
    }
}

impl TryFrom<Vec<wire::PayrollRun>> for ListPayrollRunsResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::PayrollRun>) -> Result<Self, Self::Error> {
        Ok(Self {
            runs: values
                .into_iter()
                .map(run_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListPayrollRunsResponse> for Vec<wire::PayrollRun> {
    fn from(value: ListPayrollRunsResponse) -> Self {
        value.runs.into_iter().map(run_into_wire).collect()
    }
}

fn details_from_wire(value: wire::PayrollRunDetails) -> Result<PayrollRunDetails, ParseError> {
    Ok(PayrollRunDetails {
        run: run_from_wire(value.run)?,
        employment_ids: value
            .employment_ids
            .into_iter()
            .map(|id| EmploymentId::parse(&id))
            .collect::<Result<_, _>>()?,
    })
}

fn details_into_wire(value: PayrollRunDetails) -> wire::PayrollRunDetails {
    wire::PayrollRunDetails {
        run: run_into_wire(value.run),
        employment_ids: value
            .employment_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect(),
    }
}
