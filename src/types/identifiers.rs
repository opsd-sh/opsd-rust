use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::ParseError;

macro_rules! uuid_identifier {
    ($(#[$attribute:meta])* $name:ident, $error:literal) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub const fn new(value: Uuid) -> Self {
                Self(value)
            }

            pub fn parse(value: &str) -> Result<Self, ParseError> {
                Uuid::parse_str(value)
                    .map(Self)
                    .map_err(|_| ParseError::new($error))
            }

            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = ParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

uuid_identifier!(
    /// Public identifier of an Opsd business.
    BusinessId,
    "business ID must be a UUID"
);
uuid_identifier!(
    /// Public identifier of a business invitation.
    BusinessInvitationId,
    "business invitation ID must be a UUID"
);
uuid_identifier!(
    /// Public identifier of an employee.
    EmployeeId,
    "employee ID must be a UUID"
);
uuid_identifier!(
    /// Public identifier of an employment.
    EmploymentId,
    "employment ID must be a UUID"
);
uuid_identifier!(
    /// Public identifier of a PAYE scheme.
    PayeSchemeId,
    "PAYE scheme ID must be a UUID"
);
uuid_identifier!(
    /// Public identifier of a payroll run.
    PayrollRunId,
    "payroll run ID must be a UUID"
);
uuid_identifier!(
    /// Public identifier of an Opsd user.
    UserId,
    "user ID must be a UUID"
);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::BusinessId;

    #[test]
    fn identifiers_parse_and_serialize_as_uuids() {
        let id = BusinessId::from_str("22222222-2222-4222-8222-222222222222")
            .expect("valid business ID should parse");

        assert_eq!(id.to_string(), "22222222-2222-4222-8222-222222222222");
        assert!(BusinessId::from_str("not-a-uuid").is_err());
    }
}
