use crate::types::{PayFrequency, PayScheduleId, PayeSchemeId};
use time::OffsetDateTime;
/// Human-readable schedule name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayScheduleName(String);
impl PayScheduleName {
    pub fn parse(value: &str) -> Result<Self, crate::types::ParseError> {
        let value = value.trim();
        if value.is_empty() || value.chars().count() > 120 || value.chars().any(char::is_control) {
            return Err(crate::types::ParseError::new(
                "pay schedule name must contain 1 to 120 characters without control characters",
            ));
        }
        Ok(Self(value.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::str::FromStr for PayScheduleName {
    type Err = crate::types::ParseError;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Self::parse(v)
    }
}
impl std::fmt::Display for PayScheduleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaySchedule {
    pub id: PayScheduleId,
    pub paye_scheme_id: PayeSchemeId,
    pub name: PayScheduleName,
    pub frequency: PayFrequency,
    pub created_at: OffsetDateTime,
}
