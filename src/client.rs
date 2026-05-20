use reqwest::{
    Client as HttpClient, Response,
    header::{ACCEPT, HeaderValue},
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    error::{Error, ProblemDetails},
    models::{CreateUserRequest, HelloResponse, User},
};

const ACCEPT_JSON: HeaderValue =
    HeaderValue::from_static("application/json, application/problem+json");
const PRODUCTION_BASE_URL: &str = "https://api.opsd.sh/";

#[derive(Clone, Debug)]
pub struct OpsdClient {
    base_url: Url,
    http_client: HttpClient,
}

impl OpsdClient {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            base_url: Url::parse(PRODUCTION_BASE_URL).expect("hard-coded base URL is valid"),
            http_client: HttpClient::new(),
        })
    }

    pub fn new_base(base_url: Url) -> Result<Self, Error> {
        validate_base_url(&base_url)?;

        Ok(Self {
            base_url: normalize_base_url(base_url),
            http_client: HttpClient::new(),
        })
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub async fn hello_world(&self) -> Result<HelloResponse, Error> {
        let url = self
            .base_url
            .join("hello/world")
            .expect("hard-coded endpoint path must be valid");
        let response = self
            .http_client
            .get(url)
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    pub async fn hello_application(&self) -> Result<HelloResponse, Error> {
        let url = self
            .base_url
            .join("hello/application")
            .expect("hard-coded endpoint path must be valid");
        let response = self
            .http_client
            .get(url)
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let url = self
            .base_url
            .join("test/users")
            .expect("hard-coded endpoint path must be valid");
        let response = self
            .http_client
            .get(url)
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<User, Error> {
        let url = self
            .base_url
            .join("test/users")
            .expect("hard-coded endpoint path must be valid");
        let response = self
            .http_client
            .post(url)
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }
}

fn normalize_base_url(mut url: Url) -> Url {
    if !url.path().ends_with('/') {
        url.set_path(&format!("{}/", url.path()));
    }

    url
}

fn validate_base_url(base_url: &Url) -> Result<(), Error> {
    if base_url.cannot_be_a_base() {
        return Err(Error::InvalidBaseUrl {
            base_url: base_url.to_string(),
            message: "URL cannot be used as a base URL".to_string(),
        });
    }

    Ok(())
}

async fn decode_response<T>(response: Response) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let status = response.status();
    let body = response.bytes().await?;

    if status.is_success() {
        return serde_json::from_slice(&body).map_err(|_| Error::UnexpectedResponse {
            status,
            body: String::from_utf8_lossy(&body).into_owned(),
        });
    }

    if let Ok(problem) = serde_json::from_slice::<ProblemDetails>(&body) {
        return Err(Error::Api { status, problem });
    }

    Err(Error::UnexpectedResponse {
        status,
        body: String::from_utf8_lossy(&body).into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::OpsdClient;
    use url::Url;

    #[test]
    fn defaults_to_production_base_url() {
        let client = OpsdClient::new().expect("default client should be constructed");

        assert_eq!(client.base_url().as_str(), "https://api.opsd.sh/");
    }

    #[test]
    fn preserves_existing_base_path_when_normalizing() {
        let client = OpsdClient::new_base(
            Url::parse("https://api.opsd.sh/v1").expect("URL literal should parse"),
        )
        .expect("base URL with path should be accepted");

        assert_eq!(client.base_url().as_str(), "https://api.opsd.sh/v1/");
    }
}
