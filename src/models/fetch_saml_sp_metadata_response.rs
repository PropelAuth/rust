#[derive(Clone, Debug, PartialEq, Default, Deserialize)]
pub struct FetchSamlSpMetadataResponse {
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    #[serde(rename = "acs_url")]
    pub acs_url: String,
    #[serde(rename = "logout_url")]
    pub logout_url: String,
}
