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
pub struct UserPagedResponse {
    #[serde(rename = "users")]
    pub users: Vec<crate::models::UserMetadata>,
    #[serde(rename = "total_users")]
    pub total_users: i64,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    #[serde(rename = "page_size")]
    pub page_size: i64,
    #[serde(rename = "has_more_results")]
    pub has_more_results: bool,
}

impl UserPagedResponse {
    pub fn new(users: Vec<crate::models::UserMetadata>, total_users: i64, current_page: i64, page_size: i64, has_more_results: bool) -> UserPagedResponse {
        UserPagedResponse {
            users,
            total_users,
            current_page,
            page_size,
            has_more_results,
        }
    }
}


