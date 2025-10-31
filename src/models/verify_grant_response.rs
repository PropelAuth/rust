#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyStepUpGrantResponse {
    #[serde(rename = "success")]
    pub success: bool,
}

impl VerifyStepUpGrantResponse {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
