#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub api_key_id: String,
    pub api_key_token: String,
}

impl CreateApiKeyResponse {
    pub fn new(api_key_id: String, api_key_token: String) -> Self {
        CreateApiKeyResponse {
            api_key_id,
            api_key_token,
        }
    }
}
