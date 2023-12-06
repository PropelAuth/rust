#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchApiKeysPagedResponse {
    #[serde(rename = "api_keys")]
    pub api_keys: Vec<crate::models::FetchApiKeyResponse>,
    #[serde(rename = "total_api_keys")]
    pub total_api_keys: i64,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    #[serde(rename = "page_size")]
    pub page_size: i64,
    #[serde(rename = "has_more_results")]
    pub has_more_results: bool,
}

impl FetchApiKeysPagedResponse {
    pub fn new(
        api_keys: Vec<crate::models::FetchApiKeyResponse>,
        total_api_keys: i64,
        current_page: i64,
        page_size: i64,
        has_more_results: bool,
    ) -> Self {
        Self {
            api_keys,
            total_api_keys,
            current_page,
            page_size,
            has_more_results,
        }
    }
}
