use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::validate_api_key_response::{ValidateOrgApiKeyResponse, ValidatePersonalApiKeyResponse};
use crate::propelauth::errors::DetailedForbiddenError;
use crate::propelauth::options::{RequiredOrg, UserRequirementsInOrg};

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub struct LoginMethodForAccessToken {
    pub login_method: String,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub org_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SocialLoginType {
    Google,
    Github,
    Microsoft,
    Slack,
    Salesforce,
    Linkedin,
    Quickbooks,
    Xero,
}

impl std::str::FromStr for SocialLoginType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Google" => Ok(SocialLoginType::Google),
            "GitHub" => Ok(SocialLoginType::Github),
            "Microsoft" => Ok(SocialLoginType::Microsoft),
            "Slack" => Ok(SocialLoginType::Slack),
            "Salesforce" => Ok(SocialLoginType::Salesforce),
            "LinkedIn" => Ok(SocialLoginType::Linkedin),
            "QuickBooks Online" => Ok(SocialLoginType::Quickbooks),
            "Xero" => Ok(SocialLoginType::Xero),
            _ => Err("invalid social login type".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdentityProvider {
    Google,
    Rippling,
    OneLogin,
    JumpCloud,
    Okta,
    Azure,
    Duo,
    Generic,
}

impl std::str::FromStr for IdentityProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Google" => Ok(IdentityProvider::Google),
            "Rippling" => Ok(IdentityProvider::Rippling),
            "OneLogin" => Ok(IdentityProvider::OneLogin),
            "JumpCloud" => Ok(IdentityProvider::JumpCloud),
            "Okta" => Ok(IdentityProvider::Okta),
            "Azure" => Ok(IdentityProvider::Azure),
            "Duo" => Ok(IdentityProvider::Duo),
            "Generic" => Ok(IdentityProvider::Generic),
            _ => Err("invalid identity provider".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum LoginMethod {
    Password,
    MagicLink,
    SocialSso(SocialLoginType),
    EmailConfirmationLink,
    SamlSso(IdentityProvider, String),
    Impersonation,
    TokenGeneratedFromBackendApi,
    #[default]
    Unknown,
}

impl Into<LoginMethod> for LoginMethodForAccessToken {
    fn into(self) -> LoginMethod {
        match self.login_method.as_str() {
            "password" => LoginMethod::Password,
            "magic_link" => LoginMethod::MagicLink,
            "social_sso" => LoginMethod::SocialSso(
                self.provider
                    .expect("provider is required for social_sso login method")
                    .parse::<SocialLoginType>()
                    .expect("invalid social login type for social_sso login method"),
            ),
            "email_confirmation_link" => LoginMethod::EmailConfirmationLink,
            "saml_sso" => LoginMethod::SamlSso(
                self.provider
                    .expect("provider is required for saml_sso login method")
                    .parse::<IdentityProvider>()
                    .expect("invalid identity provider for saml_sso login method"),
                self.org_id
                    .expect("org_id is required for saml_sso login method"),
            ),
            "impersonation" => LoginMethod::Impersonation,
            "generated_from_backend_api" => LoginMethod::TokenGeneratedFromBackendApi,
            _ => LoginMethod::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct User {
    pub user_id: String,

    #[serde(default)]
    pub org_id_to_org_member_info: HashMap<String, OrgMemberInfo>,

    #[serde(default)]
    pub active_org_id: Option<String>,

    pub email: String,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub properties: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,

    /** If you used our migration APIs to migrate this user from a different system,
     *  this is their original ID from that system. */
    #[serde(default)]
    pub legacy_user_id: Option<String>,

    #[serde(default)]
    pub impersonator_user_id: Option<String>,

    #[serde(default)]
    pub login_method: LoginMethod,
}

impl User {
    pub fn validate_org_membership(
        &self,
        required_org: RequiredOrg,
        user_requirements_in_org: UserRequirementsInOrg,
    ) -> Result<OrgMemberInfo, DetailedForbiddenError> {
        let org_member_info = self
            .get_org(required_org)
            .ok_or(DetailedForbiddenError::UserIsNotInOrg)?;

        match user_requirements_in_org {
            UserRequirementsInOrg::None => Ok(org_member_info.clone()),
            UserRequirementsInOrg::IsRole(required_role) => {
                if org_member_info.is_role(required_role) {
                    Ok(org_member_info.clone())
                } else {
                    Err(DetailedForbiddenError::UserRoleDoesntMatch)
                }
            }
            UserRequirementsInOrg::IsAtLeastRole(minimum_required_role) => {
                if org_member_info.is_at_least_role(minimum_required_role) {
                    Ok(org_member_info.clone())
                } else {
                    Err(DetailedForbiddenError::UserRoleDoesntMatch)
                }
            }
            UserRequirementsInOrg::HasPermission(permission) => {
                if org_member_info.has_permission(permission) {
                    Ok(org_member_info.clone())
                } else {
                    Err(DetailedForbiddenError::UserMissingPermission)
                }
            }
            UserRequirementsInOrg::HasAllPermissions(permissions) => {
                if org_member_info.has_all_permissions(permissions) {
                    Ok(org_member_info.clone())
                } else {
                    Err(DetailedForbiddenError::UserMissingPermission)
                }
            }
        }
    }

    pub fn get_org(&self, org: RequiredOrg) -> Option<&OrgMemberInfo> {
        match org {
            RequiredOrg::OrgId(required_org_id) => {
                self.org_id_to_org_member_info.get(required_org_id)
            }
            RequiredOrg::OrgName(required_org_name) => {
                self.get_all_orgs().find(|org_member_info| {
                    org_member_info.org_name == required_org_name
                        || org_member_info.url_safe_org_name == required_org_name
                })
            }
        }
    }

    pub fn get_active_org(&self) -> Option<&OrgMemberInfo> {
        match &self.active_org_id {
            Some(org_id) => self.get_org(RequiredOrg::OrgId(org_id)),
            None => None,
        }
    }

    pub fn get_active_org_id(&self) -> Option<&String> {
        self.active_org_id.as_ref()
    }

    pub fn get_all_orgs(&self) -> Values<'_, String, OrgMemberInfo> {
        self.org_id_to_org_member_info.values()
    }

    pub fn get_all_org_ids(&self) -> Keys<'_, String, OrgMemberInfo> {
        self.org_id_to_org_member_info.keys()
    }

    pub fn get_num_orgs(&self) -> usize {
        self.org_id_to_org_member_info.len()
    }

    pub fn is_impersonated(&self) -> bool {
        self.impersonator_user_id.is_some()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrgMemberInfo {
    pub org_id: String,
    pub org_name: String,
    pub org_metadata: HashMap<String, Value>,
    pub url_safe_org_name: String,
    pub org_role_structure: OrgRoleStructure,
    pub user_role: String,
    pub inherited_user_roles_plus_current_role: Vec<String>,
    pub user_permissions: Vec<String>,
    pub additional_roles: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrgRoleStructure {
    #[default]
    #[serde(rename = "single_role_in_hierarchy")]
    SingleRoleInHierarchy,
    #[serde(rename = "multi_role")]
    MultiRole,
}

impl OrgMemberInfo {
    pub fn is_role(&self, role: &str) -> bool {
        match self.org_role_structure {
            OrgRoleStructure::SingleRoleInHierarchy => self.user_role == role,
            OrgRoleStructure::MultiRole => {
                self.user_role == role || self.additional_roles.iter().any(|r| r == role)
            }
        }
    }

    pub fn is_at_least_role(&self, role: &str) -> bool {
        match self.org_role_structure {
            OrgRoleStructure::SingleRoleInHierarchy => self
                .inherited_user_roles_plus_current_role
                .iter()
                .any(|r| r == role),
            OrgRoleStructure::MultiRole => {
                self.user_role == role || self.additional_roles.iter().any(|r| r == role)
            }
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        for user_permission in &self.user_permissions {
            if user_permission == permission {
                return true;
            }
        }
        false
    }

    pub fn has_all_permissions(&self, permissions: Vec<&str>) -> bool {
        // This is n^2, but for small number of permissions should be fine
        for permission in permissions {
            if !self.has_permission(permission) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserAndOrgMemberInfo {
    pub user: User,
    pub org_member_info: OrgMemberInfo,
}


#[derive(Debug)]
pub struct UserOrApiKey {
    pub user: Option<User>,
    pub user_key_info: Option<ValidatePersonalApiKeyResponse>,
    pub org_key_info: Option<ValidateOrgApiKeyResponse>,
}

impl UserOrApiKey {
    pub fn new() -> Self {
        Self {
            user: None,
            user_key_info: None,
            org_key_info: None,
        }
    }
}