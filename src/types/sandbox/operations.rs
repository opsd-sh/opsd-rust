use serde::{Deserialize, Serialize};

use super::wire;

/// Response returned by a hello-world sandbox endpoint.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(from = "wire::HelloResponse", into = "wire::HelloResponse")]
pub struct HelloResponse {
    pub message: String,
}

impl From<wire::HelloResponse> for HelloResponse {
    fn from(value: wire::HelloResponse) -> Self {
        Self {
            message: value.message,
        }
    }
}

impl From<HelloResponse> for wire::HelloResponse {
    fn from(value: HelloResponse) -> Self {
        Self {
            message: value.message,
        }
    }
}
