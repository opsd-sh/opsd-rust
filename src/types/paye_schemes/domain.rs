use std::{fmt, str::FromStr};

use time::OffsetDateTime;

use crate::types::{ParseError, PayeSchemeId};

macro_rules! paye_text {
    ($name:ident, $description:literal, $max:literal, $length_error:literal, $control_error:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: &str) -> Result<Self, ParseError> {
                let value = value.trim();
                let character_count = value.chars().count();
                if character_count == 0 || character_count > $max {
                    return Err(ParseError::new($length_error));
                }
                if value.chars().any(char::is_control) {
                    return Err(ParseError::new($control_error));
                }
                Ok(Self(value.to_string()))
            }

            pub fn as_str(&self) -> &str {
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

paye_text!(
    PayeSchemeName,
    "User-facing name of a PAYE scheme.",
    120,
    "PAYE scheme name must contain between 1 and 120 characters",
    "PAYE scheme name must not contain control characters"
);
paye_text!(
    EmployerReference,
    "Employer PAYE reference associated with a scheme.",
    64,
    "employer reference must contain between 1 and 64 characters",
    "employer reference must not contain control characters"
);
paye_text!(
    AccountsOfficeReference,
    "Accounts Office reference associated with a scheme.",
    64,
    "Accounts Office reference must contain between 1 and 64 characters",
    "Accounts Office reference must not contain control characters"
);

/// PAYE scheme belonging to a business.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayeScheme {
    pub id: PayeSchemeId,
    pub name: PayeSchemeName,
    pub employer_reference: EmployerReference,
    pub accounts_office_reference: AccountsOfficeReference,
    pub created_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::{AccountsOfficeReference, EmployerReference, PayeSchemeName};

    #[test]
    fn paye_scheme_values_are_trimmed_and_bounded() {
        assert_eq!(
            PayeSchemeName::parse(" Main payroll ").unwrap().as_str(),
            "Main payroll"
        );
        assert_eq!(
            EmployerReference::parse(" 123/AB456 ").unwrap().as_str(),
            "123/AB456"
        );
        assert_eq!(
            AccountsOfficeReference::parse(" 123PA00045678 ")
                .unwrap()
                .as_str(),
            "123PA00045678"
        );
        assert!(PayeSchemeName::parse("").is_err());
        assert!(EmployerReference::parse(&"a".repeat(65)).is_err());
    }
}
