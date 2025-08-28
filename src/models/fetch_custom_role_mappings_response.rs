#[cfg(feature = "schemars09")]
use {
    std::convert::TryFrom,
    schemars09 as schemars,
};
#[cfg(any(feature = "schemars09", feature = "schemars1"))]
use schemars::JsonSchema;

#[derive(Deserialize, Debug)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars1"), derive(JsonSchema))]
pub struct FetchCustomRoleMappingsResponse {
    #[serde(rename = "custom_role_mappings", default)]
    pub custom_role_mappings: Vec<CustomRoleMappingResponse>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars1"), derive(JsonSchema))]
pub struct CustomRoleMappingResponse {
    #[serde(rename = "custom_role_mapping_name")]
    pub custom_role_mapping_name: String,
    #[serde(rename = "num_orgs_subscribed")]
    pub num_orgs_subscribed: i32,
}
