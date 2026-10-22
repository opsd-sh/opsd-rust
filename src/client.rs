mod billing;
mod businesses;
mod employees;
mod employments;
mod hello;
mod pay_schedules;
mod paye_schemes;
mod payroll_runs;
mod practices;

use std::sync::LazyLock;

use reqwest::{
    Client as HttpClient, RequestBuilder, Response,
    header::{AUTHORIZATION, HeaderValue},
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    ApiCredential,
    error::{Error, ProblemDetails},
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
        Self::new_base((*PRODUCTION_BASE_URL).clone(), credential)
    }

    pub fn new_base(base_url: Url, credential: ApiCredential) -> Result<Self, Error> {
        validate_base_url(&base_url)?;

        Ok(Self {
            base_url: normalize_base_url(base_url),
            // Redirects can replay request bodies containing HMRC credentials.
            http_client: HttpClient::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            credential,
        })
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    fn authenticate(&self, request: RequestBuilder) -> RequestBuilder {
        request.header(AUTHORIZATION, self.credential.authorization_header())
    }

    fn endpoint(&self, path: &str) -> Url {
        self.base_url
            .join(path)
            .expect("client endpoint path must be valid")
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
    let retry_after_seconds = response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok());
    let body = response.bytes().await?;

    if status.is_success() {
        return serde_json::from_slice(&body).map_err(|_| Error::UnexpectedResponse {
            status,
            body: String::from_utf8_lossy(&body).into_owned(),
        });
    }

    Err(decode_error(status, &body, retry_after_seconds))
}

async fn expect_no_content(response: Response) -> Result<(), Error> {
    let status = response.status();
    let retry_after_seconds = response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok());
    let body = response.bytes().await?;

    if status == reqwest::StatusCode::NO_CONTENT {
        return Ok(());
    }

    Err(decode_error(status, &body, retry_after_seconds))
}

fn decode_error(
    status: reqwest::StatusCode,
    body: &[u8],
    retry_after_seconds: Option<u64>,
) -> Error {
    if let Ok(problem) = serde_json::from_slice::<ProblemDetails>(body) {
        return Error::Api {
            status,
            problem,
            retry_after_seconds,
        };
    }

    Error::UnexpectedResponse {
        status,
        body: String::from_utf8_lossy(body).into_owned(),
    }
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

    #[tokio::test]
    async fn constructors_do_not_forward_credential_bodies_on_redirects() {
        use crate::types::{FinalizePayrollRunRequest, HmrcPassword};
        use std::{
            io::{Read, Write},
            net::TcpListener,
            thread,
            time::Duration,
        };

        for production_constructor in [false, true] {
            for status in [307, 308] {
                let origin = TcpListener::bind(("127.0.0.1", 0)).unwrap();
                let destination = TcpListener::bind(("127.0.0.1", 0)).unwrap();
                destination.set_nonblocking(true).unwrap();
                let base_url =
                    Url::parse(&format!("http://{}/v1/", origin.local_addr().unwrap())).unwrap();
                let destination_url =
                    format!("http://{}/redirected", destination.local_addr().unwrap());
                let server = thread::spawn(move || {
                    let (mut stream, _) = origin.accept().unwrap();
                    stream
                        .set_read_timeout(Some(Duration::from_secs(5)))
                        .unwrap();
                    let mut request = Vec::new();
                    let mut buffer = [0; 4096];
                    loop {
                        let n = stream.read(&mut buffer).unwrap();
                        assert_ne!(n, 0);
                        request.extend_from_slice(&buffer[..n]);
                        if let Some(end) = request.windows(4).position(|v| v == b"\r\n\r\n") {
                            let headers = String::from_utf8_lossy(&request[..end]);
                            let length = headers
                                .lines()
                                .find_map(|line| {
                                    line.to_ascii_lowercase()
                                        .strip_prefix("content-length: ")
                                        .map(|value| value.parse::<usize>().unwrap())
                                })
                                .unwrap();
                            if request.len() >= end + 4 + length {
                                break;
                            }
                        }
                    }
                    assert!(String::from_utf8_lossy(&request).contains("synthetic-password"));
                    write!(stream, "HTTP/1.1 {status} Redirect\r\nLocation: {destination_url}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
                });
                let credential = ApiCredential::new("test-token").unwrap();
                let client = if production_constructor {
                    let mut client = OpsdClient::new(credential).unwrap();
                    // Keep new()'s HTTP configuration, but direct this test to a local server.
                    client.base_url = base_url;
                    client
                } else {
                    OpsdClient::new_base(base_url, credential).unwrap()
                };
                let outcome = tokio::time::timeout(
                    Duration::from_secs(5),
                    client.finalize_payroll_run(
                        "22222222-2222-4222-8222-222222222222".parse().unwrap(),
                        "88888888-8888-4888-8888-888888888888".parse().unwrap(),
                        &FinalizePayrollRunRequest {
                            expected_version: 0,
                            sender_id: "synthetic-sender".into(),
                            password: HmrcPassword::new("synthetic-password"),
                            late_reporting_reason: None,
                        },
                    ),
                )
                .await
                .expect("a redirect must return without contacting its destination");
                server.join().unwrap();
                assert!(
                    matches!(outcome, Err(crate::Error::UnexpectedResponse { status: actual, .. }) if actual.as_u16() == status)
                );
                assert_eq!(
                    destination.accept().unwrap_err().kind(),
                    std::io::ErrorKind::WouldBlock
                );
            }
        }
    }
}
