use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use crate::models::{UserInOrg, UserMetadata};

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

impl ValidateApiKeyResponse {
    pub fn new(metadata: Option<serde_json::Value>, user: Option<UserMetadata>, org: Option<OrgMetadata>, user_in_org: Option<UserInOrg>, user_id: Option<String>, org_id: Option<String>) -> Self {
        Self { metadata, user, org, user_in_org, user_id, org_id }
    }
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

