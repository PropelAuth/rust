#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BadInviteUserToOrgRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
}

impl BadInviteUserToOrgRequest {
    pub fn new() -> BadInviteUserToOrgRequest {
        BadInviteUserToOrgRequest {
            email: None,
        }
    }
}
