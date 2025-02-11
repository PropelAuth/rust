use url::Url;

use crate::apis::auth_service_api::token_verification_metadata;
use crate::apis::configuration::Configuration;
use crate::models::AuthTokenVerificationMetadata;
use crate::propelauth::access_token::AccessTokenService;
use crate::propelauth::api_key::ApiKeyService;
use crate::propelauth::errors::InitializationError;
use crate::propelauth::helpers::map_autogenerated_error;
use crate::propelauth::options::{AuthOptions, AuthOptionsWithTokenVerification};
use crate::propelauth::org::OrgService;
use crate::propelauth::token::TokenService;
use crate::propelauth::user::UserService;

/// The main entrypoint of this library.
/// All authentication, authorization and API requests starts from this struct
#[derive(Debug, Clone)]
pub struct PropelAuth {
    config: Configuration,
    token_verification_metadata: AuthTokenVerificationMetadata,
    issuer: String,
}

impl PropelAuth {
    /// Initializes the PropelAuth library without making any external requests. This contrasts
    /// with `fetch_and_init` which will fetch the metadata needed to validate access tokens
    pub fn init(opts: AuthOptionsWithTokenVerification) -> Result<PropelAuth, InitializationError> {
        let auth_url = validate_auth_url(&opts.auth_url)?;
        let configuration = Configuration {
            base_path: auth_url.clone(),
            bearer_access_token: Some(opts.api_key),
            ..Default::default()
        };

        Ok(PropelAuth {
            config: configuration,
            token_verification_metadata: opts.manual_token_verification_metadata,
            issuer: auth_url,
        })
    }

    /// Initializes the PropelAuth library by making a single external request. This contrasts
    /// with `init` where you manually specify the metadata needed to validate access tokens
    pub async fn fetch_and_init(opts: AuthOptions) -> Result<PropelAuth, InitializationError> {
        let auth_url = validate_auth_url(&opts.auth_url)?;
        let configuration = Configuration {
            base_path: auth_url.clone(),
            bearer_access_token: Some(opts.api_key),
            ..Default::default()
        };

        let token_verification_metadata = token_verification_metadata(&configuration)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    InitializationError::UnexpectedException,
                    |status, _| match status.as_u16() {
                        401 => InitializationError::InvalidApiKey,
                        429 => InitializationError::PropelAuthRateLimit,
                        _ => InitializationError::UnexpectedException,
                    },
                )
            })?;

        Ok(PropelAuth {
            config: configuration,
            token_verification_metadata,
            issuer: auth_url,
        })
    }

    /// API requests related to users
    pub fn user(&self) -> UserService {
        UserService {
            config: &self.config,
        }
    }

    /// API requests related to organizations
    pub fn org(&self) -> OrgService {
        OrgService {
            config: &self.config,
        }
    }

    /// API requests related to organizations
    pub fn api_key(&self) -> ApiKeyService {
        ApiKeyService {
            config: &self.config,
        }
    }

    /// Verify access tokens from your frontend
    pub fn verify(&self) -> TokenService {
        TokenService {
            token_verification_metadata: &self.token_verification_metadata,
            issuer: &self.issuer,
        }
    }

    /// API requests related to access tokens.
    pub fn access_token(&self) -> AccessTokenService {
        AccessTokenService {
            config: &self.config,
        }
    }
}

fn validate_auth_url(auth_url: &str) -> Result<String, InitializationError> {
    Ok(Url::parse(auth_url)
        .map_err(|_| InitializationError::InvalidAuthUrl)?
        .origin()
        .ascii_serialization())
}

#[cfg(test)]
mod tests {
    use crate::propelauth::auth::validate_auth_url;
    use crate::propelauth::errors::InitializationError;

    #[test]
    fn bad_auth_url_is_rejected() {
        assert_eq!(
            Some(InitializationError::InvalidAuthUrl),
            validate_auth_url("not.a.url").err()
        );
        assert_eq!(
            Some(InitializationError::InvalidAuthUrl),
            validate_auth_url("fake").err()
        );
    }

    #[test]
    fn auth_urls_are_canonicalized() {
        assert_eq!(
            Some("https://blah.com".to_string()),
            validate_auth_url("https://blah.com").ok()
        );

        assert_eq!(
            Some("https://www.blah.com".to_string()),
            validate_auth_url("https://www.blah.com/").ok()
        );

        assert_eq!(
            Some("https://app.blah.co.uk".to_string()),
            validate_auth_url("https://app.blah.co.uk/more").ok()
        );
    }
}
