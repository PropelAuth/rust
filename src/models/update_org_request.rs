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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateOrgRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "can_setup_saml", skip_serializing_if = "Option::is_none")]
    pub can_setup_saml: Option<bool>,
    #[serde(rename = "max_users", skip_serializing_if = "Option::is_none")]
    pub max_users: Option<i32>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

impl UpdateOrgRequest {
    pub fn new() -> UpdateOrgRequest {
        UpdateOrgRequest {
            name: None,
            can_setup_saml: None,
            max_users: None,
            metadata: None,
        }
    }
}
