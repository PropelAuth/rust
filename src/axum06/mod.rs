use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum_06::async_trait;
use axum_06::extract::FromRequestParts;
use axum_06::http::header::AUTHORIZATION;
use axum_06::http::request::Parts;
use axum_06::http::StatusCode;
use axum_06::response::IntoResponse;
use axum_06::{body::Body, http::Request, response::Response};
use tower::{Layer, Service};

use crate::propelauth::auth::PropelAuth;
use crate::propelauth::errors::{UnauthorizedError, UnauthorizedOrForbiddenError};
use crate::propelauth::token_models::User;

#[async_trait]
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
