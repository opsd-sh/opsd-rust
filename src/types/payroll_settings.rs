//! Public payroll inputs. HMRC rules and calculations remain on the server.
use crate::types::ParseError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A non-negative amount of sterling in whole pence, bounded by the API's i64 storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "i64", into = "i64")]
pub struct NonNegativePence(i64);
impl NonNegativePence {
    pub fn new(value: i64) -> Result<Self, ParseError> {
        if value < 0 {
            return Err(ParseError::new("amount in pence must not be negative"));
        }
        Ok(Self(value))
    }
    pub const fn pence(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for NonNegativePence {
    type Error = ParseError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<NonNegativePence> for i64 {
    fn from(value: NonNegativePence) -> Self {
        value.0
    }
}
impl FromStr for NonNegativePence {
    type Err = ParseError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(
            value
                .parse()
                .map_err(|_| ParseError::new("amount must be whole pence"))?,
        )
    }
}
impl fmt::Display for NonNegativePence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! string_enum {
    ($name:ident, $error:literal, {$($variant:ident => $wire:literal),+ $(,)?}) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name { $(#[serde(rename=$wire)] $variant),+ }
        impl $name {
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $wire),+ } }
            pub fn parse(value: &str) -> Result<Self, ParseError> {
                match value { $($wire => Ok(Self::$variant)),+, _ => Err(ParseError::new($error)) }
            }
        }
        impl FromStr for $name { type Err=ParseError; fn from_str(v:&str)->Result<Self,Self::Err>{Self::parse(v)} }
        impl fmt::Display for $name { fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{f.write_str(self.as_str())} }
    }
}
string_enum!(TaxBasis, "tax basis must be cumulative or non_cumulative", {
    Cumulative => "cumulative", NonCumulative => "non_cumulative"
});
string_enum!(EmploymentAllowance, "employment allowance must be claiming or not_claiming", {
    Claiming => "claiming", NotClaiming => "not_claiming"
});
string_enum!(PayFrequency, "only monthly schedules are supported", { Monthly => "monthly" });
string_enum!(StarterDeclaration, "starter declaration must be A, B or C", { A => "A", B => "B", C => "C" });
string_enum!(NiCategory, "unsupported National Insurance category", {
    A=>"A", B=>"B", C=>"C", D=>"D", E=>"E", F=>"F", H=>"H", I=>"I", J=>"J", K=>"K", L=>"L", M=>"M", N=>"N", S=>"S", V=>"V", X=>"X", Z=>"Z"
});

/// Calendar date used in employment evidence. Validity for a tax year is checked by the server.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EmploymentDate(crate::types::PayrollPaymentDate);
impl EmploymentDate {
    pub fn parse(value: &str) -> Result<Self, ParseError> {
        let bytes = value.as_bytes();
        if bytes.len() != 10
            || !bytes.iter().enumerate().all(|(i, c)| {
                if i == 4 || i == 7 {
                    *c == b'-'
                } else {
                    c.is_ascii_digit()
                }
            })
            || value.starts_with("0000")
        {
            return Err(ParseError::new(
                "employment date must be a valid date in YYYY-MM-DD format",
            ));
        }
        crate::types::PayrollPaymentDate::parse(value).map(Self)
    }
    pub fn as_date(self) -> time::Date {
        self.0.as_date()
    }
}
impl FromStr for EmploymentDate {
    type Err = ParseError;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Self::parse(v)
    }
}
impl fmt::Display for EmploymentDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl TryFrom<String> for EmploymentDate {
    type Error = ParseError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Self::parse(&v)
    }
}
impl From<EmploymentDate> for String {
    fn from(v: EmploymentDate) -> Self {
        v.to_string()
    }
}

/// Earlier-job totals for a specified tax year, never this employment's YTD.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviousEmploymentAmounts {
    pub taxable_pay_pence: NonNegativePence,
    /// Signed: tax refunds are negative.
    pub tax_paid_pence: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "basis", rename_all = "snake_case", deny_unknown_fields)]
pub enum P45TaxDetails {
    Cumulative {
        total_pay_to_date_pence: NonNegativePence,
        total_tax_to_date_pence: i64,
    },
    NonCumulative {},
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P45 {
    pub leaving_date: EmploymentDate,
    /// An unsuffixed code; use tax_details for W1/M1. The server validates the code.
    pub tax_code: String,
    pub tax_details: P45TaxDetails,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum StarterInformation {
    /// The P45 from the last previous job. Supply a declaration when its age requires one.
    P45 {
        p45: P45,
        #[serde(skip_serializing_if = "Option::is_none")]
        starter_declaration: Option<StarterDeclaration>,
    },
    Checklist {
        declaration: StarterDeclaration,
    },
    NotProvided {},
}

/// In-year HMRC instruction. Apply it when it should take effect.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tax_basis", rename_all = "snake_case", deny_unknown_fields)]
pub enum TaxCodeNotice {
    Cumulative {
        tax_year_start: i32,
        tax_code: String,
        /// Omission retains stored cumulative totals; switching from non-cumulative needs explicit totals.
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_employment: Option<PreviousEmploymentAmounts>,
    },
    NonCumulative {
        tax_year_start: i32,
        tax_code: String,
    },
}
