use std::fmt::Display;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid API credential")]
    InvalidApiCredential,

    #[error("invalid base URL `{base_url}`: {message}")]
    InvalidBaseUrl { base_url: String, message: String },

    #[error("request failed: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("API request failed: {problem}")]
    Api {
        status: StatusCode,
        problem: ProblemDetails,
    },

    #[error("API request returned an unexpected response: {body}")]
    UnexpectedResponse { status: StatusCode, body: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub category: String,
}

impl Display for ProblemDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}

impl Error {
    pub fn problem_details(&self) -> Option<&ProblemDetails> {
        match self {
            Error::Api { problem, .. } => Some(problem),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProblemDetails;

    #[test]
    fn problem_details_deserializes_from_opsd_error_body() {
        let json = r#"{
            "type": "https://api.opsd.sh/problems/not-found",
            "title": "Not Found",
            "status": 404,
            "detail": "no route found for GET /missing",
            "category": "request"
        }"#;

        let parsed: ProblemDetails =
            serde_json::from_str(json).expect("problem details should deserialize");

        assert_eq!(
            parsed,
            ProblemDetails {
                problem_type: "https://api.opsd.sh/problems/not-found".to_string(),
                title: "Not Found".to_string(),
                status: 404,
                detail: "no route found for GET /missing".to_string(),
                category: "request".to_string(),
            }
        );
    }
}
