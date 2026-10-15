use super::{ACCEPT_JSON, OpsdClient, decode_response};
use crate::{Error, types::*};
use reqwest::header::ACCEPT;
impl OpsdClient {
    /// Creates a monthly schedule within a PAYE scheme.
    pub async fn create_pay_schedule(
        &self,
        business_id: BusinessId,
        request: &CreatePayScheduleRequest,
    ) -> Result<CreatePayScheduleResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/pay-schedules"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }
    /// Lists schedules belonging to a business.
    pub async fn list_pay_schedules(
        &self,
        business_id: BusinessId,
    ) -> Result<ListPaySchedulesResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/pay-schedules"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }
}
