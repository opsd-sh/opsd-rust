use reqwest::header::HeaderValue;

use crate::Error;

/// A secret used to authenticate requests to the public API.
///
/// OAuth access tokens and API keys are both bearer credentials at the HTTP
/// boundary. Their different acquisition and lifecycle rules remain the
/// responsibility of the calling application.
#[derive(Clone)]
pub struct ApiCredential(HeaderValue);

impl ApiCredential {
    pub fn new(secret: impl AsRef<str>) -> Result<Self, Error> {
        let secret = secret.as_ref();
        if secret.is_empty()
            || !secret.is_ascii()
            || secret.bytes().any(|byte| byte.is_ascii_whitespace())
        {
            return Err(Error::InvalidApiCredential);
        }

        let mut value = HeaderValue::from_str(&format!("Bearer {secret}"))
            .map_err(|_error| Error::InvalidApiCredential)?;
        value.set_sensitive(true);
        Ok(Self(value))
    }

    pub(crate) fn authorization_header(&self) -> HeaderValue {
        self.0.clone()
    }
}

impl std::fmt::Debug for ApiCredential {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ApiCredential([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::ApiCredential;
    use crate::Error;

    #[test]
    fn credentials_are_validated_and_marked_sensitive() {
        let credential = ApiCredential::new("opsd_key_secret").unwrap();
        let header = credential.authorization_header();

        assert_eq!(header, "Bearer opsd_key_secret");
        assert!(header.is_sensitive());
        assert_eq!(format!("{credential:?}"), "ApiCredential([REDACTED])");
    }

    #[test]
    fn empty_malformed_and_whitespace_credentials_are_rejected() {
        for secret in ["", "contains space", "contains\nnewline", "£"] {
            assert!(matches!(
                ApiCredential::new(secret),
                Err(Error::InvalidApiCredential)
            ));
        }
    }
}
