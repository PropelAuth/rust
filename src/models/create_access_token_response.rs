#[cfg(any(feature = "schemars09", feature = "schemars-latest"))]
use schemars::JsonSchema;
#[cfg(feature = "schemars09")]
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars-latest"), derive(JsonSchema))]
pub struct CreateAccessTokenResponse {
    pub access_token: String,
}

impl CreateAccessTokenResponse {
    pub fn new(access_token: String) -> Self {
        CreateAccessTokenResponse {
            access_token,
        }
    }
}
