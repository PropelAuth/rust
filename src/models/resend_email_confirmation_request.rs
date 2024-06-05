#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResendEmailConfirmationRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl ResendEmailConfirmationRequest {
    pub fn new(user_id: String) -> ResendEmailConfirmationRequest {
        ResendEmailConfirmationRequest { user_id }
    }
}
