#[cfg(any(feature = "schemars09", feature = "schemars-latest"))]
use schemars::JsonSchema;
#[cfg(feature = "schemars09")]
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars-latest"), derive(JsonSchema))]
pub struct CreateSamlConnectionLinkResponse {
    #[serde(rename = "url")]
    pub url: String,
}

impl CreateSamlConnectionLinkResponse {
    pub fn new(url: String) -> CreateSamlConnectionLinkResponse {
        CreateSamlConnectionLinkResponse { url }
    }
}
