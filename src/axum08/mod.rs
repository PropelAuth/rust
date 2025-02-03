use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum_08::extract::FromRequestParts;
use axum_08::http::header::AUTHORIZATION;
use axum_08::http::request::Parts;
use axum_08::http::StatusCode;
use axum_08::response::IntoResponse;
use axum_08::{body::Body, http::Request, response::Response};
use tower::{Layer, Service};
use crate::apis::api_key_service_api::ValidateApiKeyParams;
use crate::propelauth::auth::PropelAuth;
use crate::propelauth::errors::{UnauthorizedError, UnauthorizedOrForbiddenError};
use crate::propelauth::token_models::{User, UserOrApiKey};

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        let auth = parts
            .extensions
            .get::<Arc<PropelAuth>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No layer found"))?;

        match auth.verify().validate_authorization_header(auth_header) {
            Ok(user) => Ok(user),
            Err(UnauthorizedError::Unauthorized(_)) => {
                Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
            }
        }
    }
}

impl<S> FromRequestParts<S> for UserOrApiKey
where
    S: Send + Sync,
{
    // If extraction fails, Axum will produce a 401 automatically.
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        let auth = parts
            .extensions
            .get::<Arc<PropelAuth>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No layer found"))?;

        let config = parts
            .extensions
            .get::<MultiAuthConfig>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No config found"))?;

        let mut multi = UserOrApiKey::new();

        // 1. Try token
        match auth.verify().validate_authorization_header(auth_header) {
            Ok(u) => {
                multi.user = Some(u);
                return Ok(multi);
            }
            Err(UnauthorizedError::Unauthorized(_)) => {
                // Fall through to the next checks
            }
        }

        // 2. If that fails, try personal key
        let maybe_api_key = auth_header
            .strip_prefix("Bearer ")
            .unwrap_or("")
            .to_owned();

        // 2. If that fails, try personal key if allowed
        if config.allow_user_key {
            match auth.api_key().validate_personal_api_key(ValidateApiKeyParams {
                api_key_token: maybe_api_key.clone(),
            }).await
            {
                Ok(pk) => {
                    multi.user_key_info = Some(pk);
                    return Ok(multi);
                }
                Err(_) => {
                    // Fall through to org key
                }
            }
        }

        // 3. Finally, try org key if allowed
        if config.allow_org_key {
            match auth.api_key().validate_org_api_key(ValidateApiKeyParams {
                api_key_token: maybe_api_key,
            }).await
            {
                Ok(ok) => {
                    multi.org_key_info = Some(ok);
                    return Ok(multi);
                }
                Err(_) => {
                    // fall through to 401
                }
            }
        }

        // If all checks fail, reject with 401
        Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}

#[derive(Clone)]
pub struct PropelAuthLayer {
    auth: Arc<PropelAuth>,
    auth_config: MultiAuthConfig,
}


impl PropelAuthLayer {
    pub fn new(auth: PropelAuth) -> PropelAuthLayer {
        PropelAuthLayer {
            auth: Arc::new(auth),
            auth_config: MultiAuthConfig::default(),
        }
    }

    pub fn new_with_config(auth: PropelAuth, auth_config: MultiAuthConfig) -> PropelAuthLayer {
        PropelAuthLayer {
            auth: Arc::new(auth),
            auth_config,
        }
    }
}

impl<S> Layer<S> for PropelAuthLayer {
    type Service = PropelAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PropelAuthMiddleware {
            inner,
            auth: self.auth.clone(),
            auth_config: self.auth_config.clone()
        }
    }
}

#[derive(Clone)]
pub struct PropelAuthMiddleware<S> {
    inner: S,
    auth: Arc<PropelAuth>,
    auth_config: MultiAuthConfig
}

impl<S> Service<Request<Body>> for PropelAuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        request.extensions_mut().insert(self.auth.clone());
        request.extensions_mut().insert(self.auth_config.clone());
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}

#[derive(Clone, Debug)]
pub struct MultiAuthConfig {
    pub allow_user_key: bool,
    pub allow_org_key: bool,
}

impl Default for MultiAuthConfig {
    fn default() -> Self {
        MultiAuthConfig {
            allow_user_key: false,
            allow_org_key: false,
        }
    }
}

impl IntoResponse for UnauthorizedError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}

impl IntoResponse for UnauthorizedOrForbiddenError {
    fn into_response(self) -> Response {
        match self {
            UnauthorizedOrForbiddenError::Unauthorized(_) => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            UnauthorizedOrForbiddenError::Forbidden(_) => {
                (StatusCode::FORBIDDEN, "Forbidden").into_response()
            }
        }
    }
}
