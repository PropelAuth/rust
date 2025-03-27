use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use crate::models::{UserInOrg, UserMetadata};
use crate::propelauth::errors::DetailedForbiddenError;
use crate::propelauth::options::{RequiredOrg, UserRequirementsInOrg};

pub type OrgRole = String;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidateApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user: Option<UserMetadata>,
    pub org: Option<OrgMetadata>,
    pub user_in_org: Option<UserInOrg>,
    pub user_id: Option<String>,
    pub org_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidatePersonalApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user: UserMetadata,
    pub empty_map: HashMap<String, UserInOrg>,
}

impl ValidatePersonalApiKeyResponse {
    pub fn validate_org_membership(
        &self,
        required_org: RequiredOrg,
        user_requirements_in_org: UserRequirementsInOrg,
    ) -> Result<UserInOrg, DetailedForbiddenError> {
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

    pub fn get_org(&self, org: RequiredOrg) -> Option<&UserInOrg> {
        match org {
            RequiredOrg::OrgId(required_org_id) => {
                self.user.org_id_to_org_info.as_ref()?.get(required_org_id)
            }
            RequiredOrg::OrgName(required_org_name) => {
                self.get_all_orgs().find(|org_member_info| {
                    org_member_info.org_name == required_org_name
                })
            }
        }
    }

    pub fn get_all_orgs(&self) -> Values<'_, String, UserInOrg> {
        if let Some(org_id_to_org_info) = &self.user.org_id_to_org_info {
            org_id_to_org_info.values()
        } else {
            self.empty_map.values()
        }
    }

    pub fn get_all_org_ids(&self) -> Keys<'_, String, UserInOrg> {
        if let Some(org_id_to_org_info) = &self.user.org_id_to_org_info {
            org_id_to_org_info.keys()
        } else {
            self.empty_map.keys()
        }
    }

    pub fn get_num_orgs(&self) -> usize {
        if let Some(org_id_to_org_info) = &self.user.org_id_to_org_info {
            org_id_to_org_info.len()
        } else {
            0
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ValidateOrgApiKeyResponse {
    pub metadata: Option<serde_json::Value>,
    pub user: Option<UserMetadata>,
    pub org: OrgMetadata,
    pub user_in_org: Option<UserInOrg>,
}

impl ValidateOrgApiKeyResponse {
    pub fn validate_org_membership(
        &self,
        required_org: RequiredOrg,
    ) -> Result<bool, DetailedForbiddenError> {
        match required_org {
            RequiredOrg::OrgId(required_org_id) => {
                if self.org.org_id.to_string() == required_org_id {
                    Ok(true)
                } else {
                    Err(DetailedForbiddenError::UserIsNotInOrg)
                }
            }
            RequiredOrg::OrgName(required_org_name) => {
                if self.org.org_name == required_org_name {
                    Ok(true)
                } else {
                    Err(DetailedForbiddenError::UserIsNotInOrg)
                }
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OrgMetadata {
    pub org_id: Uuid,
    pub org_name: String,
    pub can_setup_saml: bool,
    pub max_users: Option<i32>,
    pub metadata: MetadataHashOfValue,
}

pub type MetadataHashOfValue = HashMap<String, Value>;

