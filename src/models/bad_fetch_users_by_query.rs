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
pub struct BadFetchUsersByQuery {
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Vec<String>>,
    #[serde(rename = "page_number", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<Vec<String>>,
    #[serde(rename = "email_or_username", skip_serializing_if = "Option::is_none")]
    pub email_or_username: Option<Vec<String>>,
}

impl BadFetchUsersByQuery {
    pub fn new() -> BadFetchUsersByQuery {
        BadFetchUsersByQuery {
            page_size: None,
            page_number: None,
            email_or_username: None,
        }
    }
}


