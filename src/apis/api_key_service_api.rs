/*
 * propelauth
 *
 * For consistency with the rest of this codebase, this file matches the output of the terrible openapi generator.
 */

use hex;
use reqwest;

use crate::apis::ResponseContent;

use super::{configuration, Error};

/// struct for passing parameters to the method [`fetch_api_keys`, `fetch_archived_api_keys`]
#[derive(Clone, Debug, Default, Serialize)]
pub struct ApiKeyQueryParams {
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub org_id: Option<String>,
    pub page_size: Option<i64>,
    pub page_number: Option<i64>,
}

/// struct for passing parameters to the method [`create_api_key`]
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateApiKeyParams {
    pub expires_at_seconds: Option<i64>,
    pub metadata: Option<serde_json::Value>,
    pub user_id: Option<String>,
    pub org_id: Option<String>,
}

/// struct for passing parameters to the method [`update_api_key`]
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateApiKeyParams {
    pub expires_at_seconds: Option<i64>,
    pub metadata: Option<serde_json::Value>,
}

/// struct for passing parameters to the method [`validate_api_key`]
#[derive(Clone, Debug, Default, Serialize)]
pub struct ValidateApiKeyParams {
    pub api_key_token: String,
}

// struct for typed errors on the api keys service
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ApiKeyError {
    BadRequest(crate::models::BadUpdateOrgRequest),
    InvalidIntegrationAPIKey,
    InvalidAPIKey {
        message: String,
    },
    RateLimited {
        wait_seconds: f64,
        user_facing_error: String,
    },
    InvalidPersonalAPIKey,
    InvalidOrgAPIKey,
    NotFound,
    UnknownValue(serde_json::Value),
    UnknownError,
    UnexpectedExceptionWithSDK,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ApiKeyValidationErrorResponse {
    InvalidEndUserApiKey {
        api_key_id: String,
    },
    EndUserApiKeyRateLimited {
        wait_seconds: f64,
        error_code: String,
        user_facing_error: String,
    },
}

pub async fn fetch_current_api_keys(
    configuration: &configuration::Configuration,
    params: ApiKeyQueryParams,
) -> Result<crate::models::FetchApiKeysPagedResponse, Error<ApiKeyError>> {
    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys",
        configuration.base_path
    );
    let mut req_builder = client.request(reqwest::Method::GET, uri.as_str());

    // assemble the query parameters
    if let Some(ref user_id) = params.user_id {
        req_builder = req_builder.query(&[("user_id", user_id)]);
    }
    if let Some(ref user_email) = params.user_email {
        req_builder = req_builder.query(&[("user_email", user_email)]);
    }
    if let Some(ref org_id) = params.org_id {
        req_builder = req_builder.query(&[("org_id", org_id)]);
    }
    if let Some(ref page_size) = params.page_size {
        req_builder = req_builder.query(&[("page_size", page_size)]);
    }
    if let Some(ref page_number) = params.page_number {
        req_builder = req_builder.query(&[("page_number", page_number)]);
    }

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ApiKeyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn fetch_archived_api_keys(
    configuration: &configuration::Configuration,
    params: ApiKeyQueryParams,
) -> Result<crate::models::FetchApiKeysPagedResponse, Error<ApiKeyError>> {
    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys/archived",
        configuration.base_path
    );
    let mut req_builder = client.request(reqwest::Method::GET, uri.as_str());

    // assemble the query parameters
    if let Some(ref user_id) = params.user_id {
        req_builder = req_builder.query(&[("user_id", user_id)]);
    }
    if let Some(ref user_email) = params.user_email {
        req_builder = req_builder.query(&[("user_email", user_email)]);
    }
    if let Some(ref org_id) = params.org_id {
        req_builder = req_builder.query(&[("org_id", org_id)]);
    }
    if let Some(ref page_size) = params.page_size {
        req_builder = req_builder.query(&[("page_size", page_size)]);
    }
    if let Some(ref page_number) = params.page_number {
        req_builder = req_builder.query(&[("page_number", page_number)]);
    }

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ApiKeyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn fetch_api_key(
    configuration: &configuration::Configuration,
    api_key_id: String,
) -> Result<crate::models::FetchApiKeyResponse, Error<ApiKeyError>> {
    if hex::decode(&api_key_id).is_err() {
        return Err(Error::Params("Invalid API key ID format".to_string()));
    }

    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys/{}",
        configuration.base_path, api_key_id
    );
    let mut req_builder = client.request(reqwest::Method::GET, uri.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ApiKeyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn create_api_key(
    configuration: &configuration::Configuration,
    params: CreateApiKeyParams,
) -> Result<crate::models::CreateApiKeyResponse, Error<ApiKeyError>> {
    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys",
        configuration.base_path
    );
    let mut req_builder = client.request(reqwest::Method::POST, uri.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    req_builder = req_builder.json(&params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ApiKeyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn update_api_key(
    configuration: &configuration::Configuration,
    api_key_id: String,
    params: UpdateApiKeyParams,
) -> Result<crate::models::SuccessfulResponse, Error<ApiKeyError>> {
    if hex::decode(&api_key_id).is_err() {
        return Err(Error::Params("Invalid API key ID format".to_string()));
    }

    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys/{}",
        configuration.base_path, api_key_id
    );
    let mut req_builder = client.request(reqwest::Method::PATCH, uri.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    req_builder = req_builder.json(&params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ApiKeyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn delete_api_key(
    configuration: &configuration::Configuration,
    api_key_id: String,
) -> Result<crate::models::SuccessfulResponse, Error<ApiKeyError>> {
    if hex::decode(&api_key_id).is_err() {
        return Err(Error::Params("Invalid API key ID format".to_string()));
    }

    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys/{}",
        configuration.base_path, api_key_id
    );
    let mut req_builder = client.request(reqwest::Method::DELETE, uri.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        Ok(crate::models::successful_response::SuccessfulResponse { message: None })
    } else {
        let entity: Option<ApiKeyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn validate_api_key(
    configuration: &configuration::Configuration,
    params: ValidateApiKeyParams,
) -> Result<crate::models::ValidateApiKeyResponse, Error<ApiKeyValidationErrorResponse>> {
    if hex::decode(&params.api_key_token).is_err() {
        return Err(Error::Params("Invalid API key ID format".to_string()));
    }

    let client = &configuration.client;

    let uri = format!(
        "{}/api/backend/v1/end_user_api_keys/validate",
        configuration.base_path
    );
    let mut req_builder = client.request(reqwest::Method::POST, uri.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref bearer_token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(bearer_token.to_owned());
    }

    req_builder = req_builder.json(&params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ApiKeyValidationErrorResponse> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}
