#[cfg(feature = "schemars09")]
use {
    std::convert::TryFrom,
    schemars09 as schemars,
};
#[cfg(any(feature = "schemars09", feature = "schemars1"))]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Default, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars1"), derive(JsonSchema))]
pub struct FetchSamlSpMetadataResponse {
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    #[serde(rename = "acs_url")]
    pub acs_url: String,
    #[serde(rename = "logout_url")]
    pub logout_url: String,
}
