use std::{fmt, str::FromStr};

use time::OffsetDateTime;

use crate::types::{BusinessId, BusinessInvitationId, EmailAddress, ParseError, UserId};

const BUSINESS_NAME_MAX_CHARS: usize = 120;

/// Validated name of an Opsd business.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessName(String);

impl BusinessName {
    pub fn parse(name: &str) -> Result<Self, ParseError> {
        let name = name.trim();
        let character_count = name.chars().count();
        if character_count == 0 || character_count > BUSINESS_NAME_MAX_CHARS {
            return Err(ParseError::new(
                "business name must contain between 1 and 120 characters",
            ));
        }
        if name.chars().any(char::is_control) {
            return Err(ParseError::new(
                "business name must not contain control characters",
            ));
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for BusinessName {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for BusinessName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Access granted to a user within a business.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BusinessRole {
    Admin,
    PayrollOperator,
}

impl BusinessRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::PayrollOperator => "payroll_operator",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "admin" => Ok(Self::Admin),
            "payroll_operator" => Ok(Self::PayrollOperator),
            _ => Err(ParseError::new("invalid business role")),
        }
    }
}

impl FromStr for BusinessRole {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for BusinessRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An Opsd business.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Business {
    pub id: BusinessId,
    pub name: BusinessName,
}

/// Business together with a user's access role.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessMembership {
    pub business: Business,
    pub role: BusinessRole,
}

/// User with access to a business.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessMember {
    pub id: UserId,
    pub email: EmailAddress,
    pub email_verified: bool,
    pub role: BusinessRole,
}

/// Invitation created for an email address to join a business.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessInvitation {
    pub id: BusinessInvitationId,
    pub email: EmailAddress,
    pub role: BusinessRole,
    pub expires_at: OffsetDateTime,
}

/// Pending invitation available to an authenticated user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingBusinessInvitation {
    pub id: BusinessInvitationId,
    pub business: Business,
    pub invited_by_email: EmailAddress,
    pub role: BusinessRole,
    pub expires_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::{BusinessName, BusinessRole};

    #[test]
    fn business_names_are_trimmed_and_bounded() {
        assert_eq!(
            BusinessName::parse("  Acme Ltd\n").unwrap().as_str(),
            "Acme Ltd"
        );
        assert!(BusinessName::parse("").is_err());
        assert!(BusinessName::parse(&"a".repeat(121)).is_err());
        assert!(BusinessName::parse("Acme\tLtd").is_err());
    }

    #[test]
    fn business_roles_parse_from_the_api_values() {
        assert_eq!(BusinessRole::parse("admin").unwrap(), BusinessRole::Admin);
        assert_eq!(
            BusinessRole::parse("payroll_operator").unwrap(),
            BusinessRole::PayrollOperator
        );
        assert!(BusinessRole::parse("viewer").is_err());
    }
}
