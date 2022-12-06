use crate::propelauth::auth::PropelAuth;
use crate::propelauth::errors::{UnauthorizedError, UnauthorizedOrForbiddenError};
use crate::propelauth::token_models::User;
use actix_web::dev::Payload;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, ResponseError};
use std::future::{ready, Ready};

impl FromRequest for User {
    type Error = actix_web::error::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header_opt = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let auth_header = match auth_header_opt {
            Some(auth_header) => auth_header,
            None => return ready(Err(ErrorUnauthorized("Unauthorized"))),
        };

        let auth_opt = req.app_data::<web::Data<PropelAuth>>();
        let auth = match auth_opt {
            Some(auth) => auth,
            None => return ready(Err(ErrorInternalServerError("No app_data found"))),
        };

        ready(
            match auth.verify().validate_authorization_header(auth_header) {
                Ok(user) => Ok(user),
                Err(UnauthorizedError::Unauthorized(_)) => Err(ErrorUnauthorized("Unauthorized")),
            },
        )
    }
}

impl ResponseError for UnauthorizedError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UnauthorizedError::Unauthorized(_) => HttpResponse::Unauthorized().body("Unauthorized"),
        }
    }
}

impl ResponseError for UnauthorizedOrForbiddenError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UnauthorizedOrForbiddenError::Unauthorized(_) => {
                HttpResponse::Unauthorized().body("Unauthorized")
            }
            UnauthorizedOrForbiddenError::Forbidden(_) => {
                HttpResponse::Forbidden().body("Forbidden")
            }
        }
    }
}
