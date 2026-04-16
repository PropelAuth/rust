use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialLoginTokenProvider {
    Apple,
    Google,
    Github,
    Microsoft,
    Slack,
    Salesforce,
    Linkedin,
    Outreach,
    Quickbooks,
    Xero,
    Salesloft,
    Atlassian,
    Gitlab,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialLoginToken {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "token_provider")]
    pub token_provider: SocialLoginTokenProvider,
    #[serde(rename = "token_expiration", skip_serializing_if = "Option::is_none")]
    pub token_expiration: Option<i64>,
    #[serde(rename = "authorized_scopes", skip_serializing_if = "Option::is_none")]
    pub authorized_scopes: Option<Vec<String>>,
}

pub type SocialLoginTokensResponse = HashMap<SocialLoginTokenProvider, SocialLoginToken>;

impl std::fmt::Display for SocialLoginTokenProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SocialLoginTokenProvider::Apple => "apple",
            SocialLoginTokenProvider::Google => "google",
            SocialLoginTokenProvider::Github => "github",
            SocialLoginTokenProvider::Microsoft => "microsoft",
            SocialLoginTokenProvider::Slack => "slack",
            SocialLoginTokenProvider::Salesforce => "salesforce",
            SocialLoginTokenProvider::Linkedin => "linkedin",
            SocialLoginTokenProvider::Outreach => "outreach",
            SocialLoginTokenProvider::Quickbooks => "quickbooks",
            SocialLoginTokenProvider::Xero => "xero",
            SocialLoginTokenProvider::Salesloft => "salesloft",
            SocialLoginTokenProvider::Atlassian => "atlassian",
            SocialLoginTokenProvider::Gitlab => "gitlab",
        };
        write!(f, "{}", s)
    }
}
