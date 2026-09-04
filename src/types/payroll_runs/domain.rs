use std::{fmt, str::FromStr};

use time::{Date, Month, OffsetDateTime};

use crate::types::{EmploymentId, ParseError, PayeSchemeId, PayrollRunId, UserId};

/// Contractual date on which employees in a payroll run are paid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PayrollPaymentDate(Date);

impl PayrollPaymentDate {
    pub fn parse(value: &str) -> Result<Self, ParseError> {
        let invalid = || ParseError::new("payment date must be a valid date in YYYY-MM-DD format");
        let bytes = value.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return Err(invalid());
        }
        let year = value
            .get(0..4)
            .ok_or_else(invalid)?
            .parse()
            .map_err(|_| invalid())?;
        let month = value
            .get(5..7)
            .ok_or_else(invalid)?
            .parse::<u8>()
            .map_err(|_| invalid())?;
        let day = value
            .get(8..10)
            .ok_or_else(invalid)?
            .parse()
            .map_err(|_| invalid())?;
        let month = Month::try_from(month).map_err(|_| invalid())?;
        Date::from_calendar_date(year, month, day)
            .map(Self)
            .map_err(|_| invalid())
    }

    pub const fn from_date(value: Date) -> Self {
        Self(value)
    }

    pub const fn as_date(self) -> Date {
        self.0
    }
}

impl FromStr for PayrollPaymentDate {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PayrollPaymentDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Lifecycle state of a payroll run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayrollRunStatus {
    Draft,
    Finalized,
}

impl PayrollRunStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Finalized => "finalized",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "draft" => Ok(Self::Draft),
            "finalized" => Ok(Self::Finalized),
            _ => Err(ParseError::new("payroll run status is invalid")),
        }
    }
}

impl FromStr for PayrollRunStatus {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PayrollRunStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Payroll run for a PAYE scheme and contractual payment date.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayrollRun {
    pub id: PayrollRunId,
    pub paye_scheme_id: PayeSchemeId,
    pub payment_date: PayrollPaymentDate,
    pub status: PayrollRunStatus,
    pub employment_count: u64,
    pub finalized_at: Option<OffsetDateTime>,
    pub finalized_by_user_id: Option<UserId>,
    pub created_at: OffsetDateTime,
}

/// Payroll run together with its captured employment identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayrollRunDetails {
    pub run: PayrollRun,
    pub employment_ids: Vec<EmploymentId>,
}

#[cfg(test)]
mod tests {
    use super::{PayrollPaymentDate, PayrollRunStatus};

    #[test]
    fn payment_dates_require_valid_iso_calendar_dates() {
        assert_eq!(
            PayrollPaymentDate::parse("2026-09-30").unwrap().to_string(),
            "2026-09-30"
        );
        assert!(PayrollPaymentDate::parse("2026-02-29").is_err());
        assert!(PayrollPaymentDate::parse("30-09-2026").is_err());
    }

    #[test]
    fn payroll_run_statuses_parse_from_the_api_values() {
        assert_eq!(
            PayrollRunStatus::parse("draft").unwrap(),
            PayrollRunStatus::Draft
        );
        assert_eq!(
            PayrollRunStatus::parse("finalized").unwrap(),
            PayrollRunStatus::Finalized
        );
        assert!(PayrollRunStatus::parse("submitted").is_err());
    }
}
