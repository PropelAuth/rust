use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct User {
    pub user_id: String,

    #[serde(default)]
    pub org_id_to_org_member_info: HashMap<String, OrgMemberInfo>,

    /** If you used our migration APIs to migrate this user from a different system,
     *  this is their original ID from that system. */
    #[serde(default)]
    pub legacy_user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrgMemberInfo {
    pub org_id: String,
    pub org_name: String,
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
