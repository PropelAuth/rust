#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyTotpChallengeResponse {
    #[serde(rename = "step_up_grant")]
    pub step_up_grant: String,
}

impl VerifyTotpChallengeResponse {
    pub fn new(step_up_grant: String) -> Self {
        Self { step_up_grant }
    }
}
