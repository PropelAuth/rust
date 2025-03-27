use std::collections::HashMap;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::models::AuthTokenVerificationMetadata;
use crate::propelauth::errors::{
    DetailedAuthError, UnauthorizedError, UnauthorizedOrForbiddenError,
};
use crate::propelauth::options::{RequiredOrg, UserRequirementsInOrg};
use crate::propelauth::token_models::{OrgMemberInfo, User, UserAndOrgMemberInfo};

use super::token_models::LoginMethodForAccessToken;

pub struct TokenService<'a> {
    pub(crate) token_verification_metadata: &'a AuthTokenVerificationMetadata,
    pub(crate) issuer: &'a str,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
struct DecodedUserFromToken {
    user_id: String,

    email: String,

    #[serde(default)]
    first_name: Option<String>,

    #[serde(default)]
    last_name: Option<String>,

    #[serde(default)]
    username: Option<String>,

    #[serde(default)]
    org_id_to_org_member_info: Option<HashMap<String, OrgMemberInfo>>,

    #[serde(default)]
    org_member_info: Option<OrgMemberInfo>,

    #[serde(default)]
    legacy_user_id: Option<String>,

    #[serde(default)]
    properties: Option<HashMap<String, serde_json::Value>>,

    #[serde(default)]
    metadata: HashMap<String, String>,

    #[serde(default)]
    impersonator_user_id: Option<String>,

    #[serde(default = "default_login_method")]
    login_method: LoginMethodForAccessToken,
}

fn default_login_method() -> LoginMethodForAccessToken {
    LoginMethodForAccessToken {
        login_method: "unknown".to_string(),
        provider: None,
        org_id: None,
    }
}

impl Into<User> for DecodedUserFromToken {
    fn into(self) -> User {
        let mut active_org_id: Option<String> = None;
        let mut org_id_to_org_member_info = HashMap::<String, OrgMemberInfo>::new();
        if let Some(org_member_info) = &self.org_member_info {
            active_org_id = Some(org_member_info.org_id.clone());
            org_id_to_org_member_info
                .insert(org_member_info.org_id.clone(), org_member_info.clone());
        } else if let Some(org_id_to_org_member_info_from_token) = self.org_id_to_org_member_info {
            for (org_id, org_member_info) in org_id_to_org_member_info_from_token {
                org_id_to_org_member_info.insert(org_id, org_member_info);
            }
        }
        User {
            user_id: self.user_id,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            username: self.username,
            org_id_to_org_member_info,
            legacy_user_id: self.legacy_user_id,
            impersonator_user_id: self.impersonator_user_id,
            properties: self.properties,
            metadata: self.metadata,
            active_org_id,
            login_method: self.login_method.into(),
        }
    }
}

impl TokenService<'_> {
    pub fn validate_authorization_header(
        &self,
        authorization_header: &str,
    ) -> Result<User, UnauthorizedError> {
        let bearer_token = Self::extract_bearer_token(authorization_header)?;
        Ok(self.verify_token(bearer_token)?)
    }

    pub fn validate_authorization_header_and_check_org_access(
        &self,
        authorization_header: &str,
        required_org: RequiredOrg,
        user_requirements_in_org: UserRequirementsInOrg,
    ) -> Result<UserAndOrgMemberInfo, UnauthorizedOrForbiddenError> {
        let user = self.validate_authorization_header(authorization_header)?;
        let org_member_info =
            user.validate_org_membership(required_org, user_requirements_in_org)?;
        Ok(UserAndOrgMemberInfo {
            user,
            org_member_info,
        })
    }

    fn extract_bearer_token(authorization_header: &str) -> Result<String, DetailedAuthError> {
        if authorization_header.len() < 7 {
            return Err(DetailedAuthError::IncorrectlyFormattedHeader);
        }

        if authorization_header[0..7].to_lowercase() != "bearer " {
            return Err(DetailedAuthError::IncorrectlyFormattedHeader);
        }

        let token = authorization_header[7..].trim().to_string();
        Ok(token)
    }

    fn verify_token(&self, bearer_token: String) -> Result<User, DetailedAuthError> {
        let decoding_key_bytes = self.token_verification_metadata.public_key_pem.as_bytes();
        let decoding_key = DecodingKey::from_rsa_pem(decoding_key_bytes)
            .map_err(|_| DetailedAuthError::InvalidPublicKey)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[self.issuer]);
        validation.validate_aud = false;

        let decoded_user =
            decode::<DecodedUserFromToken>(&bearer_token, &decoding_key, &validation)
                .map(|jwt| jwt.claims)
                .map_err(|_| DetailedAuthError::CannotVerifyToken);

        decoded_user.map(|decoded_user| decoded_user.into())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::SystemTime;

    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use openssl::rsa::Rsa;

    use crate::models::AuthTokenVerificationMetadata;
    use crate::propelauth::errors::{
        DetailedAuthError, DetailedForbiddenError, UnauthorizedError, UnauthorizedOrForbiddenError,
    };
    use crate::propelauth::options::RequiredOrg::{OrgId, OrgName};
    use crate::propelauth::options::UserRequirementsInOrg;
    use crate::propelauth::token::TokenService;
    use crate::propelauth::token_models::{
        LoginMethod, OrgMemberInfo, OrgRoleStructure, User, UserAndOrgMemberInfo,
    };

    const ISSUER: &str = "https://testissuer.propelauthtest.com";

    #[test]
    fn validation_gets_user_back() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            ..Default::default()
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Bearer {}", jwt);

        let user = token_service
            .validate_authorization_header(&auth_header)
            .unwrap();

        assert_eq!(user.user_id, expected_user.user_id);
        assert_eq!(
            user.org_id_to_org_member_info,
            expected_user.org_id_to_org_member_info
        );
        assert_eq!(user.legacy_user_id, expected_user.legacy_user_id);
    }

    #[test]
    fn validation_gets_user_with_orgs_back() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            email: "easteregg@propelauth.com".to_string(),
            first_name: Some("Easter".to_string()),
            last_name: Some("Egg".to_string()),
            username: None,
            org_id_to_org_member_info: get_org_id_to_org_member_info(),
            legacy_user_id: Some("legacy_id".to_string()),
            impersonator_user_id: None,
            properties: None,
            active_org_id: None,
            metadata: HashMap::new(),
            login_method: LoginMethod::Unknown,
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Bearer {}", jwt);

        let user = token_service
            .validate_authorization_header(&auth_header)
            .unwrap();

        assert_eq!(user.user_id, expected_user.user_id);
        assert_eq!(
            user.org_id_to_org_member_info,
            expected_user.org_id_to_org_member_info
        );
        assert_eq!(user.legacy_user_id, expected_user.legacy_user_id);
        assert_eq!(user.email, expected_user.email);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.username, expected_user.username);
    }

    #[test]
    fn validation_gets_user_with_orgs_back_multi_role() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            email: "easteregg@propelauth.com".to_string(),
            first_name: Some("Easter".to_string()),
            last_name: Some("Egg".to_string()),
            username: None,
            org_id_to_org_member_info: get_org_id_to_org_member_info_multi_role(),
            legacy_user_id: Some("legacy_id".to_string()),
            impersonator_user_id: None,
            properties: None,
            active_org_id: None,
            metadata: HashMap::new(),
            login_method: LoginMethod::Unknown,
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Bearer {}", jwt);

        let user = token_service
            .validate_authorization_header(&auth_header)
            .unwrap();

        assert_eq!(user.user_id, expected_user.user_id);
        assert_eq!(
            user.org_id_to_org_member_info,
            expected_user.org_id_to_org_member_info
        );
        assert_eq!(user.legacy_user_id, expected_user.legacy_user_id);
        assert_eq!(user.email, expected_user.email);
        assert_eq!(user.first_name, expected_user.first_name);
        assert_eq!(user.last_name, expected_user.last_name);
        assert_eq!(user.username, expected_user.username);
    }

    #[test]
    fn reject_expired_tokens() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            ..Default::default()
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), -1);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Bearer {}", jwt);

        let result = token_service.validate_authorization_header(&auth_header);
        assert_eq!(
            result.err(),
            Some(UnauthorizedError::Unauthorized(
                DetailedAuthError::CannotVerifyToken
            ))
        );
    }

    #[test]
    fn reject_invalid_headers() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            ..Default::default()
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);

        let result = token_service.validate_authorization_header("Bearer fake");
        assert_eq!(
            result.err(),
            Some(UnauthorizedError::Unauthorized(
                DetailedAuthError::CannotVerifyToken
            ))
        );

        let result = token_service.validate_authorization_header("");
        assert_eq!(
            result.err(),
            Some(UnauthorizedError::Unauthorized(
                DetailedAuthError::IncorrectlyFormattedHeader
            ))
        );

        let result = token_service.validate_authorization_header(&jwt);
        assert_eq!(
            result.err(),
            Some(UnauthorizedError::Unauthorized(
                DetailedAuthError::IncorrectlyFormattedHeader
            ))
        );
    }

    #[test]
    fn reject_invalid_issuer() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            ..Default::default()
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let mut token_service = get_token_service(&token_verification_metadata);
        token_service.issuer = "overridden";

        let auth_header = format!("Bearer {}", jwt);

        let result = token_service.validate_authorization_header(&auth_header);
        assert_eq!(
            result.err(),
            Some(UnauthorizedError::Unauthorized(
                DetailedAuthError::CannotVerifyToken
            ))
        );
    }

    #[test]
    fn validation_checks_orgs_correctly() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            email: "easteregg@propelauth.com".to_string(),
            first_name: Some("Easter".to_string()),
            last_name: None,
            username: None,
            org_id_to_org_member_info: get_org_id_to_org_member_info(),
            legacy_user_id: Some("legacy_id".to_string()),
            impersonator_user_id: None,
            properties: None,
            active_org_id: None,
            metadata: HashMap::new(),
            login_method: LoginMethod::Unknown,
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Bearer {}", jwt);

        let expected_successful_response_for_org_id_1 =
            Ok(get_expected_org_response(&expected_user, "org_id_1"));
        for required_org in vec![OrgId("org_id_1"), OrgName("org_name_1")] {
            for requirements in vec![
                UserRequirementsInOrg::None,
                UserRequirementsInOrg::IsRole("Owner"),
                UserRequirementsInOrg::IsAtLeastRole("Owner"),
                UserRequirementsInOrg::IsAtLeastRole("Admin"),
                UserRequirementsInOrg::IsAtLeastRole("Member"),
                UserRequirementsInOrg::HasPermission("custom_permission_for_owner"),
                UserRequirementsInOrg::HasAllPermissions(vec![]),
                UserRequirementsInOrg::HasAllPermissions(vec!["custom_permission_for_owner"]),
            ] {
                let result = token_service.validate_authorization_header_and_check_org_access(
                    &auth_header,
                    required_org.clone(),
                    requirements,
                );
                assert_eq!(result, expected_successful_response_for_org_id_1);
            }

            for requirements in vec![
                UserRequirementsInOrg::IsRole("Admin"),
                UserRequirementsInOrg::IsRole("Member"),
                UserRequirementsInOrg::IsRole("fake"),
                UserRequirementsInOrg::IsAtLeastRole("fake"),
                UserRequirementsInOrg::HasPermission("something"),
                UserRequirementsInOrg::HasAllPermissions(vec![
                    "custom_permission_for_owner",
                    "fake",
                ]),
            ] {
                let result = token_service.validate_authorization_header_and_check_org_access(
                    &auth_header,
                    required_org.clone(),
                    requirements,
                );
                assert!(result.is_err())
            }
        }

        let expected_successful_response_for_org_id_2 =
            Ok(get_expected_org_response(&expected_user, "org_id_2"));
        for required_org in vec![OrgId("org_id_2"), OrgName("org_name_2")] {
            for requirements in vec![
                UserRequirementsInOrg::None,
                UserRequirementsInOrg::IsRole("Admin"),
                UserRequirementsInOrg::IsAtLeastRole("Admin"),
                UserRequirementsInOrg::IsAtLeastRole("Member"),
                UserRequirementsInOrg::HasPermission("custom_permission_for_admin"),
                UserRequirementsInOrg::HasAllPermissions(vec![]),
                UserRequirementsInOrg::HasAllPermissions(vec!["custom_permission_for_admin"]),
            ] {
                let result = token_service.validate_authorization_header_and_check_org_access(
                    &auth_header,
                    required_org.clone(),
                    requirements,
                );
                assert_eq!(result, expected_successful_response_for_org_id_2);
            }

            for requirements in vec![
                UserRequirementsInOrg::IsRole("Owner"),
                UserRequirementsInOrg::IsRole("Member"),
                UserRequirementsInOrg::IsRole("fake"),
                UserRequirementsInOrg::IsAtLeastRole("Owner"),
                UserRequirementsInOrg::IsAtLeastRole("fake"),
                UserRequirementsInOrg::HasPermission("something"),
                UserRequirementsInOrg::HasAllPermissions(vec![
                    "custom_permission_for_admin",
                    "fake",
                ]),
            ] {
                let result = token_service.validate_authorization_header_and_check_org_access(
                    &auth_header,
                    required_org.clone(),
                    requirements,
                );
                assert!(result.is_err())
            }
        }

        let expected_successful_response_for_org_id_3 =
            Ok(get_expected_org_response(&expected_user, "org_id_3"));
        for required_org in vec![OrgId("org_id_3"), OrgName("org_name_3")] {
            for requirements in vec![
                UserRequirementsInOrg::None,
                UserRequirementsInOrg::IsRole("Member"),
                UserRequirementsInOrg::IsAtLeastRole("Member"),
                UserRequirementsInOrg::HasPermission("custom_permission_for_member"),
                UserRequirementsInOrg::HasAllPermissions(vec![]),
                UserRequirementsInOrg::HasAllPermissions(vec!["custom_permission_for_member"]),
            ] {
                let result = token_service.validate_authorization_header_and_check_org_access(
                    &auth_header,
                    required_org.clone(),
                    requirements,
                );
                assert_eq!(result, expected_successful_response_for_org_id_3);
            }

            for requirements in vec![
                UserRequirementsInOrg::IsRole("Owner"),
                UserRequirementsInOrg::IsRole("Admin"),
                UserRequirementsInOrg::IsRole("fake"),
                UserRequirementsInOrg::IsAtLeastRole("Owner"),
                UserRequirementsInOrg::IsAtLeastRole("Admin"),
                UserRequirementsInOrg::IsAtLeastRole("fake"),
                UserRequirementsInOrg::HasPermission("something"),
                UserRequirementsInOrg::HasAllPermissions(vec![
                    "custom_permission_for_member",
                    "fake",
                ]),
            ] {
                let result = token_service.validate_authorization_header_and_check_org_access(
                    &auth_header,
                    required_org.clone(),
                    requirements,
                );
                assert!(result.is_err())
            }
        }

        for required_org in vec![
            OrgId("fake"),
            OrgId("idk"),
            OrgName("hihi"),
            OrgName("whatsup"),
        ] {
            let result = token_service.validate_authorization_header_and_check_org_access(
                &auth_header,
                required_org.clone(),
                UserRequirementsInOrg::None,
            );
            assert_eq!(
                result.err(),
                Some(UnauthorizedOrForbiddenError::Forbidden(
                    DetailedForbiddenError::UserIsNotInOrg
                ))
            );
        }
    }

    #[test]
    fn org_validation_can_throw_unauthorized() {
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            email: "easteregg@propelauth.com".to_string(),
            first_name: None,
            last_name: None,
            username: None,
            org_id_to_org_member_info: get_org_id_to_org_member_info(),
            legacy_user_id: Some("legacy_id".to_string()),
            impersonator_user_id: None,
            properties: None,
            active_org_id: None,
            metadata: HashMap::new(),
            login_method: LoginMethod::Unknown,
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Token {}", jwt);

        let result = token_service.validate_authorization_header_and_check_org_access(
            &auth_header,
            OrgId("org_id_1"),
            UserRequirementsInOrg::None,
        );
        assert_eq!(
            result,
            Err(UnauthorizedOrForbiddenError::Unauthorized(
                DetailedAuthError::IncorrectlyFormattedHeader
            ))
        );
    }

    fn get_expected_org_response(
        expected_user: &User,
        org_id: &'static str,
    ) -> UserAndOrgMemberInfo {
        UserAndOrgMemberInfo {
            user: expected_user.clone(),
            org_member_info: expected_user.get_org(OrgId(org_id)).unwrap().clone(),
        }
    }

    fn get_jwt_and_token_verification_metadata(
        user: User,
        expires_in_hours: i64,
    ) -> (String, AuthTokenVerificationMetadata) {
        let rsa = Rsa::generate(2048).unwrap();
        let public_key_pem = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();

        let iat = now_secs();
        let exp = iat + expires_in_hours * 60 * 60;
        let claims = TestJwtClaims {
            iat,
            exp,
            iss: ISSUER.to_string(),
            user_id: user.user_id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            org_id_to_org_member_info: user.org_id_to_org_member_info,
            legacy_user_id: user.legacy_user_id,
        };

        let header = Header::new(Algorithm::RS256);
        let encoding_key =
            EncodingKey::from_rsa_pem(rsa.private_key_to_pem().unwrap().as_ref()).unwrap();

        let jwt = encode(&header, &claims, &encoding_key).unwrap();

        (jwt, AuthTokenVerificationMetadata { public_key_pem })
    }

    fn get_token_service(
        token_verification_metadata: &AuthTokenVerificationMetadata,
    ) -> TokenService {
        TokenService {
            token_verification_metadata,
            issuer: ISSUER,
        }
    }

    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap()
    }

    fn get_org_id_to_org_member_info() -> HashMap<String, OrgMemberInfo> {
        let mut org_id_to_org_member_info = HashMap::new();
        org_id_to_org_member_info.insert(
            "org_id_1".to_string(),
            OrgMemberInfo {
                org_id: "org_id_1".to_string(),
                org_name: "org_name_1".to_string(),
                org_metadata: HashMap::new(),
                url_safe_org_name: "org_name_1".to_string(),
                org_role_structure: OrgRoleStructure::SingleRoleInHierarchy,
                user_role: "Owner".to_string(),
                inherited_user_roles_plus_current_role: vec![
                    "Owner".to_string(),
                    "Admin".to_string(),
                    "Member".to_string(),
                ],
                user_permissions: vec!["custom_permission_for_owner".to_string()],
                additional_roles: vec![],
            },
        );
        org_id_to_org_member_info.insert(
            "org_id_2".to_string(),
            OrgMemberInfo {
                org_id: "org_id_2".to_string(),
                org_name: "org_name_2".to_string(),
                org_metadata: HashMap::new(),
                url_safe_org_name: "org_name_2".to_string(),
                org_role_structure: OrgRoleStructure::SingleRoleInHierarchy,
                user_role: "Admin".to_string(),
                inherited_user_roles_plus_current_role: vec![
                    "Admin".to_string(),
                    "Member".to_string(),
                ],
                user_permissions: vec!["custom_permission_for_admin".to_string()],
                additional_roles: vec![],
            },
        );
        org_id_to_org_member_info.insert(
            "org_id_3".to_string(),
            OrgMemberInfo {
                org_id: "org_id_3".to_string(),
                org_name: "org_name_3".to_string(),
                org_metadata: HashMap::new(),
                url_safe_org_name: "org_name_3".to_string(),
                org_role_structure: OrgRoleStructure::SingleRoleInHierarchy,
                user_role: "Member".to_string(),
                inherited_user_roles_plus_current_role: vec!["Member".to_string()],
                user_permissions: vec!["custom_permission_for_member".to_string()],
                additional_roles: vec![],
            },
        );
        org_id_to_org_member_info
    }

    fn get_org_id_to_org_member_info_multi_role() -> HashMap<String, OrgMemberInfo> {
        let mut org_id_to_org_member_info = HashMap::new();
        org_id_to_org_member_info.insert(
            "org_id_1".to_string(),
            OrgMemberInfo {
                org_id: "org_id_1".to_string(),
                org_name: "org_name_1".to_string(),
                org_metadata: HashMap::new(),
                url_safe_org_name: "org_name_1".to_string(),
                org_role_structure: OrgRoleStructure::MultiRole,
                user_role: "Role A".to_string(),
                inherited_user_roles_plus_current_role: vec!["Role A".to_string()],
                user_permissions: vec!["custom_permission_for_owner".to_string()],
                additional_roles: vec!["Role B".to_string(), "Role C".to_string()],
            },
        );
        org_id_to_org_member_info.insert(
            "org_id_2".to_string(),
            OrgMemberInfo {
                org_id: "org_id_2".to_string(),
                org_name: "org_name_2".to_string(),
                org_metadata: HashMap::new(),
                url_safe_org_name: "org_name_2".to_string(),
                org_role_structure: OrgRoleStructure::MultiRole,
                user_role: "Role B".to_string(),
                inherited_user_roles_plus_current_role: vec!["Role B".to_string()],
                user_permissions: vec!["custom_permission_for_admin".to_string()],
                additional_roles: vec!["Role C".to_string()],
            },
        );
        org_id_to_org_member_info.insert(
            "org_id_3".to_string(),
            OrgMemberInfo {
                org_id: "org_id_3".to_string(),
                org_name: "org_name_3".to_string(),
                org_metadata: HashMap::new(),
                url_safe_org_name: "org_name_3".to_string(),
                org_role_structure: OrgRoleStructure::MultiRole,
                user_role: "Role C".to_string(),
                inherited_user_roles_plus_current_role: vec!["Role C".to_string()],
                user_permissions: vec!["custom_permission_for_member".to_string()],
                additional_roles: vec![],
            },
        );
        org_id_to_org_member_info
    }

    #[derive(Serialize)]
    struct TestJwtClaims {
        iat: i64,
        exp: i64,
        iss: String,
        user_id: String,

        email: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        org_id_to_org_member_info: HashMap<String, OrgMemberInfo>,
        #[serde(skip_serializing_if = "Option::is_none")]
        legacy_user_id: Option<String>,
    }
}
