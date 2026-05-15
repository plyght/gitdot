use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct ListRepositoryCommitFiltersRequest {
    pub user_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl ListRepositoryCommitFiltersRequest {
    pub fn new(user_id: Option<Uuid>, owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}
