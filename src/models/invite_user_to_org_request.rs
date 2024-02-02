#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteUserToOrgRequest {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: String,
}
