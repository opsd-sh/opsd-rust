use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response};
use crate::{Error, types::HelloResponse};

impl OpsdClient {
    pub async fn hello_world(&self) -> Result<HelloResponse, Error> {
        let response = self
            .authenticate(self.http_client.get(self.endpoint("hello/world")))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    pub async fn hello_application(&self) -> Result<HelloResponse, Error> {
        let response = self
            .authenticate(self.http_client.get(self.endpoint("hello/application")))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }
}
