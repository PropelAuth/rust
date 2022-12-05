use crate::models::AuthTokenVerificationMetadata;
use crate::propelauth::errors::{
    DetailedAuthError, DetailedForbiddenError, UnauthorizedError, UnauthorizedOrForbiddenError,
};
use crate::propelauth::options::{RequiredOrg, UserRequirementsInOrg};
use crate::propelauth::token_models::{OrgMemberInfo, User, UserAndOrgMemberInfo};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::collections::HashMap;

pub struct TokenService<'a> {
    pub(crate) token_verification_metadata: &'a AuthTokenVerificationMetadata,
    pub(crate) issuer: &'a str,
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
        let org_member_info = Self::validate_org_access_and_get_member_info(
            &user,
            required_org,
            user_requirements_in_org,
        )?;
        Ok(UserAndOrgMemberInfo {
            user,
            org_member_info,
        })
    }

    fn validate_org_access_and_get_member_info(
        user: &User,
        required_org: RequiredOrg,
        user_requirements_in_org: UserRequirementsInOrg,
    ) -> Result<OrgMemberInfo, DetailedForbiddenError> {
        let org_member_info =
            Self::get_user_info_in_org(required_org, &user.org_id_to_org_member_info)
                .ok_or(DetailedForbiddenError::UserIsNotInOrg)?;

        match user_requirements_in_org {
            UserRequirementsInOrg::None => Ok(org_member_info),
            UserRequirementsInOrg::IsRole(required_role) => {
                if org_member_info.is_role(required_role) {
                    Ok(org_member_info)
                } else {
                    Err(DetailedForbiddenError::UserRoleDoesntMatch)
                }
            }
            UserRequirementsInOrg::IsAtLeastRole(minimum_required_role) => {
                if org_member_info.is_at_least_role(minimum_required_role) {
                    Ok(org_member_info)
                } else {
                    Err(DetailedForbiddenError::UserRoleDoesntMatch)
                }
            }
            UserRequirementsInOrg::HasPermission(permission) => {
                if org_member_info.has_permission(permission) {
                    Ok(org_member_info)
                } else {
                    Err(DetailedForbiddenError::UserMissingPermission)
                }
            }
            UserRequirementsInOrg::HasAllPermissions(permissions) => {
                if org_member_info.has_all_permissions(permissions) {
                    Ok(org_member_info)
                } else {
                    Err(DetailedForbiddenError::UserMissingPermission)
                }
            }
        }
    }

    fn get_user_info_in_org(
        required_org: RequiredOrg,
        org_id_to_org_member_info: &HashMap<String, OrgMemberInfo>,
    ) -> Option<OrgMemberInfo> {
        let org_member_info = match required_org {
            RequiredOrg::OrgId(required_org_id) => {
                org_id_to_org_member_info.get(required_org_id)?
            }
            RequiredOrg::OrgName(required_org_name) => {
                org_id_to_org_member_info.values().find(|org_member_info| {
                    org_member_info.org_name == required_org_name
                        || org_member_info.url_safe_org_name == required_org_name
                })?
            }
        };

        Some((*org_member_info).clone())
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

        decode::<User>(&bearer_token, &decoding_key, &validation)
            .map(|jwt| jwt.claims)
            .map_err(|_| DetailedAuthError::CannotVerifyToken)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::AuthTokenVerificationMetadata;
    use crate::propelauth::errors::{
        DetailedAuthError, DetailedForbiddenError, UnauthorizedError, UnauthorizedOrForbiddenError,
    };
    use crate::propelauth::options::RequiredOrg::{OrgId, OrgName};
    use crate::propelauth::options::UserRequirementsInOrg;
    use crate::propelauth::token::TokenService;
    use crate::propelauth::token_models::{OrgMemberInfo, User, UserAndOrgMemberInfo};
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use openssl::rsa::Rsa;
    use std::collections::HashMap;
    use std::time::SystemTime;

    const ISSUER: &'static str = "https://testissuer.propelauthtest.com";

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
            org_id_to_org_member_info: get_org_id_to_org_member_info(),
            legacy_user_id: Some("legacy_id".to_string()),
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
        let org_id_to_org_member_info = get_org_id_to_org_member_info();
        let expected_user = User {
            user_id: "bf7b3bc0-739d-45a2-ba60-60655249a5b0".to_string(),
            org_id_to_org_member_info: org_id_to_org_member_info.clone(),
            legacy_user_id: Some("legacy_id".to_string()),
        };
        let (jwt, token_verification_metadata) =
            get_jwt_and_token_verification_metadata(expected_user.clone(), 24);
        let token_service = get_token_service(&token_verification_metadata);
        let auth_header = format!("Bearer {}", jwt);

        let expected_successful_response_for_org_id_1 = Ok(get_expected_org_response(
            &expected_user,
            &org_id_to_org_member_info,
            "org_id_1",
        ));
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

        let expected_successful_response_for_org_id_2 = Ok(get_expected_org_response(
            &expected_user,
            &org_id_to_org_member_info,
            "org_id_2",
        ));
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

        let expected_successful_response_for_org_id_3 = Ok(get_expected_org_response(
            &expected_user,
            &org_id_to_org_member_info,
            "org_id_3",
        ));
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
            org_id_to_org_member_info: get_org_id_to_org_member_info(),
            legacy_user_id: Some("legacy_id".to_string()),
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
        org_id_to_org_member_info: &HashMap<String, OrgMemberInfo>,
        org_id: &'static str,
    ) -> UserAndOrgMemberInfo {
        UserAndOrgMemberInfo {
            user: expected_user.clone(),
            org_member_info: org_id_to_org_member_info.get(org_id).unwrap().clone(),
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
                url_safe_org_name: "org_name_1".to_string(),
                user_role: "Owner".to_string(),
                inherited_user_roles_plus_current_role: vec![
                    "Owner".to_string(),
                    "Admin".to_string(),
                    "Member".to_string(),
                ],
                user_permissions: vec!["custom_permission_for_owner".to_string()],
            },
        );
        org_id_to_org_member_info.insert(
            "org_id_2".to_string(),
            OrgMemberInfo {
                org_id: "org_id_2".to_string(),
                org_name: "org_name_2".to_string(),
                url_safe_org_name: "org_name_2".to_string(),
                user_role: "Admin".to_string(),
                inherited_user_roles_plus_current_role: vec![
                    "Admin".to_string(),
                    "Member".to_string(),
                ],
                user_permissions: vec!["custom_permission_for_admin".to_string()],
            },
        );
        org_id_to_org_member_info.insert(
            "org_id_3".to_string(),
            OrgMemberInfo {
                org_id: "org_id_3".to_string(),
                org_name: "org_name_3".to_string(),
                url_safe_org_name: "org_name_3".to_string(),
                user_role: "Member".to_string(),
                inherited_user_roles_plus_current_role: vec!["Member".to_string()],
                user_permissions: vec!["custom_permission_for_member".to_string()],
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
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub org_id_to_org_member_info: HashMap<String, OrgMemberInfo>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub legacy_user_id: Option<String>,
    }
}
