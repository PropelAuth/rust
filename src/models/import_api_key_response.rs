#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImportApiKeyResponse {
    pub api_key_id: String,
}

impl ImportApiKeyResponse {
    pub fn new(api_key_id: String) -> Self {
        ImportApiKeyResponse {
            api_key_id,
        }
    }
}
