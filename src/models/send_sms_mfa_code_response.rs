#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendSmsCodeResponse {
    #[serde(rename = "challenge_id")]
    pub challenge_id: String,
}

impl SendSmsCodeResponse {
    pub fn new(challenge_id: String) -> Self {
        Self { challenge_id }
    }
}
