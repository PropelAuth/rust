#[cfg(feature = "schemars09")]
use {
    std::convert::TryFrom,
    schemars09 as schemars,
};
#[cfg(any(feature = "schemars09", feature = "schemars1"))]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars1"), derive(JsonSchema))]
pub struct BadCreateAccessTokenError {
    #[serde(rename = "active_org_id", skip_serializing_if = "Option::is_none")]
    pub active_org_id: Option<Vec<String>>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Vec<String>>,
}

impl BadCreateAccessTokenError {
    pub fn new() -> BadCreateAccessTokenError {
        BadCreateAccessTokenError {
            active_org_id: None,
            user_id: None,
        }
    }
}
