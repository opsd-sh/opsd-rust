use serde::{Deserialize, Serialize};

use super::{domain::*, wire};
use crate::types::{BusinessId, BusinessInvitationId, EmailAddress, ParseError, UserId};

/// Request accepted when creating a business.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreateBusinessRequest",
    into = "wire::CreateBusinessRequest"
)]
pub struct CreateBusinessRequest {
    pub name: BusinessName,
}

/// Response returned after creating a business.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Business", into = "wire::Business")]
pub struct CreateBusinessResponse {
    pub membership: BusinessMembership,
}

/// Response returned when listing a user's businesses.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "Vec<wire::Business>", into = "Vec<wire::Business>")]
pub struct ListBusinessesResponse {
    pub memberships: Vec<BusinessMembership>,
}

/// Response returned when getting a business.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Business", into = "wire::Business")]
pub struct GetBusinessResponse {
    pub membership: BusinessMembership,
}

/// Response returned when listing business members.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::BusinessMember>",
    into = "Vec<wire::BusinessMember>"
)]
pub struct ListBusinessMembersResponse {
    pub members: Vec<BusinessMember>,
}

/// Request accepted when changing a business member's role.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::UpdateBusinessMemberRequest",
    into = "wire::UpdateBusinessMemberRequest"
)]
pub struct UpdateBusinessMemberRequest {
    pub role: BusinessRole,
}

/// Request accepted when inviting someone to a business.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::CreateBusinessInvitationRequest",
    into = "wire::CreateBusinessInvitationRequest"
)]
pub struct CreateBusinessInvitationRequest {
    pub email: EmailAddress,
    pub role: BusinessRole,
}

/// State included in responses describing outgoing invitations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BusinessInvitationStatus {
    Pending,
}

/// Invitation and state returned to a business administrator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBusinessInvitation {
    pub invitation: BusinessInvitation,
    pub status: BusinessInvitationStatus,
}

/// Response returned after creating a business invitation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "wire::OutgoingBusinessInvitation",
    into = "wire::OutgoingBusinessInvitation"
)]
pub struct CreateBusinessInvitationResponse {
    pub invitation: OutgoingBusinessInvitation,
}

/// Response returned when listing a business's pending invitations.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::OutgoingBusinessInvitation>",
    into = "Vec<wire::OutgoingBusinessInvitation>"
)]
pub struct ListOutgoingBusinessInvitationsResponse {
    pub invitations: Vec<OutgoingBusinessInvitation>,
}

/// Response returned when listing invitations for the authenticated user.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    try_from = "Vec<wire::PendingBusinessInvitation>",
    into = "Vec<wire::PendingBusinessInvitation>"
)]
pub struct ListPendingBusinessInvitationsResponse {
    pub invitations: Vec<PendingBusinessInvitation>,
}

/// Response returned after accepting a business invitation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "wire::Business", into = "wire::Business")]
pub struct AcceptBusinessInvitationResponse {
    pub membership: BusinessMembership,
}

impl TryFrom<wire::CreateBusinessRequest> for CreateBusinessRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreateBusinessRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: BusinessName::parse(&value.name)?,
        })
    }
}

impl From<CreateBusinessRequest> for wire::CreateBusinessRequest {
    fn from(value: CreateBusinessRequest) -> Self {
        Self {
            name: value.name.to_string(),
        }
    }
}

fn membership_from_wire(value: wire::Business) -> Result<BusinessMembership, ParseError> {
    Ok(BusinessMembership {
        business: Business {
            id: BusinessId::parse(&value.id)?,
            name: BusinessName::parse(&value.name)?,
        },
        role: BusinessRole::parse(&value.role)?,
    })
}

fn membership_into_wire(value: BusinessMembership) -> wire::Business {
    wire::Business {
        id: value.business.id.to_string(),
        name: value.business.name.to_string(),
        role: value.role.to_string(),
    }
}

macro_rules! membership_response {
    ($name:ident) => {
        impl TryFrom<wire::Business> for $name {
            type Error = ParseError;

            fn try_from(value: wire::Business) -> Result<Self, Self::Error> {
                Ok(Self {
                    membership: membership_from_wire(value)?,
                })
            }
        }

        impl From<$name> for wire::Business {
            fn from(value: $name) -> Self {
                membership_into_wire(value.membership)
            }
        }
    };
}

membership_response!(CreateBusinessResponse);
membership_response!(GetBusinessResponse);
membership_response!(AcceptBusinessInvitationResponse);

impl TryFrom<Vec<wire::Business>> for ListBusinessesResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::Business>) -> Result<Self, Self::Error> {
        Ok(Self {
            memberships: values
                .into_iter()
                .map(membership_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListBusinessesResponse> for Vec<wire::Business> {
    fn from(value: ListBusinessesResponse) -> Self {
        value
            .memberships
            .into_iter()
            .map(membership_into_wire)
            .collect()
    }
}

fn member_from_wire(value: wire::BusinessMember) -> Result<BusinessMember, ParseError> {
    Ok(BusinessMember {
        id: UserId::parse(&value.id)?,
        email: EmailAddress::parse(&value.email)?,
        email_verified: value.email_verified,
        role: BusinessRole::parse(&value.role)?,
    })
}

fn member_into_wire(value: BusinessMember) -> wire::BusinessMember {
    wire::BusinessMember {
        id: value.id.to_string(),
        email: value.email.to_string(),
        email_verified: value.email_verified,
        role: value.role.to_string(),
    }
}

impl TryFrom<Vec<wire::BusinessMember>> for ListBusinessMembersResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::BusinessMember>) -> Result<Self, Self::Error> {
        Ok(Self {
            members: values
                .into_iter()
                .map(member_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListBusinessMembersResponse> for Vec<wire::BusinessMember> {
    fn from(value: ListBusinessMembersResponse) -> Self {
        value.members.into_iter().map(member_into_wire).collect()
    }
}

impl TryFrom<wire::UpdateBusinessMemberRequest> for UpdateBusinessMemberRequest {
    type Error = ParseError;

    fn try_from(value: wire::UpdateBusinessMemberRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            role: BusinessRole::parse(&value.role)?,
        })
    }
}

impl From<UpdateBusinessMemberRequest> for wire::UpdateBusinessMemberRequest {
    fn from(value: UpdateBusinessMemberRequest) -> Self {
        Self {
            role: value.role.to_string(),
        }
    }
}

impl TryFrom<wire::CreateBusinessInvitationRequest> for CreateBusinessInvitationRequest {
    type Error = ParseError;

    fn try_from(value: wire::CreateBusinessInvitationRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            email: EmailAddress::parse(&value.email)?,
            role: BusinessRole::parse(&value.role)?,
        })
    }
}

impl From<CreateBusinessInvitationRequest> for wire::CreateBusinessInvitationRequest {
    fn from(value: CreateBusinessInvitationRequest) -> Self {
        Self {
            email: value.email.to_string(),
            role: value.role.to_string(),
        }
    }
}

fn outgoing_from_wire(
    value: wire::OutgoingBusinessInvitation,
) -> Result<OutgoingBusinessInvitation, ParseError> {
    let status = match value.status.as_str() {
        "pending" => BusinessInvitationStatus::Pending,
        _ => return Err(ParseError::new("invalid business invitation status")),
    };
    Ok(OutgoingBusinessInvitation {
        invitation: BusinessInvitation {
            id: BusinessInvitationId::parse(&value.id)?,
            email: EmailAddress::parse(&value.email)?,
            role: BusinessRole::parse(&value.role)?,
            expires_at: value.expires_at,
        },
        status,
    })
}

fn outgoing_into_wire(value: OutgoingBusinessInvitation) -> wire::OutgoingBusinessInvitation {
    wire::OutgoingBusinessInvitation {
        id: value.invitation.id.to_string(),
        email: value.invitation.email.to_string(),
        role: value.invitation.role.to_string(),
        status: "pending".to_string(),
        expires_at: value.invitation.expires_at,
    }
}

impl TryFrom<wire::OutgoingBusinessInvitation> for CreateBusinessInvitationResponse {
    type Error = ParseError;

    fn try_from(value: wire::OutgoingBusinessInvitation) -> Result<Self, Self::Error> {
        Ok(Self {
            invitation: outgoing_from_wire(value)?,
        })
    }
}

impl From<CreateBusinessInvitationResponse> for wire::OutgoingBusinessInvitation {
    fn from(value: CreateBusinessInvitationResponse) -> Self {
        outgoing_into_wire(value.invitation)
    }
}

impl TryFrom<Vec<wire::OutgoingBusinessInvitation>> for ListOutgoingBusinessInvitationsResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::OutgoingBusinessInvitation>) -> Result<Self, Self::Error> {
        Ok(Self {
            invitations: values
                .into_iter()
                .map(outgoing_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListOutgoingBusinessInvitationsResponse> for Vec<wire::OutgoingBusinessInvitation> {
    fn from(value: ListOutgoingBusinessInvitationsResponse) -> Self {
        value
            .invitations
            .into_iter()
            .map(outgoing_into_wire)
            .collect()
    }
}

fn pending_from_wire(
    value: wire::PendingBusinessInvitation,
) -> Result<PendingBusinessInvitation, ParseError> {
    Ok(PendingBusinessInvitation {
        id: BusinessInvitationId::parse(&value.id)?,
        business: Business {
            id: BusinessId::parse(&value.business_id)?,
            name: BusinessName::parse(&value.business_name)?,
        },
        invited_by_email: EmailAddress::parse(&value.invited_by_email)?,
        role: BusinessRole::parse(&value.role)?,
        expires_at: value.expires_at,
    })
}

fn pending_into_wire(value: PendingBusinessInvitation) -> wire::PendingBusinessInvitation {
    wire::PendingBusinessInvitation {
        id: value.id.to_string(),
        business_id: value.business.id.to_string(),
        business_name: value.business.name.to_string(),
        invited_by_email: value.invited_by_email.to_string(),
        role: value.role.to_string(),
        expires_at: value.expires_at,
    }
}

impl TryFrom<Vec<wire::PendingBusinessInvitation>> for ListPendingBusinessInvitationsResponse {
    type Error = ParseError;

    fn try_from(values: Vec<wire::PendingBusinessInvitation>) -> Result<Self, Self::Error> {
        Ok(Self {
            invitations: values
                .into_iter()
                .map(pending_from_wire)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<ListPendingBusinessInvitationsResponse> for Vec<wire::PendingBusinessInvitation> {
    fn from(value: ListPendingBusinessInvitationsResponse) -> Self {
        value
            .invitations
            .into_iter()
            .map(pending_into_wire)
            .collect()
    }
}
