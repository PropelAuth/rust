#[cfg(any(feature = "schemars09", feature = "schemars-latest"))]
use schemars::JsonSchema;
#[cfg(feature = "schemars09")]
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(any(feature = "schemars09", feature = "schemars-latest"), derive(JsonSchema))]
pub struct ResendEmailConfirmationRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl ResendEmailConfirmationRequest {
    pub fn new(user_id: String) -> ResendEmailConfirmationRequest {
        ResendEmailConfirmationRequest { user_id }
    }
}
