#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOrgSamlConnectionLinkResponse {
    #[serde(rename = "url")]
    pub url: String,
}

impl CreateOrgSamlConnectionLinkResponse {
    pub fn new(url: String) -> CreateOrgSamlConnectionLinkResponse {
        CreateOrgSamlConnectionLinkResponse {
            url,
        }
    }
}