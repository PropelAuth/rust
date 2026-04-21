#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetOidcIdpMetadataRequestBase {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "uses_pkce")]
    pub uses_pkce: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetGenericOidcMetadataRequest {
    #[serde(flatten)]
    pub base: SetOidcIdpMetadataRequestBase,
    #[serde(rename = "auth_url")]
    pub auth_url: String,
    #[serde(rename = "token_url")]
    pub token_url: String,
    #[serde(rename = "userinfo_url")]
    pub userinfo_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetOktaOidcMetadataRequest {
    #[serde(flatten)]
    pub base: SetOidcIdpMetadataRequestBase,
    #[serde(rename = "okta_sso_domain")]
    pub okta_sso_domain: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetAzureOidcMetadataRequest {
    #[serde(flatten)]
    pub base: SetOidcIdpMetadataRequestBase,
    #[serde(rename = "entra_tenant_id")]
    pub entra_tenant_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "idp_type")]
pub enum SetOidcIdpMetadataRequest {
    Generic(SetGenericOidcMetadataRequest),
    Okta(SetOktaOidcMetadataRequest),
    Azure(SetAzureOidcMetadataRequest),
}
