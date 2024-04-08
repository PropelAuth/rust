#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateAccessTokenRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "duration_in_minutes")]
    pub duration_in_minutes: u64,
}

impl CreateAccessTokenRequest {
    pub fn new(user_id: String, duration_in_minutes: u64) -> CreateAccessTokenRequest {
        CreateAccessTokenRequest {
            user_id,
            duration_in_minutes,
        }
    }
}
