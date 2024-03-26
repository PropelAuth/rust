use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`create_access_token`]
#[derive(Clone, Debug, Default)]
pub struct CreateAccessTokenParams {
    pub create_access_token_request: crate::models::CreateAccessTokenRequest,
}

#[derive(Clone, Debug, Default)]
pub struct CreateAccessTokenV2Params {
    pub create_access_token_request: crate::models::CreateAccessTokenV2Request,
}

/// struct for typed errors of method [`create_access_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAccessTokenError {
    Status400(crate::models::BadCreateAccessTokenError),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

pub async fn create_access_token(
    configuration: &configuration::Configuration,
    params: CreateAccessTokenParams,
) -> Result<crate::models::CreateAccessTokenResponse, Error<CreateAccessTokenError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_access_token_request = params.create_access_token_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/access_token",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_access_token_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAccessTokenError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error: ResponseContent<CreateAccessTokenError> = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_access_token_v2(
    configuration: &configuration::Configuration,
    params: CreateAccessTokenV2Params,
) -> Result<crate::models::CreateAccessTokenResponse, Error<CreateAccessTokenError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_access_token_request = params.create_access_token_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/access_token",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_access_token_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAccessTokenError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error: ResponseContent<CreateAccessTokenError> = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
