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
pub struct CreateOrgRequest {
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateOrgRequest {
    pub fn new(name: String) -> CreateOrgRequest {
        CreateOrgRequest {
            name,
        }
    }
}


