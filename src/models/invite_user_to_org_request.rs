#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteUserToOrgRequest {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "additional_roles")]
    pub additional_roles: Vec<String>,
}

impl InviteUserToOrgRequest {
    pub fn new(org_id: String, email: String, role: String) -> InviteUserToOrgRequest {
        InviteUserToOrgRequest {
            org_id,
            email,
            role,
            additional_roles: vec![],
        }
    }

    pub fn with_multiple_roles(
        org_id: String,
        email: String,
        mut roles: Vec<String>,
    ) -> Option<InviteUserToOrgRequest> {
        let role = roles.pop()?;

        Some(InviteUserToOrgRequest {
            org_id,
            email,
            role,
            additional_roles: roles,
        })
    }
}
