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
pub struct CreateMagicLinkRequest {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "redirect_to_url", skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<String>,
    #[serde(rename = "expires_in_hours", skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i64>,
    #[serde(rename = "create_new_user_if_one_doesnt_exist", skip_serializing_if = "Option::is_none")]
    pub create_new_user_if_one_doesnt_exist: Option<bool>,
}

impl CreateMagicLinkRequest {
    pub fn new(email: String) -> CreateMagicLinkRequest {
        CreateMagicLinkRequest {
            email,
            redirect_to_url: None,
            expires_in_hours: None,
            create_new_user_if_one_doesnt_exist: None,
        }
    }
}


