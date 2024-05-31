#[derive(Deserialize)]
pub struct FetchCustomRoleMappingsResponse {
    #[serde(rename = "custom_role_mappings", default)]
    pub custom_role_mappings: Vec<CustomRoleMappingResponse>,
}

#[derive(Deserialize)]
pub struct CustomRoleMappingResponse {
    #[serde(rename = "custom_role_mapping_name")]
    pub custom_role_mapping_name: String,
    #[serde(rename = "num_orgs_subscribed")]
    pub num_orgs_subscribed: i32,
}
