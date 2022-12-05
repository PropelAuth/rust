/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.3
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FetchUsersOrderBy {
    #[serde(rename = "CREATED_AT_ASC")]
    CreatedAtAsc,
    #[serde(rename = "CREATED_AT_DESC")]
    CreatedAtDesc,
    #[serde(rename = "LAST_ACTIVE_AT_ASC")]
    LastActiveAtAsc,
    #[serde(rename = "LAST_ACTIVE_AT_DESC")]
    LastActiveAtDesc,
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "USERNAME")]
    Username,

}

impl ToString for FetchUsersOrderBy {
    fn to_string(&self) -> String {
        match self {
            Self::CreatedAtAsc => String::from("CREATED_AT_ASC"),
            Self::CreatedAtDesc => String::from("CREATED_AT_DESC"),
            Self::LastActiveAtAsc => String::from("LAST_ACTIVE_AT_ASC"),
            Self::LastActiveAtDesc => String::from("LAST_ACTIVE_AT_DESC"),
            Self::Email => String::from("EMAIL"),
            Self::Username => String::from("USERNAME"),
        }
    }
}

impl Default for FetchUsersOrderBy {
    fn default() -> FetchUsersOrderBy {
        Self::CreatedAtAsc
    }
}




