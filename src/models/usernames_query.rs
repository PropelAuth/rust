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
pub struct UsernamesQuery {
    #[serde(rename = "usernames")]
    pub usernames: Vec<String>,
}

impl UsernamesQuery {
    pub fn new(usernames: Vec<String>) -> UsernamesQuery {
        UsernamesQuery {
            usernames,
        }
    }
}


