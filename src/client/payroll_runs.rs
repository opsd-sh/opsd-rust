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

    /// Finalizes a payroll run and returns its captured employments.
    pub async fn finalize_payroll_run(
        &self,
        business_id: BusinessId,
        payroll_run_id: PayrollRunId,
    ) -> Result<FinalizePayrollRunResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/payroll-runs/{payroll_run_id}/finalize"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
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
