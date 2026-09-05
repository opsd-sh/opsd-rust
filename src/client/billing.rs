//! Business billing operations authenticated with an API credential.

use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response};
use crate::{
    Error,
    types::{BusinessId, GetBillingStatusResponse},
};

impl OpsdClient {
    /// Checks whether the business has a recorded payment setup.
    ///
    /// Requires business administrator access. No Checkout session ID is
    /// needed. This reports saved confirmation rather than checking the
    /// payment provider live, and does not guarantee a future charge succeeds.
    pub async fn get_billing_status(
        &self,
        business_id: BusinessId,
    ) -> Result<GetBillingStatusResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/billing/status"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
}
