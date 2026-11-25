//! Payroll-run domain types and API models.

/// A time-limited bearer link. Handle the URL as private payroll information.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PayslipDownload {
    pub url: String,
    pub filename: String,
    pub mime_type: String,
    pub expires_at: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PayslipExportReceipt {
    pub export_id: super::PayslipExportId,
}

/// A ready export contains a short-lived bearer link to its ZIP.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PayslipExportStatus {
    Preparing,
    Ready { download: PayslipDownload },
    Failed,
    Expired,
}

mod calculations;
mod domain;
mod operations;
mod wire;
pub use calculations::*;

pub use domain::*;
pub use operations::*;

mod actions;
pub use actions::*;
mod history;
pub use history::*;
