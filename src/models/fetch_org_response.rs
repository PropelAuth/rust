/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgMetadata {
    #[serde(flatten)]
    metadata: HashMap<String, Value>,
}

impl OrgMetadata {
    pub fn is_empty(&self) -> bool {
        self.metadata.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchOrgResponse {
    pub org_id: String,
    pub name: String,
    pub is_saml_configured: bool,
    #[serde(default, skip_serializing_if = "OrgMetadata::is_empty")]
    pub metadata: OrgMetadata,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_users: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_role_mapping_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_org_id: Option<String>,
}

impl FetchOrgResponse {
    pub fn new(
        org_id: String,
        name: String,
        metadata: OrgMetadata,
        is_saml_configured: bool,
    ) -> FetchOrgResponse {
        FetchOrgResponse {
            org_id,
            name,
            metadata,
            is_saml_configured,
            max_users: None,
            custom_role_mapping_name: None,
            legacy_org_id: None,
        }
    }
}
