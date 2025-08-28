#[cfg(feature = "schemars09")]
use {
    std::convert::TryFrom,
    schemars09 as schemars,
};
#[cfg(any(feature = "schemars09", feature = "schemars-latest"))]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars-latest"), derive(JsonSchema))]
pub struct SubscribeOrgToRoleMappingRequest {
    #[serde(rename = "custom_role_mapping_name")]
    pub custom_role_mapping_name: String,
}

impl SubscribeOrgToRoleMappingRequest {
    pub fn new(custom_role_mapping_name: String) -> SubscribeOrgToRoleMappingRequest {
        SubscribeOrgToRoleMappingRequest {
            custom_role_mapping_name,
        }
    }
}
