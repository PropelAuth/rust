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
use crate::models::validate_api_key_response::{ValidateOrgApiKeyResponse, ValidatePersonalApiKeyResponse};
use crate::propelauth::auth::PropelAuth;
use crate::propelauth::errors::{UnauthorizedError, UnauthorizedOrForbiddenError};
use crate::propelauth::token_models::User;

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

#[derive(Clone)]
pub struct PropelAuthLayer {
    auth: Arc<PropelAuth>,
}

impl PropelAuthLayer {
    pub fn new(auth: PropelAuth) -> PropelAuthLayer {
        PropelAuthLayer {
            auth: Arc::new(auth),
        }
    }
}

impl<S> Layer<S> for PropelAuthLayer {
    type Service = PropelAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PropelAuthMiddleware {
            inner,
            auth: self.auth.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PropelAuthMiddleware<S> {
    inner: S,
    auth: Arc<PropelAuth>,
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
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}

#[derive(Clone, Debug)]
pub struct MultiAuthConfig {
    pub allow_personal_key: bool,
    pub allow_org_key: bool,
}

#[derive(Clone)]
pub struct PropelAuthMultiLayer {
    auth: Arc<PropelAuth>,
    config: MultiAuthConfig,
}

impl PropelAuthMultiLayer {
    pub fn new(auth: PropelAuth, config: MultiAuthConfig) -> PropelAuthMultiLayer {
        PropelAuthMultiLayer {
            auth: Arc::new(auth),
            config,
        }
    }
}


impl<S> Layer<S> for PropelAuthMultiLayer {
    type Service = PropelAuthMultiMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PropelAuthMultiMiddleware {
            inner,
            auth: self.auth.clone(),
            config: self.config.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PropelAuthMultiMiddleware<S> {
    inner: S,
    auth: Arc<PropelAuth>,
    config: MultiAuthConfig,
}

impl<S> Service<Request<Body>> for PropelAuthMultiMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        // Extract authorization header if present
        let auth_header_opt = request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .map(|s| s.to_owned());

        // We’ll do the actual logic here so we can store user or API key info
        let auth = self.auth.clone();
        let config = self.config.clone();

        Box::pin(async move {
            // Prepare placeholders for user and/or key
            let mut user: Option<User> = None;
            let mut user_key_info: Option<ValidatePersonalApiKeyResponse> = None;
            let mut org_key_info: Option<ValidateOrgApiKeyResponse> = None;

            if let Some(auth_header) = auth_header_opt {
                // First try token
                match auth.verify().validate_authorization_header(&auth_header) {
                    Ok(returned_user) => {
                        user = Some(returned_user);
                    }
                    Err(UnauthorizedError::Unauthorized(_)) => {
                        // If configured, try personal key
                        if config.allow_personal_key {
                            let api_key = auth_header.strip_prefix("Bearer ").map(|s| s.to_string()).unwrap_or_default();
                            match auth.api_key().validate_personal_api_key(ValidateApiKeyParams { api_key_token: api_key }).await {
                                Ok(api_key_resp) => {
                                    user_key_info = Some(api_key_resp);
                                }
                                Err(_) => {
                                    // If configured, try org key
                                    if config.allow_org_key {
                                        let api_key = auth_header.strip_prefix("Bearer ").map(|s| s.to_string()).unwrap_or_default();
                                        match auth.api_key().validate_org_api_key(ValidateApiKeyParams { api_key_token: api_key }).await {
                                            Ok(org_key_resp) => {
                                                org_key_info = Some(org_key_resp);
                                            }
                                            Err(_) => {
                                                // All attempts failed, return 401
                                                return Ok(
                                                    (StatusCode::UNAUTHORIZED, "Unauthorized")
                                                        .into_response(),
                                                );
                                            }
                                        }
                                    } else {
                                        // Org key not allowed, so fail
                                        return Ok((StatusCode::UNAUTHORIZED, "Unauthorized")
                                            .into_response());
                                    }
                                }
                            }
                        } else if config.allow_org_key {
                            // If personal key not allowed but org key is
                            let api_key = auth_header.strip_prefix("Bearer ").map(|s| s.to_string()).unwrap_or_default();
                            match auth.api_key().validate_org_api_key(ValidateApiKeyParams { api_key_token: api_key }).await {
                                Ok(org_key_resp) => {
                                    org_key_info = Some(org_key_resp);
                                }
                                Err(_) => {
                                    return Ok(
                                        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
                                    );
                                }
                            }
                        } else {
                            // No key fallback is allowed
                            return Ok((StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
                        }
                    }
                }
            }

            // Store them in the request’s extensions so downstream can pick them up
            request
                .extensions_mut()
                .insert::<Arc<PropelAuth>>(auth.clone());
            request.extensions_mut().insert(user);
            request.extensions_mut().insert(user_key_info);
            request.extensions_mut().insert(org_key_info);

            // Continue
            let response = self.inner.call(request).await?;
            Ok(response)
        })
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
