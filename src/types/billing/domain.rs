use crate::types::ParseError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use time::Date;

/// UTC service month in the API's YYYY-MM format, not a payroll payment date.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BillingMonth(Date);

impl BillingMonth {
    pub fn parse(value: &str) -> Result<Self, ParseError> {
        let invalid = || ParseError::new("billing month must be YYYY-MM with a valid month");
        if value.len() != 7
            || value.as_bytes()[4] != b'-'
            || !value
                .bytes()
                .enumerate()
                .all(|(i, b)| i == 4 || b.is_ascii_digit())
        {
            return Err(invalid());
        }
        let year = value[..4].parse().map_err(|_| invalid())?;
        let month = value[5..]
            .parse::<u8>()
            .map_err(|_| invalid())?
            .try_into()
            .map_err(|_| invalid())?;
        Date::from_calendar_date(year, month, 1)
            .map(Self)
            .map_err(|_| invalid())
    }
    pub const fn first_day(self) -> Date {
        self.0
    }
}
impl fmt::Display for BillingMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.0.year(), self.0.month() as u8)
    }
}
impl FromStr for BillingMonth {
    type Err = ParseError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}
impl TryFrom<String> for BillingMonth {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl From<BillingMonth> for String {
    fn from(value: BillingMonth) -> Self {
        value.to_string()
    }
}

/// Stripe payment-method identifier used for listing and default selection.
/// This is not a customer ID, card number or client secret.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct StripePaymentMethodId(String);
impl StripePaymentMethodId {
    pub fn parse(value: &str) -> Result<Self, ParseError> {
        if value.len() <= 3
            || value.len() > 255
            || !value.starts_with("pm_")
            || value.contains("_secret_")
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            return Err(ParseError::new("invalid Stripe payment method ID"));
        }
        Ok(Self(value.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for StripePaymentMethodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl FromStr for StripePaymentMethodId {
    type Err = ParseError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}
impl TryFrom<String> for StripePaymentMethodId {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl From<StripePaymentMethodId> for String {
    fn from(value: StripePaymentMethodId) -> Self {
        value.0
    }
}

/// Bounded page size for billing lists (1–100, default 25).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u16", into = "u16")]
pub struct BillingPageSize(u16);
impl BillingPageSize {
    pub fn parse(value: u16) -> Result<Self, ParseError> {
        if (1..=100).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ParseError::new(
                "billing page size must be between 1 and 100",
            ))
        }
    }
    pub const fn get(self) -> u16 {
        self.0
    }
}
impl Default for BillingPageSize {
    fn default() -> Self {
        Self(25)
    }
}
impl TryFrom<u16> for BillingPageSize {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}
impl From<BillingPageSize> for u16 {
    fn from(value: BillingPageSize) -> Self {
        value.0
    }
}
