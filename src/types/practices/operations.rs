use serde::{Deserialize, Serialize};

use super::{domain::*, wire};
use crate::types::{BusinessId, BusinessName};
use crate::types::{EmailAddress, ParseError, PracticeId, PracticeInvitationId, UserId};

/// Creates a new practice-owned business; does not link an existing business.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreatePracticeRequest",
    into = "wire::CreatePracticeRequest"
)]
pub struct CreatePracticeBusinessRequest {
    pub name: BusinessName,
}

impl TryFrom<wire::CreatePracticeRequest> for CreatePracticeBusinessRequest {
    type Error = ParseError;
    fn try_from(value: wire::CreatePracticeRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: BusinessName::parse(&value.name)?,
        })
    }
}
impl From<CreatePracticeBusinessRequest> for wire::CreatePracticeRequest {
    fn from(value: CreatePracticeBusinessRequest) -> Self {
        Self {
            name: value.name.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::PracticeBusiness", into = "wire::PracticeBusiness")]
pub struct CreatePracticeBusinessResponse {
    pub business: PracticeBusiness,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::PracticeBusiness>",
    into = "Vec<wire::PracticeBusiness>"
)]
pub struct ListPracticeBusinessesResponse {
    pub businesses: Vec<PracticeBusiness>,
}

fn business_from_wire(value: wire::PracticeBusiness) -> Result<PracticeBusiness, ParseError> {
    Ok(PracticeBusiness {
        id: BusinessId::parse(&value.id)?,
        name: BusinessName::parse(&value.name)?,
        practice_id: PracticeId::parse(&value.practice_id)?,
    })
}
fn business_into_wire(value: PracticeBusiness) -> wire::PracticeBusiness {
    wire::PracticeBusiness {
        id: value.id.to_string(),
        name: value.name.to_string(),
        practice_id: value.practice_id.to_string(),
    }
}
impl TryFrom<wire::PracticeBusiness> for CreatePracticeBusinessResponse {
    type Error = ParseError;
    fn try_from(value: wire::PracticeBusiness) -> Result<Self, Self::Error> {
        Ok(Self {
            business: business_from_wire(value)?,
        })
    }
}
impl From<CreatePracticeBusinessResponse> for wire::PracticeBusiness {
    fn from(value: CreatePracticeBusinessResponse) -> Self {
        business_into_wire(value.business)
    }
}
impl TryFrom<Vec<wire::PracticeBusiness>> for ListPracticeBusinessesResponse {
    type Error = ParseError;
    fn try_from(value: Vec<wire::PracticeBusiness>) -> Result<Self, Self::Error> {
        Ok(Self {
            businesses: value
                .into_iter()
                .map(business_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}
impl From<ListPracticeBusinessesResponse> for Vec<wire::PracticeBusiness> {
    fn from(value: ListPracticeBusinessesResponse) -> Self {
        value
            .businesses
            .into_iter()
            .map(business_into_wire)
            .collect()
    }
}

/// Request accepted when creating a practice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreatePracticeRequest",
    into = "wire::CreatePracticeRequest"
)]
pub struct CreatePracticeRequest {
    pub name: PracticeName,
}

/// Response returned after creating a practice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Practice", into = "wire::Practice")]
pub struct CreatePracticeResponse {
    pub membership: PracticeMembership,
}

/// Response returned when listing a user's practices.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::Practice>", into = "Vec<wire::Practice>")]
pub struct ListPracticesResponse {
    pub memberships: Vec<PracticeMembership>,
}

/// Response returned when getting a practice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Practice", into = "wire::Practice")]
pub struct GetPracticeResponse {
    pub membership: PracticeMembership,
}

/// Response returned when listing practice members.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::PracticeMember>",
    into = "Vec<wire::PracticeMember>"
)]
pub struct ListPracticeMembersResponse {
    pub members: Vec<PracticeMember>,
}

/// Request accepted when changing a practice member's role.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::UpdatePracticeMemberRequest",
    into = "wire::UpdatePracticeMemberRequest"
)]
pub struct UpdatePracticeMemberRequest {
    pub role: PracticeRole,
}

/// Request accepted when inviting someone to a practice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreatePracticeInvitationRequest",
    into = "wire::CreatePracticeInvitationRequest"
)]
pub struct CreatePracticeInvitationRequest {
    pub email: EmailAddress,
    pub role: PracticeRole,
}

/// State included in responses describing outgoing invitations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PracticeInvitationStatus {
    Pending,
}

/// Invitation and state returned to a practice administrator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingPracticeInvitation {
    pub invitation: PracticeInvitation,
    pub status: PracticeInvitationStatus,
}

/// Response returned after creating a practice invitation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::OutgoingPracticeInvitation",
    into = "wire::OutgoingPracticeInvitation"
)]
pub struct CreatePracticeInvitationResponse {
    pub invitation: OutgoingPracticeInvitation,
}

/// Response returned when listing a practice's pending invitations.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::OutgoingPracticeInvitation>",
    into = "Vec<wire::OutgoingPracticeInvitation>"
)]
pub struct ListOutgoingPracticeInvitationsResponse {
    pub invitations: Vec<OutgoingPracticeInvitation>,
}

/// Response returned when listing invitations for the authenticated user.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::PendingPracticeInvitation>",
    into = "Vec<wire::PendingPracticeInvitation>"
)]
pub struct ListPendingPracticeInvitationsResponse {
    pub invitations: Vec<PendingPracticeInvitation>,
}

/// Response returned after accepting a practice invitation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Practice", into = "wire::Practice")]
pub struct AcceptPracticeInvitationResponse {
    pub membership: PracticeMembership,
}

impl TryFrom<wire::CreatePracticeRequest> for CreatePracticeRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreatePracticeRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: PracticeName::parse(&value.name)?,
        })
    }
}

impl From<CreatePracticeRequest> for wire::CreatePracticeRequest {
    fn from(value: CreatePracticeRequest) -> Self {
        Self {
            name: value.name.to_string(),
        }
    }
}

fn membership_from_wire(value: wire::Practice) -> Result<PracticeMembership, ParseError> {
    Ok(PracticeMembership {
        practice: Practice {
            id: PracticeId::parse(&value.id)?,
            name: PracticeName::parse(&value.name)?,
        },
        role: PracticeRole::parse(&value.role)?,
    })
}

fn membership_into_wire(value: PracticeMembership) -> wire::Practice {
    wire::Practice {
        id: value.practice.id.to_string(),
        name: value.practice.name.to_string(),
        role: value.role.to_string(),
    }
}

macro_rules! membership_response {
    ($name:ident) => {
        impl TryFrom<wire::Practice> for $name {
            type Error = ParseError;

            fn try_from(value: wire::Practice) -> Result<Self, Self::Error> {
                Ok(Self {
                    membership: membership_from_wire(value)?,
                })
            }
        }

        impl From<$name> for wire::Practice {
            fn from(value: $name) -> Self {
                membership_into_wire(value.membership)
            }
        }
    };
}

membership_response!(CreatePracticeResponse);
membership_response!(GetPracticeResponse);
membership_response!(AcceptPracticeInvitationResponse);

impl TryFrom<Vec<wire::Practice>> for ListPracticesResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::Practice>) -> Result<Self, Self::Error> {
        Ok(Self {
            memberships: values
                .into_iter()
                .map(membership_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListPracticesResponse> for Vec<wire::Practice> {
    fn from(value: ListPracticesResponse) -> Self {
        value
            .memberships
            .into_iter()
            .map(membership_into_wire)
            .collect()
    }
}

fn member_from_wire(value: wire::PracticeMember) -> Result<PracticeMember, ParseError> {
    Ok(PracticeMember {
        id: UserId::parse(&value.id)?,
        email: EmailAddress::parse(&value.email)?,
        email_verified: value.email_verified,
        role: PracticeRole::parse(&value.role)?,
    })
}

fn member_into_wire(value: PracticeMember) -> wire::PracticeMember {
    wire::PracticeMember {
        id: value.id.to_string(),
        email: value.email.to_string(),
        email_verified: value.email_verified,
        role: value.role.to_string(),
    }
}

impl TryFrom<Vec<wire::PracticeMember>> for ListPracticeMembersResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::PracticeMember>) -> Result<Self, Self::Error> {
        Ok(Self {
            members: values
                .into_iter()
                .map(member_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListPracticeMembersResponse> for Vec<wire::PracticeMember> {
    fn from(value: ListPracticeMembersResponse) -> Self {
        value.members.into_iter().map(member_into_wire).collect()
    }
}

impl TryFrom<wire::UpdatePracticeMemberRequest> for UpdatePracticeMemberRequest {
    type Error = ParseError;

    fn try_from(value: wire::UpdatePracticeMemberRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            role: PracticeRole::parse(&value.role)?,
        })
    }
}

impl From<UpdatePracticeMemberRequest> for wire::UpdatePracticeMemberRequest {
    fn from(value: UpdatePracticeMemberRequest) -> Self {
        Self {
            role: value.role.to_string(),
        }
    }
}

impl TryFrom<wire::CreatePracticeInvitationRequest> for CreatePracticeInvitationRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreatePracticeInvitationRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            email: EmailAddress::parse(&value.email)?,
            role: PracticeRole::parse(&value.role)?,
        })
    }
}

impl From<CreatePracticeInvitationRequest> for wire::CreatePracticeInvitationRequest {
    fn from(value: CreatePracticeInvitationRequest) -> Self {
        Self {
            email: value.email.to_string(),
            role: value.role.to_string(),
        }
    }
}

fn outgoing_from_wire(
    value: wire::OutgoingPracticeInvitation,
) -> Result<OutgoingPracticeInvitation, ParseError> {
    let status = match value.status.as_str() {
        "pending" => PracticeInvitationStatus::Pending,
        _ => return Err(ParseError::new("invalid practice invitation status")),
    };
    Ok(OutgoingPracticeInvitation {
        invitation: PracticeInvitation {
            id: PracticeInvitationId::parse(&value.id)?,
            email: EmailAddress::parse(&value.email)?,
            role: PracticeRole::parse(&value.role)?,
            expires_at: value.expires_at,
        },
        status,
    })
}

fn outgoing_into_wire(value: OutgoingPracticeInvitation) -> wire::OutgoingPracticeInvitation {
    wire::OutgoingPracticeInvitation {
        id: value.invitation.id.to_string(),
        email: value.invitation.email.to_string(),
        role: value.invitation.role.to_string(),
        status: "pending".to_string(),
        expires_at: value.invitation.expires_at,
    }
}

impl TryFrom<wire::OutgoingPracticeInvitation> for CreatePracticeInvitationResponse {
    type Error = ParseError;

    fn try_from(value: wire::OutgoingPracticeInvitation) -> Result<Self, Self::Error> {
        Ok(Self {
            invitation: outgoing_from_wire(value)?,
        })
    }
}

impl From<CreatePracticeInvitationResponse> for wire::OutgoingPracticeInvitation {
    fn from(value: CreatePracticeInvitationResponse) -> Self {
        outgoing_into_wire(value.invitation)
    }
}

impl TryFrom<Vec<wire::OutgoingPracticeInvitation>> for ListOutgoingPracticeInvitationsResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::OutgoingPracticeInvitation>) -> Result<Self, Self::Error> {
        Ok(Self {
            invitations: values
                .into_iter()
                .map(outgoing_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListOutgoingPracticeInvitationsResponse> for Vec<wire::OutgoingPracticeInvitation> {
    fn from(value: ListOutgoingPracticeInvitationsResponse) -> Self {
        value
            .invitations
            .into_iter()
            .map(outgoing_into_wire)
            .collect()
    }
}

fn pending_from_wire(
    value: wire::PendingPracticeInvitation,
) -> Result<PendingPracticeInvitation, ParseError> {
    Ok(PendingPracticeInvitation {
        id: PracticeInvitationId::parse(&value.id)?,
        practice: Practice {
            id: PracticeId::parse(&value.practice_id)?,
            name: PracticeName::parse(&value.practice_name)?,
        },
        invited_by_email: EmailAddress::parse(&value.invited_by_email)?,
        role: PracticeRole::parse(&value.role)?,
        expires_at: value.expires_at,
    })
}

fn pending_into_wire(value: PendingPracticeInvitation) -> wire::PendingPracticeInvitation {
    wire::PendingPracticeInvitation {
        id: value.id.to_string(),
        practice_id: value.practice.id.to_string(),
        practice_name: value.practice.name.to_string(),
        invited_by_email: value.invited_by_email.to_string(),
        role: value.role.to_string(),
        expires_at: value.expires_at,
    }
}

impl TryFrom<Vec<wire::PendingPracticeInvitation>> for ListPendingPracticeInvitationsResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::PendingPracticeInvitation>) -> Result<Self, Self::Error> {
        Ok(Self {
            invitations: values
                .into_iter()
                .map(pending_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListPendingPracticeInvitationsResponse> for Vec<wire::PendingPracticeInvitation> {
    fn from(value: ListPendingPracticeInvitationsResponse) -> Self {
        value
            .invitations
            .into_iter()
            .map(pending_into_wire)
            .collect()
    }
}
