use uuid::Uuid;

// total_invites: i64,
// current_page: i64,
// page_size: i64,
// has_more_results: bool,
// invites: Vec<InviteToOrgResponse>,

#[derive(Deserialize, Debug)]
pub struct FetchPendingInvitesResponse {
    #[serde(rename = "total_invites")]
    pub total_invites: i64,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    #[serde(rename = "page_size")]
    pub page_size: i64,
    #[serde(rename = "has_more_results")]
    pub has_more_results: bool,
    #[serde(rename = "invites", default)]
    pub invites: Vec<PendingInviteResponse>,
}

#[derive(Deserialize, Debug)]
pub struct PendingInviteResponse {
    #[serde(rename = "invitee_email")]
    pub invitee_email: String,
    #[serde(rename = "org_id")]
    pub org_id: Uuid,
    #[serde(rename = "org_name")]
    pub org_name: String,
    #[serde(rename = "role_in_org")]
    pub role_in_org: String,
    #[serde(rename = "additional_roles_in_org", default)]
    pub additional_roles_in_org: Vec<String>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "expires_at")]
    pub expires_at: i64,
    #[serde(rename = "inviter_email")]
    pub inviter_email: Option<String>,
    #[serde(rename = "inviter_user_id")]
    pub inviter_user_id: Option<Uuid>,
}
