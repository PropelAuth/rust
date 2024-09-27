/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;
use crate::models::FetchOrgOrderBy;

/// struct for passing parameters to the method [`add_user_to_org`]
#[derive(Clone, Debug, Default)]
pub struct AddUserToOrgParams {
    pub add_user_to_org_request: crate::models::AddUserToOrgRequest,
}

/// struct for passing parameters to the method [`allow_org_to_enable_saml`]
#[derive(Clone, Debug, Default)]
pub struct AllowOrgToEnableSamlParams {
    pub org_id: String,
}

/// struct for passing parameters to the method [`change_user_role_in_org`]
#[derive(Clone, Debug, Default)]
pub struct ChangeUserRoleInOrgParams {
    pub change_user_role_in_org_request: crate::models::ChangeUserRoleInOrgRequest,
}

/// struct for passing parameters to the method [`create_org`]
#[derive(Clone, Debug, Default)]
pub struct CreateOrgParams {
    pub create_org_request: crate::models::CreateOrgRequest,
}

/// struct for passing parameters to the method [`disallow_saml`]
#[derive(Clone, Debug, Default)]
pub struct DisallowSamlParams {
    pub org_id: String,
}

/// struct for passing parameters to the method [`fetch_org`]
#[derive(Clone, Debug, Default)]
pub struct FetchOrgParams {
    pub org_id: String,
}

/// struct for passing parameters to the method [`fetch_orgs_by_query`]
#[derive(Clone, Debug, Default)]
pub struct FetchOrgsByQueryParams {
    pub page_size: Option<i64>,
    pub page_number: Option<i64>,
    pub order_by: Option<FetchOrgOrderBy>,
    pub name: Option<String>,
    pub legacy_org_id: Option<String>,
}

/// struct for passing parameters to the method [`fetch_pending_invites`]
#[derive(Clone, Debug, Default)]
pub struct FetchPendingInvitesParams {
    pub page_size: Option<i64>,
    pub page_number: Option<i64>,
    pub org_id: Option<String>,
}

/// struct for passing parameters to the method [`fetch_users_in_org`]
#[derive(Clone, Debug, Default)]
pub struct FetchUsersInOrgParams {
    pub org_id: String,
    pub page_size: Option<i64>,
    pub page_number: Option<i64>,
    /// Defaults to false
    pub include_orgs: Option<bool>,
    pub role: Option<String>,
}

/// struct for passing parameters to the method [`remove_user_from_org`]
#[derive(Clone, Debug, Default)]
pub struct RemoveUserFromOrgParams {
    pub remove_user_from_org_request: crate::models::RemoveUserFromOrgRequest,
}

/// struct for passing parameters to the method [`update_org`]
#[derive(Clone, Debug, Default)]
pub struct UpdateOrgParams {
    pub org_id: String,
    pub update_org_request: crate::models::UpdateOrgRequest,
}

/// struct for passing parameters to the method [`subscribe_org_to_role_mapping`]
#[derive(Clone, Debug, Default)]
pub struct SubscribeOrgToRoleMappingParams {
    pub org_id: String,
    pub update_org_request: crate::models::SubscribeOrgToRoleMappingRequest,
}

/// struct for passing parameters to the method [`delete_org`]
#[derive(Clone, Debug, Default)]
pub struct DeleteOrgParams {
    pub org_id: String,
}

/// struct for passing parameters to the method [`revoke_pending_org_invite`]
#[derive(Clone, Debug, Default)]
pub struct RevokePendingOrgInviteParams {
    pub revoke_pending_org_invite_request: crate::models::RevokePendingOrgInviteRequest,
}

/// struct for typed errors of method [`add_user_to_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUserToOrgError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allow_org_to_enable_saml`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllowOrgToEnableSamlError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_user_role_in_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeUserRoleInOrgError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrgError {
    Status400(crate::models::BadCreateOrgRequest),
    Status401(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disallow_saml`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisallowSamlError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchOrgError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_custom_role_mappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchCustomRoleMappingsError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_pending_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchPendingInvitesError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`revoke_pending_org_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevokePendingOrgInviteError {
    Status401(serde_json::Value),
    Status400(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_orgs_by_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchOrgsByQueryError {
    Status400(crate::models::BadFetchOrgQuery),
    Status401(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_users_in_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchUsersInOrgError {
    Status400(crate::models::BadFetchUsersInOrgQuery),
    Status401(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_user_from_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserFromOrgError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrgError {
    Status400(crate::models::BadUpdateOrgRequest),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubscribeOrgToRoleMappingError {
    Status400(crate::models::BadUpdateOrgRequest),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrgError {
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

pub async fn add_user_to_org(
    configuration: &configuration::Configuration,
    params: AddUserToOrgParams,
) -> Result<crate::models::SuccessfulResponse, Error<AddUserToOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let add_user_to_org_request = params.add_user_to_org_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/add_user",
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
    local_var_req_builder = local_var_req_builder.json(&add_user_to_org_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddUserToOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allow_org_to_enable_saml(
    configuration: &configuration::Configuration,
    params: AllowOrgToEnableSamlParams,
) -> Result<crate::models::SuccessfulResponse, Error<AllowOrgToEnableSamlError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/{org_id}/allow_saml",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AllowOrgToEnableSamlError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn change_user_role_in_org(
    configuration: &configuration::Configuration,
    params: ChangeUserRoleInOrgParams,
) -> Result<crate::models::SuccessfulResponse, Error<ChangeUserRoleInOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let change_user_role_in_org_request = params.change_user_role_in_org_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/change_role",
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
    local_var_req_builder = local_var_req_builder.json(&change_user_role_in_org_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeUserRoleInOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_org(
    configuration: &configuration::Configuration,
    params: CreateOrgParams,
) -> Result<crate::models::CreateOrgResponse, Error<CreateOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_org_request = params.create_org_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/backend/v1/org/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_org_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn disallow_saml(
    configuration: &configuration::Configuration,
    params: DisallowSamlParams,
) -> Result<crate::models::SuccessfulResponse, Error<DisallowSamlError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/{org_id}/disallow_saml",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DisallowSamlError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_org(
    configuration: &configuration::Configuration,
    params: FetchOrgParams,
) -> Result<crate::models::FetchOrgResponse, Error<FetchOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/{org_id}",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchOrgError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_custom_role_mappings(
    configuration: &configuration::Configuration,
) -> Result<crate::models::FetchCustomRoleMappingsResponse, Error<FetchCustomRoleMappingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/custom_role_mappings",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchCustomRoleMappingsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_pending_invites(
    configuration: &configuration::Configuration,
    params: FetchPendingInvitesParams,
) -> Result<crate::models::FetchPendingInvitesResponse, Error<FetchPendingInvitesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let FetchPendingInvitesParams {
        page_size,
        page_number,
        org_id,
    } = params;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/pending_org_invites",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_number", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = org_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("org_id", &local_var_str.to_string())]);
    }

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchPendingInvitesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn revoke_pending_org_invite(
    configuration: &configuration::Configuration,
    params: RevokePendingOrgInviteParams,
) -> Result<crate::models::SuccessfulResponse, Error<RevokePendingOrgInviteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let revoke_pending_org_invite_request = params.revoke_pending_org_invite_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/pending_org_invites",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&revoke_pending_org_invite_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(|e| Error::from(e))
    } else {
        let local_var_entity: Option<RevokePendingOrgInviteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_orgs_by_query(
    configuration: &configuration::Configuration,
    params: FetchOrgsByQueryParams,
) -> Result<crate::models::FetchOrgsResponse, Error<FetchOrgsByQueryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let FetchOrgsByQueryParams {
        page_size,
        page_number,
        order_by,
        name,
        legacy_org_id,
    } = params;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/query",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_number", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("order_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = legacy_org_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("legacy_org_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchOrgsByQueryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_users_in_org(
    configuration: &configuration::Configuration,
    params: FetchUsersInOrgParams,
) -> Result<crate::models::UserPagedResponse, Error<FetchUsersInOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let FetchUsersInOrgParams {
        org_id,
        page_size,
        page_number,
        include_orgs,
        role,
    } = params;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/user/org/{org_id}",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_number", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_orgs {
        local_var_req_builder =
            local_var_req_builder.query(&[("include_orgs", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = role {
        local_var_req_builder =
            local_var_req_builder.query(&[("role", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchUsersInOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn remove_user_from_org(
    configuration: &configuration::Configuration,
    params: RemoveUserFromOrgParams,
) -> Result<crate::models::SuccessfulResponse, Error<RemoveUserFromOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let remove_user_from_org_request = params.remove_user_from_org_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/remove_user",
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
    local_var_req_builder = local_var_req_builder.json(&remove_user_from_org_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RemoveUserFromOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_org(
    configuration: &configuration::Configuration,
    params: UpdateOrgParams,
) -> Result<crate::models::SuccessfulResponse, Error<UpdateOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;
    let update_org_request = params.update_org_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/{org_id}",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_org_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn subscribe_org_to_role_mapping(
    configuration: &configuration::Configuration,
    params: SubscribeOrgToRoleMappingParams,
) -> Result<crate::models::SuccessfulResponse, Error<SubscribeOrgToRoleMappingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;
    let update_org_request = params.update_org_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/{org_id}",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_org_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubscribeOrgToRoleMappingError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_org(
    configuration: &configuration::Configuration,
    params: DeleteOrgParams,
) -> Result<crate::models::SuccessfulResponse, Error<DeleteOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let org_id = params.org_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/backend/v1/org/{org_id}",
        local_var_configuration.base_path,
        org_id = crate::apis::urlencode(org_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(|e| Error::from(e))
    } else {
        let local_var_entity: Option<DeleteOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
