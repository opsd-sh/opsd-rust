use std::sync::LazyLock;

use reqwest::{
    Client as HttpClient, RequestBuilder, Response,
    header::{ACCEPT, AUTHORIZATION, HeaderValue},
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    ApiCredential,
    error::{Error, ProblemDetails},
    models::{CreateUserRequest, HelloResponse, User},
};

const ACCEPT_JSON: HeaderValue =
    HeaderValue::from_static("application/json, application/problem+json");
static PRODUCTION_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://api.opsd.sh/v1/").expect("hard-coded production API URL is valid")
});

#[derive(Clone)]
pub struct OpsdClient {
    base_url: Url,
    http_client: HttpClient,
    credential: ApiCredential,
}

impl OpsdClient {
    pub fn new(credential: ApiCredential) -> Result<Self, Error> {
        Ok(Self {
            base_url: (*PRODUCTION_BASE_URL).clone(),
            http_client: HttpClient::new(),
            credential,
        })
    }

    pub fn new_base(base_url: Url, credential: ApiCredential) -> Result<Self, Error> {
        validate_base_url(&base_url)?;

        Ok(Self {
            base_url: normalize_base_url(base_url),
            http_client: HttpClient::new(),
            credential,
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
            .authenticate(self.http_client.get(url))
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
            .authenticate(self.http_client.get(url))
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
            .authenticate(self.http_client.get(url))
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
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder {
        request.header(AUTHORIZATION, self.credential.authorization_header())
    }
}

impl std::fmt::Debug for OpsdClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpsdClient")
            .field("base_url", &self.base_url)
            .field("credential", &self.credential)
            .finish_non_exhaustive()
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
    use crate::ApiCredential;
    use reqwest::header::AUTHORIZATION;
    use url::Url;

    #[test]
    fn defaults_to_production_base_url() {
        let client = OpsdClient::new(ApiCredential::new("test-token").unwrap())
            .expect("default client should be constructed");

        assert_eq!(client.base_url().as_str(), "https://api.opsd.sh/v1/");
    }

    #[test]
    fn preserves_existing_base_path_when_normalizing() {
        let client = OpsdClient::new_base(
            Url::parse("https://api.opsd.sh/v1").expect("URL literal should parse"),
            ApiCredential::new("test-token").unwrap(),
        )
        .expect("base URL with path should be accepted");

        assert_eq!(client.base_url().as_str(), "https://api.opsd.sh/v1/");
    }

    #[test]
    fn bearer_tokens_are_attached_and_redacted() {
        let client = OpsdClient::new(ApiCredential::new("secret-access-token").unwrap()).unwrap();
        let request = client
            .authenticate(client.http_client.get("https://api.opsd.sh/v1/hello/world"))
            .build()
            .unwrap();

        assert_eq!(
            request.headers().get(AUTHORIZATION).unwrap(),
            "Bearer secret-access-token"
        );
        assert!(!format!("{client:?}").contains("secret-access-token"));
    }
}
