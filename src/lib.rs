#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
mod credential;
#[cfg(feature = "client")]
mod error;
pub mod types;

#[cfg(feature = "client")]
pub use client::OpsdClient;
#[cfg(feature = "client")]
pub use credential::ApiCredential;
#[cfg(feature = "client")]
pub use error::{Error, ProblemDetails};
