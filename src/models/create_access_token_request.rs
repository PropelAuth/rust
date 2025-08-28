#[cfg(feature = "schemars09")]
use {
    std::convert::TryFrom,
    schemars09 as schemars,
};
#[cfg(any(feature = "schemars09", feature = "schemars-latest"))]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars-latest"), derive(JsonSchema))]
pub struct CreateAccessTokenRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "duration_in_minutes")]
    pub duration_in_minutes: u64,
    #[serde(rename = "active_org_id")]
    pub active_org_id: Option<String>,
}

impl CreateAccessTokenRequest {
    pub fn new(user_id: String, duration_in_minutes: u64, active_org_id: Option<String>) -> CreateAccessTokenRequest {
        CreateAccessTokenRequest {
            user_id,
            duration_in_minutes,
            active_org_id
        }
    }
}
