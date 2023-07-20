#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteUserToOrgRequest {
    pub email: String,
    pub org_id: String,
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
