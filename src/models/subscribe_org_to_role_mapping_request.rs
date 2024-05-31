#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubscribeOrgToRoleMappingRequest {
    #[serde(rename = "custom_role_mapping_id")]
    pub custom_role_mapping_id: Option<String>,
}

impl SubscribeOrgToRoleMappingRequest {
    pub fn new() -> SubscribeOrgToRoleMappingRequest {
        SubscribeOrgToRoleMappingRequest {
            custom_role_mapping_id: None,
        }
    }
}
