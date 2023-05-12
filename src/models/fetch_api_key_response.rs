#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchApiKeyResponse {
    pub api_key_id: String,
    pub created_at: i32,
    pub expires_at_seconds: i64,
    pub metadata: Option<serde_json::Value>,
    pub user_id: Option<String>,
    pub org_id: Option<String>,
}

impl FetchApiKeyResponse {
    pub fn new(api_key_id: String, created_at: i32, expires_at_seconds: i64, metadata: Option<serde_json::Value>, user_id: Option<String>, org_id: Option<String>) -> Self {
        Self { api_key_id, created_at, expires_at_seconds, metadata, user_id, org_id }
    }
}
