#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateAccessTokenResponse {
    pub access_token: String,
}

impl CreateAccessTokenResponse {
    pub fn new(access_token: String) -> Self {
        CreateAccessTokenResponse { access_token }
    }
}
