use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
};

#[derive(Debug, Clone)]
pub struct UpdateOrganizationMemberRequest {
    pub org_name: OwnerName,
    pub member_id: Uuid,
    pub role_description: Option<String>,
}

impl UpdateOrganizationMemberRequest {
    pub fn new(
        org_name: &str,
        member_id: Uuid,
        role_description: Option<String>,
    ) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
            member_id,
            role_description,
        })
    }
}
