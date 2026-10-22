//! Public reporting inputs. The API validates HMRC's current field rules.
use super::{ParseError, PayrollPaymentDate};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

macro_rules! string_enum {
    ($name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name { $(#[serde(rename = $value)] $variant),+ }
        impl $name {
            pub fn parse(value: &str) -> Result<Self, ParseError> {
                match value { $($value => Ok(Self::$variant),)+
                    _ => Err(ParseError::new(concat!("expected one of: ", $($value, " ",)+))), }
            }
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $value,)+ }
            }
        }
        impl FromStr for $name {
            type Err = ParseError;
            fn from_str(value: &str) -> Result<Self, Self::Err> { Self::parse(value) }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
        }
    };
}
string_enum!(EmployeeSex { Male => "M", Female => "F" });
string_enum!(NormalWeeklyHoursBand {
    LessThan16 => "less_than_16",
    SixteenToLessThan24 => "sixteen_to_less_than_24",
    TwentyFourToLessThan30 => "twenty_four_to_less_than_30",
    ThirtyOrMore => "thirty_or_more",
    Other => "other",
});
// Explicit declarations: loan deductions are not supported yet. Do not default
// a missing answer to NotApplicable; the operator must establish it.
string_enum!(StudentLoan { NotApplicable => "not_applicable" });
string_enum!(PostgraduateLoan { NotApplicable => "not_applicable" });
string_enum!(LateReportingReason {
    NotionalPaymentToExpat => "A",
    EmploymentRelatedSecurity => "B",
    OtherNotionalPayment => "C",
    Class1NiButReportedOnP11dOrP9d => "D",
    MicroEmployerRelaxation => "E",
    NoDeductionsWorkingSheetOrImpracticalToReportOnDay => "F",
    ReasonableExcuse => "G",
    CorrectionToEarlierSubmission => "H",
});

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateOfBirth(PayrollPaymentDate);
impl FromStr for DateOfBirth {
    type Err = ParseError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        PayrollPaymentDate::parse(value)
            .map(Self)
            .map_err(|_| ParseError::new("date of birth must be a valid date in YYYY-MM-DD format"))
    }
}
impl fmt::Display for DateOfBirth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum EmployeeAddress {
    Uk {
        lines: Vec<String>,
        postcode: String,
    },
    Foreign {
        lines: Vec<String>,
        country: String,
    },
    Lines {
        lines: Vec<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeReportingDetails {
    pub name: String,
    pub address: Option<ReportingAddress>,
    pub contact_name: Option<ReportingContactName>,
    pub email: Option<String>,
    pub telephone: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportingAddress {
    pub lines: Vec<String>,
    pub post_code: Option<String>,
    pub country: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportingContactName {
    pub title: Option<String>,
    pub forenames: Vec<String>,
    pub surname: String,
}

/// A nullable PATCH field. Omitted means unchanged; null means clear.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum FieldUpdate<T> {
    #[default]
    Unchanged,
    Clear,
    Set(T),
}
impl<T> FieldUpdate<T> {
    pub fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }
}
impl<T: Serialize> Serialize for FieldUpdate<T> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Set(value) => value.serialize(s),
            _ => s.serialize_none(),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for FieldUpdate<T> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(match Option::<T>::deserialize(d)? {
            Some(v) => Self::Set(v),
            None => Self::Clear,
        })
    }
}
