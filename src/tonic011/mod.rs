use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper_014::Body;
use tonic_011::body::BoxBody;
use tonic_011::codegen::http::header::AUTHORIZATION;
use tonic_011::server::NamedService;
use tonic_011::Status;
use tower::{Layer, Service};

use crate::propelauth::auth::PropelAuth;

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

impl<S: NamedService> NamedService for PropelAuthMiddleware<S> {
    const NAME: &'static str = S::NAME;
}

impl<S> Service<hyper_014::Request<Body>> for PropelAuthMiddleware<S>
where
    S: Service<hyper_014::Request<Body>, Response = hyper_014::Response<BoxBody>>
        + Send
        + Clone
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: hyper_014::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let auth = self.auth.clone();
        request.extensions_mut().insert(self.auth.clone());

        Box::pin(async move {
            match request
                .headers()
                .get(AUTHORIZATION)
                .and_then(|header| header.to_str().ok())
            {
                None => Ok(Status::unauthenticated("Unauthenticated").to_http()),
                Some(auth_header) => {
                    match auth.verify().validate_authorization_header(auth_header) {
                        Ok(user) => {
                            // Insert an extension, which can be inspected by the service.
                            request.extensions_mut().insert(user);
                            let response = inner.call(request).await?;
                            Ok(response)
                        }
                        Err(_) => Ok(Status::unauthenticated("Unauthenticated").to_http()),
                    }
                }
            }
        })
    }
}
