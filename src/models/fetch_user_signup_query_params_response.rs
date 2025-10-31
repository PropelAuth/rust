use std::collections::HashMap;

use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchSignupQueryParamsResponse {
    #[serde(rename = "user_signup_query_parameters")]
    pub user_signup_query_parameters: HashMap<String, Value>,
}

impl FetchSignupQueryParamsResponse {
    pub fn new(user_signup_query_parameters: HashMap<String, Value>) -> Self {
        Self { user_signup_query_parameters }
    }
}
