use super::{domain::*, wire};
use crate::types::{ParseError, PayFrequency, PayScheduleId, PayeSchemeId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreatePayScheduleRequest",
    into = "wire::CreatePayScheduleRequest"
)]
pub struct CreatePayScheduleRequest {
    pub paye_scheme_id: PayeSchemeId,
    pub name: PayScheduleName,
    pub frequency: PayFrequency,
}
impl TryFrom<wire::CreatePayScheduleRequest> for CreatePayScheduleRequest {
    type Error = ParseError;
    fn try_from(v: wire::CreatePayScheduleRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            paye_scheme_id: PayeSchemeId::parse(&v.paye_scheme_id)?,
            name: PayScheduleName::parse(&v.name)?,
            frequency: v.frequency,
        })
    }
}
impl From<CreatePayScheduleRequest> for wire::CreatePayScheduleRequest {
    fn from(v: CreatePayScheduleRequest) -> Self {
        Self {
            paye_scheme_id: v.paye_scheme_id.to_string(),
            name: v.name.to_string(),
            frequency: v.frequency,
        }
    }
}
fn from_wire(v: wire::PaySchedule) -> Result<PaySchedule, ParseError> {
    Ok(PaySchedule {
        id: PayScheduleId::parse(&v.id)?,
        paye_scheme_id: PayeSchemeId::parse(&v.paye_scheme_id)?,
        name: PayScheduleName::parse(&v.name)?,
        frequency: v.frequency,
        created_at: v.created_at,
    })
}
fn into_wire(v: PaySchedule) -> wire::PaySchedule {
    wire::PaySchedule {
        id: v.id.to_string(),
        paye_scheme_id: v.paye_scheme_id.to_string(),
        name: v.name.to_string(),
        frequency: v.frequency,
        created_at: v.created_at,
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::PaySchedule", into = "wire::PaySchedule")]
pub struct CreatePayScheduleResponse {
    pub schedule: PaySchedule,
}
impl TryFrom<wire::PaySchedule> for CreatePayScheduleResponse {
    type Error = ParseError;
    fn try_from(v: wire::PaySchedule) -> Result<Self, Self::Error> {
        Ok(Self {
            schedule: from_wire(v)?,
        })
    }
}
impl From<CreatePayScheduleResponse> for wire::PaySchedule {
    fn from(v: CreatePayScheduleResponse) -> Self {
        into_wire(v.schedule)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::PaySchedule>", into = "Vec<wire::PaySchedule>")]
pub struct ListPaySchedulesResponse {
    pub schedules: Vec<PaySchedule>,
}
impl TryFrom<Vec<wire::PaySchedule>> for ListPaySchedulesResponse {
    type Error = ParseError;
    fn try_from(v: Vec<wire::PaySchedule>) -> Result<Self, Self::Error> {
        Ok(Self {
            schedules: v.into_iter().map(from_wire).collect::<Result<_, _>>()?,
        })
    }
}
impl From<ListPaySchedulesResponse> for Vec<wire::PaySchedule> {
    fn from(v: ListPaySchedulesResponse) -> Self {
        v.schedules.into_iter().map(into_wire).collect()
    }
}
