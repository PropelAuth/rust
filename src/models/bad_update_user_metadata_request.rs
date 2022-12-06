/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BadUpdateUserMetadataRequest {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<Vec<String>>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Vec<String>>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Vec<String>>,
    #[serde(rename = "picture_url", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<Vec<String>>,
}

impl BadUpdateUserMetadataRequest {
    pub fn new() -> BadUpdateUserMetadataRequest {
        BadUpdateUserMetadataRequest {
            username: None,
            first_name: None,
            last_name: None,
            picture_url: None,
        }
    }
}


