#[cfg(feature = "schemars09")]
use {
    std::convert::TryFrom,
    schemars09 as schemars,
};
#[cfg(any(feature = "schemars09", feature = "schemars1"))]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars1"), derive(JsonSchema))]
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
