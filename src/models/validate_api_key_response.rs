use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use crate::models::UserMetadata;

pub type OrgRole = String;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidateApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user_metadata: Option<UserMetadata>,
    pub org_metadata: Option<OrgInternalMetadata>,
    pub user_role_in_org: Option<OrgRole>,
    pub user_id: Option<String>,
    pub org_id: Option<String>,
}

impl ValidateApiKeyResponse {
    pub fn new(metadata: Option<serde_json::Value>, user_metadata: Option<UserMetadata>, org_metadata: Option<OrgInternalMetadata>, user_role_in_org: Option<OrgRole>, user_id: Option<String>, org_id: Option<String>) -> Self {
        Self { metadata, user_metadata, org_metadata, user_role_in_org, user_id, org_id }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidatePersonalApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user_metadata: UserMetadata,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidateOrgApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user_metadata: Option<UserMetadata>,
    pub org_metadata: OrgInternalMetadata,
    pub user_role_in_org: Option<OrgRole>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OrgInternalMetadata {
    pub org_id: Uuid,
    pub org_name: String,
    pub org_definition: OrgDefinition,
    pub can_setup_saml: bool,
    pub autojoin_by_domain: bool,
    pub restrict_to_domain: bool,
    pub domain: Option<String>,
    //pub require_2fa_by: Option<chrono::DateTime<chrono::Utc>>,
    pub max_users: Option<i32>,
    pub metadata: MetadataHashOfValue,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OrgDefinition {
    pub roles: Vec<OrgRoleDefinition>,
    pub default_role: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OrgRoleDefinition {
    pub name: String,
    pub description: Option<String>,
    pub can_invite: bool,
    pub can_change_roles: bool,
    pub can_remove_users: bool,
    pub can_setup_saml: Option<bool>,
    pub external_permissions: Vec<String>,
    pub deprecated: bool,
    pub is_visible_to_end_user: bool,
}

pub type MetadataHashOfValue = HashMap<String, Value>;

