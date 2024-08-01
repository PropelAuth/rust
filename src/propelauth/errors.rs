use crate::models::{
    BadCreateAccessTokenError, BadCreateMagicLinkRequest, BadCreateOrgRequest,
    BadCreateUserRequest, BadFetchOrgQuery, BadFetchUsersByQuery, BadFetchUsersInOrgQuery,
    BadMigrateUserRequest, BadUpdateOrgRequest, BadUpdatePasswordRequest,
    BadUpdateUserEmailRequest, BadUpdateUserMetadataRequest,
};
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum InitializationError {
    #[error("Invalid auth URL")]
    InvalidAuthUrl,

    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum ErrorsWithNotFound {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Not found")]
    NotFound,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum BatchFetchError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum FetchByQueryError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadFetchUsersByQuery),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum CreateUserError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadCreateUserRequest),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum UpdateUserMetadataError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadUpdateUserMetadataRequest),

    #[error("Not found")]
    NotFound,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum UpdateUserEmailError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadUpdateUserEmailRequest),

    #[error("Not found")]
    NotFound,

    #[error("Email sent too recently")]
    EmailSentTooRecently,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ResendEmailConfirmationError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(String),

    #[error("Too many requests")]
    TooManyRequests,

    #[error("Not found")]
    NotFound,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum UpdatePasswordError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadUpdatePasswordRequest),

    #[error("Not found")]
    NotFound,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ClearPasswordError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Not found")]
    NotFound,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum MigrateUserError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadMigrateUserRequest),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum CreateMagicLinkError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadCreateMagicLinkRequest),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum UpdateOrgError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Not found")]
    NotFound,

    #[error("Bad request")]
    BadRequest(BadUpdateOrgRequest),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum SubscribeOrgToRoleMappingError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Not found")]
    NotFound,

    #[error("Bad request")]
    BadRequest(BadUpdateOrgRequest),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum InviteUserToOrgError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Not found")]
    NotFound,

    #[error("Bad request")]
    BadRequest(serde_json::Value),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum CreateOrgError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadCreateOrgRequest),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum OrgMissingOrRoleError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Unknown role")]
    UnknownRoleError,

    #[error("Not found")]
    NotFound,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum FetchUsersInOrgError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadFetchUsersInOrgQuery),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum FetchOrgsByQueryError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Bad request")]
    BadRequest(BadFetchOrgQuery),

    #[error("Unexpected exception, please try again")]
    UnexpectedException,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum CreateAccessTokenError {
    #[error("Invalid API Key")]
    InvalidApiKey,

    #[error("Unexpected exception, please try again")]
    UnexpectedException,

    #[error("Not found")]
    NotFound,

    #[error("Bad request")]
    BadRequest(BadCreateAccessTokenError),
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum DetailedAuthError {
    #[error("Invalid public key for decoding the JWT - make sure it's formatted correctly")]
    InvalidPublicKey,

    #[error("Access token is invalid")]
    CannotVerifyToken,

    #[error("Expected the Authorization header to be `Bearer ACCESS_TOKEN`")]
    IncorrectlyFormattedHeader,
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnauthorizedError {
    #[error("Unauthorized")]
    Unauthorized(DetailedAuthError),
}

impl From<DetailedAuthError> for UnauthorizedError {
    fn from(err: DetailedAuthError) -> Self {
        UnauthorizedError::Unauthorized(err)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum DetailedForbiddenError {
    #[error("User is not a member of that organization")]
    UserIsNotInOrg,

    #[error("User's role doesn't match required role")]
    UserRoleDoesntMatch,

    #[error("User doesn't have all required permissions")]
    UserMissingPermission,
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnauthorizedOrForbiddenError {
    #[error("Unauthorized")]
    Unauthorized(DetailedAuthError),

    #[error("Forbidden error")]
    Forbidden(DetailedForbiddenError),
}

impl From<DetailedForbiddenError> for UnauthorizedOrForbiddenError {
    fn from(err: DetailedForbiddenError) -> Self {
        UnauthorizedOrForbiddenError::Forbidden(err)
    }
}

impl From<DetailedAuthError> for UnauthorizedOrForbiddenError {
    fn from(err: DetailedAuthError) -> Self {
        UnauthorizedOrForbiddenError::Unauthorized(err)
    }
}

impl From<UnauthorizedError> for UnauthorizedOrForbiddenError {
    fn from(err: UnauthorizedError) -> Self {
        match err {
            UnauthorizedError::Unauthorized(detailed_error) => {
                UnauthorizedOrForbiddenError::Unauthorized(detailed_error)
            }
        }
    }
}
