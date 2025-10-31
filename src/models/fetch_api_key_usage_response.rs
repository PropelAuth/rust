#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchApiKeyUsageResponse {
    #[serde(rename = "count")]
    pub count: i64,
}

impl FetchApiKeyUsageResponse {
    pub fn new(count: i64) -> Self {
        Self { count }
    }
}
