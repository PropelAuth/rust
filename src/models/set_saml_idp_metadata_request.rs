#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSamlIdpMetadataRequest {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "idp_entity_id")]
    pub idp_entity_id: String,
    #[serde(rename = "idp_sso_url")]
    pub idp_sso_url: String,
    #[serde(rename = "idp_certificate")]
    pub idp_certificate: String,
}
