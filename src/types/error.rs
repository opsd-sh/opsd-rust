use std::{error::Error, fmt};

/// Error returned when text cannot be parsed into an Opsd API domain type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    message: &'static str,
}

impl ParseError {
    pub(crate) const fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl Error for ParseError {}
