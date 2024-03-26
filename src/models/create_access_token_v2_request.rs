#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]

pub struct CreateAccessTokenV2Request {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "duration_in_minutes")]
    pub duration_in_minutes: u64,

    #[serde(rename = "active_org_id", skip_serializing_if = "Option::is_none")]
    pub active_org_id: Option<String>,
}

impl CreateAccessTokenV2Request {
    pub fn new(user_id: String, duration_in_minutes: u64) -> CreateAccessTokenV2Request {
        CreateAccessTokenV2Request {
            user_id,
            duration_in_minutes,
            active_org_id: None,
        }
    }
}
