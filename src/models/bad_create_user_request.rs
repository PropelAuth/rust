/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BadCreateUserRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<Vec<String>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Vec<String>>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Vec<String>>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Vec<String>>,
}

impl BadCreateUserRequest {
    pub fn new() -> BadCreateUserRequest {
        BadCreateUserRequest {
            email: None,
            username: None,
            password: None,
            first_name: None,
            last_name: None,
        }
    }
}


