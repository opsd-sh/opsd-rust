use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::ParseError;

const MAX_EMAIL_BYTES: usize = 254;
const MAX_EMAIL_LOCAL_PART_BYTES: usize = 64;
const MAX_EMAIL_DOMAIN_BYTES: usize = 253;
const MAX_EMAIL_DOMAIN_LABEL_BYTES: usize = 63;

/// Canonical email address used to identify an Opsd user.
///
/// Opsd accepts a deliberately narrower shape than the full RFC grammar and
/// lowercases the complete address because email addresses are account
/// identifiers within the product.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn parse(email: &str) -> Result<Self, ParseError> {
        let email = email.trim().to_lowercase();

        if email.is_empty() {
            return Err(ParseError::new("email must not be empty"));
        }
        if email.len() > MAX_EMAIL_BYTES {
            return Err(ParseError::new("email must be at most 254 bytes"));
        }
        if email
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(ParseError::new(
                "email must not contain whitespace or control characters",
            ));
        }

        let Some((local_part, domain)) = email.split_once('@') else {
            return Err(ParseError::new("email must contain a single @ separator"));
        };
        if domain.contains('@') {
            return Err(ParseError::new("email must contain a single @ separator"));
        }
        if local_part.is_empty() {
            return Err(ParseError::new("email local part must not be empty"));
        }
        if local_part.len() > MAX_EMAIL_LOCAL_PART_BYTES {
            return Err(ParseError::new("email local part must be at most 64 bytes"));
        }
        if domain.is_empty() {
            return Err(ParseError::new("email domain must not be empty"));
        }
        if domain.len() > MAX_EMAIL_DOMAIN_BYTES {
            return Err(ParseError::new("email domain must be at most 253 bytes"));
        }
        if !domain.contains('.') {
            return Err(ParseError::new("email domain must contain a dot"));
        }
        if domain
            .split('.')
            .any(|label| label.is_empty() || label.len() > MAX_EMAIL_DOMAIN_LABEL_BYTES)
        {
            return Err(ParseError::new(
                "email domain labels must be non-empty and at most 63 bytes",
            ));
        }

        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for EmailAddress {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::EmailAddress;

    #[test]
    fn email_addresses_are_validated_and_canonicalized() {
        let email = EmailAddress::parse(" User@Example.com ").unwrap();

        assert_eq!(email.as_str(), "user@example.com");
        assert!(EmailAddress::parse("user@localhost").is_err());
    }
}
