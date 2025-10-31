
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MfaPhones {
    #[serde(rename = "mfa_phone_number_suffix")]
    pub mfa_phone_number_suffix: String,
     #[serde(rename = "mfa_phone_id")]
    pub mfa_phone_id: String
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MfaSetupType {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "phone_numbers")]
    pub phone_numbers: Option<Vec<MfaPhones>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchUserMfaMethodsResponse {
    #[serde(rename = "mfa_setup")]
    pub mfa_setup: Option<MfaSetupType>,
}

impl FetchUserMfaMethodsResponse {
    pub fn new(mfa_setup: Option<MfaSetupType>) -> Self {
        FetchUserMfaMethodsResponse {
            mfa_setup,
        }
    }
}
