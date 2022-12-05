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
pub struct UpdateMetadataRequest {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "picture_url", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

impl UpdateMetadataRequest {
    pub fn new() -> UpdateMetadataRequest {
        UpdateMetadataRequest {
            username: None,
            first_name: None,
            last_name: None,
            picture_url: None,
        }
    }
}


