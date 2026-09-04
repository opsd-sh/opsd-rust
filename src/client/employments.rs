use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response};
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
