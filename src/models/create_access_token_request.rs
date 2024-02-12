#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateAccessTokenRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "duration_in_minutes")]
    pub duration_in_minutes: u64,
    #[serde(rename = "active_org_id", skip_serializing_if = "Option::is_none")]
    pub active_org_id: Option<String>,
    #[serde(rename = "with_active_org_support")]
    pub with_active_org_support: bool,
}

impl CreateAccessTokenRequest {
    pub fn new(
        user_id: String,
        duration_in_minutes: u64,
        with_active_org_support: bool,
    ) -> CreateAccessTokenRequest {
        CreateAccessTokenRequest {
            user_id,
            active_org_id: None,
            duration_in_minutes,
            with_active_org_support,
        }
    }
}
