use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
    model::OrganizationRole,
};

#[derive(Debug, Clone)]
pub struct AddMemberRequest {
    pub org_name: OwnerName,
    pub user_name: OwnerName,
    pub role: OrganizationRole,
    pub role_description: Option<String>,
}

impl AddMemberRequest {
    pub fn new(
        org_name: &str,
        user_name: &str,
        role: &str,
        role_description: Option<String>,
    ) -> Result<Self, OrganizationError> {
        let role = match role {
            "admin" => OrganizationRole::Admin,
            "member" => OrganizationRole::Member,
            _ => return Err(InputError::new("role", role).into()),
        };

        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| InputError::new("user name", e))?,
            role,
            role_description,
        })
    }
}
