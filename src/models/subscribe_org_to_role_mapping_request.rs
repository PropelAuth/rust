#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
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
