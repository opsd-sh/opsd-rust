use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response, expect_no_content};
use crate::{
    Error,
    types::{
        BusinessId, CreateEmploymentRequest, CreateEmploymentResponse, ListEmploymentsResponse,
    },
};

impl OpsdClient {
    /// Creates an employment belonging to a business.
    pub async fn create_employment(
        &self,
        business_id: BusinessId,
        request: &CreateEmploymentRequest,
    ) -> Result<CreateEmploymentResponse, Error> {
        self.post_employment(business_id, request).await
    }

    /// Continue a job previously paid under the same PAYE scheme.
    pub async fn import_employment(
        &self,
        business_id: BusinessId,
        request: &ImportEmploymentRequest,
    ) -> Result<CreateEmploymentResponse, Error> {
        self.post_employment(business_id, request).await
    }

    async fn post_employment(
        &self,
        business_id: BusinessId,
        request: &impl serde::Serialize,
    ) -> Result<CreateEmploymentResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/employments"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists employments belonging to a business.
    pub async fn list_employments(
        &self,
        business_id: BusinessId,
    ) -> Result<ListEmploymentsResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/employments"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }
}

use crate::types::*;
impl OpsdClient {
    /// Reads current employment defaults and active-year tax settings.
    pub async fn get_employment(
        &self,
        business_id: BusinessId,
        employment_id: EmploymentId,
    ) -> Result<GetEmploymentResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/employments/{employment_id}"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
    /// Deletes an unused employment and its imported balances, preserving the employee.
    pub async fn discard_employment(
        &self,
        business_id: BusinessId,
        employment_id: EmploymentId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/employments/{employment_id}"
        ));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        expect_no_content(response).await
    }

    /// Updates starter settings or applies an HMRC notice.
    pub async fn update_employment(
        &self,
        business_id: BusinessId,
        employment_id: EmploymentId,
        request: &UpdateEmploymentRequest,
    ) -> Result<UpdateEmploymentResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/employments/{employment_id}"
        ));
        let response = self
            .authenticate(self.http_client.patch(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
}

impl OpsdClient {
    /// Reads the opening balances retained for an imported employment.
    pub async fn get_employment_payroll_import(
        &self,
        business_id: BusinessId,
        employment_id: EmploymentId,
    ) -> Result<PayrollImportOpening, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/employments/{employment_id}/payroll-import"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }

    /// Corrects selected external payroll figures before the first local finalisation.
    pub async fn amend_employment_payroll_import(
        &self,
        business_id: BusinessId,
        employment_id: EmploymentId,
        changes: &PayrollImportAmendment,
    ) -> Result<PayrollImportOpening, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/employments/{employment_id}/payroll-import"
        ));
        let response = self
            .authenticate(self.http_client.patch(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(changes)
            .send()
            .await?;
        decode_response(response).await
    }
}
