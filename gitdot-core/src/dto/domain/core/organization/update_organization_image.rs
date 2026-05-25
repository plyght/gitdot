use bytes::Bytes;

use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
};

#[derive(Debug, Clone)]
pub struct UpdateOrganizationImageRequest {
    pub org_name: OwnerName,
    pub bytes: Bytes,
}

impl UpdateOrganizationImageRequest {
    pub fn new(org_name: &str, bytes: Bytes) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
            bytes,
        })
    }
}
