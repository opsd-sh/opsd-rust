use std::{fmt, str::FromStr};

use time::OffsetDateTime;

use crate::types::{EmployeeId, ParseError};

const EMPLOYEE_NAME_MAX_CHARS: usize = 100;
const EMPLOYEE_FORENAMES_MAX_COUNT: usize = 100;

macro_rules! employee_name {
    ($name:ident, $description:literal, $length_error:literal, $control_error:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: &str) -> Result<Self, ParseError> {
                let value = value.trim();
                let character_count = value.chars().count();
                if character_count == 0 || character_count > EMPLOYEE_NAME_MAX_CHARS {
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

employee_name!(
    EmployeeForename,
    "One of an employee's forenames.",
    "employee forename must contain between 1 and 100 characters",
    "employee forename must not contain control characters"
);
employee_name!(
    EmployeeSurname,
    "An employee's surname.",
    "employee surname must contain between 1 and 100 characters",
    "employee surname must not contain control characters"
);

/// Non-empty, bounded collection of an employee's complete forenames.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmployeeForenames(Vec<EmployeeForename>);

impl EmployeeForenames {
    pub fn parse(values: Vec<String>) -> Result<Self, ParseError> {
        if values.is_empty() || values.len() > EMPLOYEE_FORENAMES_MAX_COUNT {
            return Err(ParseError::new(
                "employee must have between 1 and 100 forenames",
            ));
        }
        Ok(Self(
            values
                .iter()
                .map(|value| EmployeeForename::parse(value))
                .collect::<Result<_, _>>()?,
        ))
    }

    pub fn as_slice(&self) -> &[EmployeeForename] {
        &self.0
    }
}

/// Employee belonging to a business.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Employee {
    pub id: EmployeeId,
    pub forenames: EmployeeForenames,
    pub surname: EmployeeSurname,
    pub created_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::{EmployeeForename, EmployeeForenames, EmployeeSurname};

    #[test]
    fn employee_names_are_trimmed_and_bounded() {
        assert_eq!(EmployeeForename::parse("  Ada  ").unwrap().as_str(), "Ada");
        assert_eq!(
            EmployeeSurname::parse(" Lovelace ").unwrap().as_str(),
            "Lovelace"
        );
        assert!(EmployeeForename::parse("").is_err());
        assert!(EmployeeSurname::parse(&"a".repeat(101)).is_err());
        assert!(EmployeeForename::parse("Ada\nAugusta").is_err());
    }

    #[test]
    fn employees_require_a_bounded_collection_of_forenames() {
        assert!(EmployeeForenames::parse(Vec::new()).is_err());
        assert!(EmployeeForenames::parse(vec!["Ada".to_string(); 100]).is_ok());
        assert!(EmployeeForenames::parse(vec!["Ada".to_string(); 101]).is_err());
    }
}
