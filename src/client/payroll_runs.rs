use reqwest::{StatusCode, header::ACCEPT};
use url::{Host, Url};

use super::{ACCEPT_JSON, OpsdClient, decode_response, expect_no_content};
use crate::{
    Error,
    types::{
        BusinessId, CreatePayrollRunRequest, CreatePayrollRunResponse, EmploymentId,
        FinalizePayrollRunResponse, GetPayrollRunResponse, ListPayrollRunsResponse, PayrollRunId,
    },
};

fn validate_payslip_url(base_url: &Url, link: &str) -> Result<Url, Error> {
    let invalid_link = || Error::UnexpectedResponse {
        status: StatusCode::OK,
        body: "invalid payslip download link".into(),
    };
    let url = Url::parse(link).map_err(|_| invalid_link())?;
    let local_http = url.scheme() == "http"
        && match url.host() {
            Some(Host::Domain(host)) => host == "localhost",
            Some(Host::Ipv4(ip)) => ip.is_loopback(),
            Some(Host::Ipv6(ip)) => ip.is_loopback(),
            None => false,
        };
    if url.origin() != base_url.origin()
        || (url.scheme() != "https" && !local_http)
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(invalid_link());
    }
    Ok(url)
}

fn invalid_document(format: &str) -> Error {
    Error::UnexpectedResponse {
        status: StatusCode::OK,
        body: format!("The download returned an invalid {format}."),
    }
}

impl OpsdClient {
    /// Requests a private, expiring link on this API origin for a payment in an accepted run.
    pub async fn request_payslip(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        employment_id: EmploymentId,
    ) -> Result<PayslipDownload, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/employments/{employment_id}/payslip"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return decode_response(response).await;
        }
        let link: PayslipDownload =
            response
                .json()
                .await
                .map_err(|_| Error::UnexpectedResponse {
                    status,
                    body: "invalid payslip download descriptor".into(),
                })?;
        validate_payslip_url(self.base_url(), &link.url)?;
        Ok(link)
    }

    /// Requests a ZIP for the accepted revision. The returned ID is ready to poll.
    pub async fn request_payslip_export(
        &self,
        business: BusinessId,
        run: PayrollRunId,
    ) -> Result<PayslipExportReceipt, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business}/payroll-runs/{run}/payslip-exports"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }

    pub async fn get_payslip_export(
        &self,
        business: BusinessId,
        run: PayrollRunId,
        export: PayslipExportId,
    ) -> Result<PayslipExportStatus, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business}/payroll-runs/{run}/payslip-exports/{export}"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return decode_response(response).await;
        }
        let result: PayslipExportStatus =
            response
                .json()
                .await
                .map_err(|_| Error::UnexpectedResponse {
                    status,
                    body: "invalid payslip export response".into(),
                })?;
        if let PayslipExportStatus::Ready { download } = &result {
            validate_payslip_url(self.base_url(), &download.url)?;
            if download.mime_type != "application/zip" {
                return Err(Error::UnexpectedResponse {
                    status,
                    body: "invalid payslip ZIP descriptor".into(),
                });
            }
        }
        Ok(result)
    }

    /// Streams a ready ZIP to the supplied writer using only the short-lived download link.
    pub async fn write_payslip_zip(
        &self,
        link: &PayslipDownload,
        mut output: impl std::io::Write,
    ) -> Result<(), Error> {
        let mut response = self
            .fetch_payslip_document(link, "application/zip", "ZIP")
            .await?;
        // The signature may span HTTP chunks. Accept an empty archive as well.
        let mut signature = Vec::with_capacity(4);
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| Error::Transport(e.without_url()))?
        {
            let mut data = chunk.as_ref();
            if signature.len() < 4 {
                let count = (4 - signature.len()).min(data.len());
                signature.extend_from_slice(&data[..count]);
                data = &data[count..];
                if signature.len() < 4 {
                    continue;
                }
                if signature != b"PK\x03\x04" && signature != b"PK\x05\x06" {
                    return Err(invalid_document("ZIP"));
                }
                output.write_all(&signature)?;
            }
            output.write_all(data)?;
        }
        if signature.len() < 4 {
            return Err(invalid_document("ZIP"));
        }
        Ok(())
    }

    async fn fetch_payslip_document(
        &self,
        link: &PayslipDownload,
        mime: &str,
        format: &str,
    ) -> Result<reqwest::Response, Error> {
        let url = validate_payslip_url(self.base_url(), &link.url)?;
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Transport(e.without_url()))?;
        let status = response.status();
        if status != StatusCode::OK {
            let body = match status {
                StatusCode::NOT_FOUND => {
                    "Payslip link expired or could not be found. Request a fresh link.".into()
                }
                StatusCode::SERVICE_UNAVAILABLE => {
                    "Payslip download temporarily unavailable. Try again shortly.".into()
                }
                _ => format!("Payslip download returned HTTP {}.", status.as_u16()),
            };
            return Err(Error::UnexpectedResponse { status, body });
        }

        let valid = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(';').next())
            .is_some_and(|v| v.trim().eq_ignore_ascii_case(mime));
        if !valid {
            return Err(invalid_document(format));
        }
        Ok(response)
    }

    /// Fetches a payslip from the API origin using only the download link.
    pub async fn fetch_payslip_pdf(&self, link: &PayslipDownload) -> Result<Vec<u8>, Error> {
        let response = self
            .fetch_payslip_document(link, "application/pdf", "PDF")
            .await?;
        let body = response
            .bytes()
            .await
            .map_err(|e| Error::Transport(e.without_url()))?;
        if !body.starts_with(b"%PDF-") {
            return Err(invalid_document("PDF"));
        }
        Ok(body.to_vec())
    }

    /// Creates a draft payroll run belonging to a business.
    pub async fn create_payroll_run(
        &self,
        business_id: BusinessId,
        request: &CreatePayrollRunRequest,
    ) -> Result<CreatePayrollRunResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/payroll-runs"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists payroll runs belonging to a business.
    pub async fn list_payroll_runs(
        &self,
        business_id: BusinessId,
    ) -> Result<ListPayrollRunsResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/payroll-runs"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Gets a payroll run and its included employments.
    pub async fn get_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
    ) -> Result<GetPayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Finalises and submits a run; inspect the returned state for its outcome.
    pub async fn finalize_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        request: &FinalizePayrollRunRequest,
    ) -> Result<FinalizePayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/finalize"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Includes an employment in a draft payroll run.
    pub async fn include_payroll_run_employment(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        employment_id: EmploymentId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/employments/{employment_id}"
        ));
        let response = self
            .authenticate(self.http_client.put(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Excludes an employment from a draft payroll run.
    pub async fn exclude_payroll_run_employment(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        employment_id: EmploymentId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/employments/{employment_id}"
        ));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }
}

use crate::types::*;
impl OpsdClient {
    /// Calculates a draft using current settings without saving results.
    pub async fn preview_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
    ) -> Result<PayrollPreviewResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/preview"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
    /// Overrides one saved draft gross payment, without changing regular monthly pay.
    pub async fn update_payroll_payment(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        employment_id: EmploymentId,
        request: &UpdatePayrollPaymentRequest,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/employments/{employment_id}"
        ));
        let response = self
            .authenticate(self.http_client.patch(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        expect_no_content(response).await
    }
    /// Discards a draft run. Finalised payroll cannot be deleted.
    pub async fn discard_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}"
        ));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        expect_no_content(response).await
    }
}

impl OpsdClient {
    /// Retry a run using its expected version and available actions.
    pub async fn retry_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        request: &RetryPayrollRunRequest,
    ) -> Result<GetPayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/retry"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Resume a run using its expected version and available actions.
    pub async fn resume_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        request: &ResumePayrollRunRequest,
    ) -> Result<GetPayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/resume"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Reopen a run using its expected version and available actions.
    pub async fn reopen_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        request: &VersionedActionRequest,
    ) -> Result<GetPayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/reopen"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Cancel a run using its expected version and available actions.
    pub async fn cancel_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
        request: &VersionedActionRequest,
    ) -> Result<GetPayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/cancel"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Reads saved revisions, attempts and actions.
    pub async fn get_payroll_run_history(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
    ) -> Result<PayrollRunHistoryResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/history"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payslip_links_use_the_configured_api_origin() {
        for origin in [
            "https://api.opsd.sh",
            "https://api.dev.opsd.sh",
            "http://localhost:3000",
            "http://127.0.0.1:3000",
            "http://[::1]:3000",
        ] {
            let base = Url::parse(&format!("{origin}/v1/")).unwrap();
            let link = format!("{origin}/downloads/example");
            assert_eq!(validate_payslip_url(&base, &link).unwrap().as_str(), link);
        }
    }

    #[test]
    fn payslip_links_reject_other_origins_and_unsafe_urls() {
        for (base, link) in [
            (
                "https://api.dev.opsd.sh",
                "https://api.opsd.sh/downloads/example",
            ),
            (
                "https://api.opsd.sh",
                "https://api.dev.opsd.sh/downloads/example",
            ),
            (
                "http://localhost:3000",
                "http://localhost:3001/downloads/example",
            ),
            (
                "https://api.dev.opsd.sh",
                "http://api.dev.opsd.sh/downloads/example",
            ),
            (
                "http://api.dev.opsd.sh",
                "http://api.dev.opsd.sh/downloads/example",
            ),
            (
                "https://api.dev.opsd.sh",
                "https://user@api.dev.opsd.sh/downloads/example",
            ),
            ("https://api.dev.opsd.sh", "/downloads/example"),
        ] {
            let base = Url::parse(base).unwrap();
            assert!(
                validate_payslip_url(&base, link).is_err(),
                "{base} accepted {link}"
            );
        }
    }
}
