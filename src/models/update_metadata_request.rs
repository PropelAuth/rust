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
pub struct UpdateMetadataRequest {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "picture_url", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,
}

impl UpdateMetadataRequest {
    pub fn new() -> UpdateMetadataRequest {
        UpdateMetadataRequest {
            username: None,
            first_name: None,
            last_name: None,
            picture_url: None,
            metadata: None,
            properties: None,
        }
    }
}


