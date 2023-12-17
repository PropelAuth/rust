use crate::models::{UserInOrg, UserMetadata};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub type OrgRole = String;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidateApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user: Option<UserMetadata>,
    pub org: Option<OrgMetadata>,
    pub user_in_org: Option<UserInOrg>,
    pub user_id: Option<String>,
    pub org_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidatePersonalApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user: UserMetadata,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidateOrgApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user: Option<UserMetadata>,
    pub org: OrgMetadata,
    pub user_in_org: Option<UserInOrg>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OrgMetadata {
    pub org_id: Uuid,
    pub org_name: String,
    pub can_setup_saml: bool,
    pub max_users: Option<i32>,
    pub metadata: MetadataHashOfValue,
}

pub type MetadataHashOfValue = HashMap<String, Value>;
