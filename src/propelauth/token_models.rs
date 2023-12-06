use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::propelauth::errors::DetailedForbiddenError;
use crate::propelauth::options::{RequiredOrg, UserRequirementsInOrg};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct User {
    pub user_id: String,

    #[serde(default)]
    pub org_id_to_org_member_info: HashMap<String, OrgMemberInfo>,

    pub email: String,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,

    /** If you used our migration APIs to migrate this user from a different system,
     *  this is their original ID from that system. */
    #[serde(default)]
    pub legacy_user_id: Option<String>,

    #[serde(default)]
    pub impersonated_user_id: Option<String>,

    #[serde(default)]
    pub metadata: HashMap<String, String>,
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
        self.impersonated_user_id.is_some()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrgMemberInfo {
    pub org_id: String,
    pub org_name: String,
    pub org: HashMap<String, Value>,
    pub url_safe_org_name: String,
    pub user_role: String,
    pub inherited_user_roles_plus_current_role: Vec<String>,
    pub user_permissions: Vec<String>,
}

impl OrgMemberInfo {
    pub fn is_role(&self, role: &str) -> bool {
        self.user_role == role
    }

    pub fn is_at_least_role(&self, role: &str) -> bool {
        for user_role in &self.inherited_user_roles_plus_current_role {
            if user_role == role {
                return true;
            }
        }
        false
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
