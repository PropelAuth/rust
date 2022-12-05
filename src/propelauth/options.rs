use crate::models::AuthTokenVerificationMetadata;

pub struct AuthOptionsWithTokenVerification {
    pub auth_url: String,
    pub api_key: String,

    /// By default, this library performs a one-time fetch on startup for
    ///   token verification metadata from your authUrl using your apiKey.
    /// This is usually preferred to make sure you have the most up to date information,
    ///  however, in environments like serverless, this one-time fetch becomes a
    ///  per-request fetch.
    /// You can specify the token verification metadata manually,
    ///  which you can obtain from your PropelAuth project.
    pub manual_token_verification_metadata: AuthTokenVerificationMetadata,
}

pub struct AuthOptions {
    pub auth_url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequiredOrg<'a> {
    OrgId(&'a str),
    OrgName(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRequirementsInOrg<'a> {
    None,
    IsRole(&'a str),
    IsAtLeastRole(&'a str),
    HasPermission(&'a str),
    HasAllPermissions(Vec<&'a str>),
}
