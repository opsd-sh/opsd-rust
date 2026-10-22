use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response, expect_no_content};
use crate::{
    Error,
    types::{
        BusinessId, CreatePayrollRunRequest, CreatePayrollRunResponse, EmploymentId,
        FinalizePayrollRunResponse, GetPayrollRunResponse, ListPayrollRunsResponse, PayrollRunId,
    },
};

impl OpsdClient {
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
