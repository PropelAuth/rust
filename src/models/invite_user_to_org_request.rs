#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteUserToOrgRequest {
    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "org_id")]
    pub org_id: String,

    #[serde(rename = "role")]
    pub role: String,
}

impl InviteUserToOrgRequest {
    pub fn new(email: String, org_id: String, role: String) -> InviteUserToOrgRequest {
        InviteUserToOrgRequest {
            email,
            org_id,
            role,
        }
    }
}
