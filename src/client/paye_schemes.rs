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

use crate::types::*;
impl OpsdClient {
    /// Requests year rollover, including an end-of-year FPS and a new-year EPS.
    /// The EPS declares the supplied Employment Allowance decision.
    /// Use the returned ID and available actions to resume an interrupted rollover.
    pub async fn rollover_paye_scheme(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
        request: &RolloverPayeSchemeRequest,
    ) -> Result<RolloverPayeSchemeResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollover"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
}

impl OpsdClient {
    /// Lists active and completed rollovers.
    pub async fn list_paye_scheme_rollovers(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
    ) -> Result<Vec<RolloverPayeSchemeResponse>, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollovers"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
    /// Reads saved rollover progress or history.
    pub async fn get_paye_scheme_rollover(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
        rollover_id: PayeSchemeRolloverId,
    ) -> Result<RolloverPayeSchemeResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollovers/{rollover_id}"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
    /// Reads saved rollover progress or history.
    pub async fn get_paye_scheme_rollover_history(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
        rollover_id: PayeSchemeRolloverId,
    ) -> Result<RolloverHistoryResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollovers/{rollover_id}/history"
        ));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
    /// Retry the specified rollover using its expected version.
    pub async fn retry_paye_scheme_rollover(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
        rollover_id: PayeSchemeRolloverId,
        request: &RetryRolloverRequest,
    ) -> Result<RolloverPayeSchemeResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollovers/{rollover_id}/retry"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Resume the specified rollover using its expected version.
    pub async fn resume_paye_scheme_rollover(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
        rollover_id: PayeSchemeRolloverId,
        request: &ResumeRolloverRequest,
    ) -> Result<RolloverPayeSchemeResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollovers/{rollover_id}/resume"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Cancel the specified rollover using its expected version.
    pub async fn cancel_paye_scheme_rollover(
        &self,
        business_id: BusinessId,
        paye_scheme_id: PayeSchemeId,
        rollover_id: PayeSchemeRolloverId,
        request: &VersionedActionRequest,
    ) -> Result<RolloverPayeSchemeResponse, Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/paye-schemes/{paye_scheme_id}/rollovers/{rollover_id}/cancel"
        ));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
}
