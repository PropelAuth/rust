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
pub struct EmailsQuery {
    #[serde(rename = "emails")]
    pub emails: Vec<String>,
}

impl EmailsQuery {
    pub fn new(emails: Vec<String>) -> EmailsQuery {
        EmailsQuery {
            emails,
        }
    }
}


