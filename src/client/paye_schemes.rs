use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response};
use crate::{
    Error,
    types::{
        BusinessId, CreatePayeSchemeRequest, CreatePayeSchemeResponse, ListPayeSchemesResponse,
    },
};

impl OpsdClient {
    /// Creates a PAYE scheme belonging to a business.
    pub async fn create_paye_scheme(
        &self,
        business_id: BusinessId,
        request: &CreatePayeSchemeRequest,
    ) -> Result<CreatePayeSchemeResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/paye-schemes"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists PAYE schemes belonging to a business.
    pub async fn list_paye_schemes(
        &self,
        business_id: BusinessId,
    ) -> Result<ListPayeSchemesResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/paye-schemes"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }
}
