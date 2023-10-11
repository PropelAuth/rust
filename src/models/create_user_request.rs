/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "email_confirmed")]
    pub email_confirmed: bool,
    #[serde(rename = "send_email_to_confirm_email_address")]
    pub send_email_to_confirm_email_address: bool,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,
}

impl CreateUserRequest {
    pub fn new(
        email: String,
        email_confirmed: bool,
        send_email_to_confirm_email_address: bool,
    ) -> CreateUserRequest {
        CreateUserRequest {
            email,
            email_confirmed,
            send_email_to_confirm_email_address,
            password: None,
            username: None,
            first_name: None,
            last_name: None,
            properties: None,
        }
    }
}
