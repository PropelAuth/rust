#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSamlConnectionLinkResponse {
    #[serde(rename = "url")]
    pub url: String,
}

impl CreateSamlConnectionLinkResponse {
    pub fn new(url: String) -> CreateSamlConnectionLinkResponse {
        CreateSamlConnectionLinkResponse { url }
    }
}
