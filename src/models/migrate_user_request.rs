/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MigrateUserRequest {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "email_confirmed")]
    pub email_confirmed: bool,
    #[serde(rename = "existing_user_id", skip_serializing_if = "Option::is_none")]
    pub existing_user_id: Option<String>,
    #[serde(rename = "existing_password_hash", skip_serializing_if = "Option::is_none")]
    pub existing_password_hash: Option<String>,
    #[serde(rename = "existing_mfa_base32_encoded_secret", skip_serializing_if = "Option::is_none")]
    pub existing_mfa_base32_encoded_secret: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl MigrateUserRequest {
    pub fn new(email: String, email_confirmed: bool) -> MigrateUserRequest {
        MigrateUserRequest {
            email,
            email_confirmed,
            existing_user_id: None,
            existing_password_hash: None,
            existing_mfa_base32_encoded_secret: None,
            enabled: None,
            username: None,
            first_name: None,
            last_name: None,
        }
    }
}


