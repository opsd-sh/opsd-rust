use std::{fmt, str::FromStr};

use time::OffsetDateTime;

use crate::types::{BusinessId, BusinessName};
use crate::types::{EmailAddress, ParseError, PracticeId, PracticeInvitationId, UserId};

/// Client business managed by a practice, without direct business memberships.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticeBusiness {
    pub id: BusinessId,
    pub name: BusinessName,
    pub practice_id: PracticeId,
}

const PRACTICE_NAME_MAX_CHARS: usize = 120;

/// Validated name of an Opsd practice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticeName(String);

impl PracticeName {
    pub fn parse(name: &str) -> Result<Self, ParseError> {
        let name = name.trim();
        let character_count = name.chars().count();
        if character_count == 0 || character_count > PRACTICE_NAME_MAX_CHARS {
            return Err(ParseError::new(
                "practice name must contain between 1 and 120 characters",
            ));
        }
        if name.chars().any(char::is_control) {
            return Err(ParseError::new(
                "practice name must not contain control characters",
            ));
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for PracticeName {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PracticeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Access granted to a user within a practice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PracticeRole {
    Admin,
    Member,
}

impl PracticeRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Member => "member",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "admin" => Ok(Self::Admin),
            "member" => Ok(Self::Member),
            _ => Err(ParseError::new("invalid practice role")),
        }
    }
}

impl FromStr for PracticeRole {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PracticeRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An Opsd practice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Practice {
    pub id: PracticeId,
    pub name: PracticeName,
}

/// Practice together with a user's access role.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticeMembership {
    pub practice: Practice,
    pub role: PracticeRole,
}

/// User with access to a practice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticeMember {
    pub id: UserId,
    pub email: EmailAddress,
    pub email_verified: bool,
    pub role: PracticeRole,
}

/// Invitation created for an email address to join a practice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticeInvitation {
    pub id: PracticeInvitationId,
    pub email: EmailAddress,
    pub role: PracticeRole,
    pub expires_at: OffsetDateTime,
}

/// Pending invitation available to an authenticated user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingPracticeInvitation {
    pub id: PracticeInvitationId,
    pub practice: Practice,
    pub invited_by_email: EmailAddress,
    pub role: PracticeRole,
    pub expires_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::{PracticeName, PracticeRole};

    #[test]
    fn practice_names_are_trimmed_and_bounded() {
        assert_eq!(
            PracticeName::parse("  Acme Ltd\n").unwrap().as_str(),
            "Acme Ltd"
        );
        assert!(PracticeName::parse("").is_err());
        assert!(PracticeName::parse(&"a".repeat(121)).is_err());
        assert!(PracticeName::parse("Acme\tLtd").is_err());
    }

    #[test]
    fn practice_roles_parse_from_the_api_values() {
        assert_eq!(PracticeRole::parse("admin").unwrap(), PracticeRole::Admin);
        assert_eq!(PracticeRole::parse("member").unwrap(), PracticeRole::Member);
        assert!(PracticeRole::parse("viewer").is_err());
    }
}
