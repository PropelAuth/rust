use crate::apis::access_token_service_api::{CreateAccessTokenParams, CreateAccessTokenV2Params};
use crate::apis::configuration::Configuration;

use crate::models::CreateAccessTokenResponse;
use crate::propelauth::errors::CreateAccessTokenError;
use crate::propelauth::helpers::{is_valid_id, map_autogenerated_error};

pub struct AccessTokenService<'a> {
    pub(crate) config: &'a Configuration,
}

impl AccessTokenService<'_> {
    pub async fn create_access_token(
        &self,
        params: CreateAccessTokenParams,
    ) -> Result<CreateAccessTokenResponse, CreateAccessTokenError> {
        if !is_valid_id(&params.create_access_token_request.user_id) {
            return Err(CreateAccessTokenError::NotFound);
        }

        crate::apis::access_token_service_api::create_access_token(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    CreateAccessTokenError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::access_token_service_api::CreateAccessTokenError::Status400(
                                bad_request,
                            )),
                        ) => CreateAccessTokenError::BadRequest(bad_request),
                        (401, _) => CreateAccessTokenError::InvalidApiKey,
                        (429, _) => CreateAccessTokenError::PropelAuthRateLimit,
                        (404, _) => CreateAccessTokenError::NotFound,
                        _ => CreateAccessTokenError::UnexpectedException,
                    },
                )
            })
    }

    pub async fn create_access_token_v2(
        &self,
        params: CreateAccessTokenV2Params,
    ) -> Result<CreateAccessTokenResponse, CreateAccessTokenError> {
        if !is_valid_id(&params.create_access_token_request.user_id) {
            return Err(CreateAccessTokenError::NotFound);
        }

        crate::apis::access_token_service_api::create_access_token_v2(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    CreateAccessTokenError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::access_token_service_api::CreateAccessTokenError::Status400(
                                bad_request,
                            )),
                        ) => CreateAccessTokenError::BadRequest(bad_request),
                        (401, _) => CreateAccessTokenError::InvalidApiKey,
                        (429, _) => CreateAccessTokenError::PropelAuthRateLimit,
                        (404, _) => CreateAccessTokenError::NotFound,
                        _ => CreateAccessTokenError::UnexpectedException,
                    },
                )
            })
    }
}
