use uuid::Uuid;

use crate::{
    dto::{FilterName, OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct CreateRepositoryCommitFilterRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub name: FilterName,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

impl CreateRepositoryCommitFilterRequest {
    pub fn new(
        user_id: Uuid,
        owner: &str,
        repo: &str,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            name: FilterName::try_new(name).map_err(|e| InputError::new("filter name", e))?,
            authors: normalize(authors),
            tags: normalize(tags),
            paths: normalize(paths),
        })
    }
}

fn normalize(values: Option<Vec<String>>) -> Option<Vec<String>> {
    values.map(|v| {
        v.into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    })
}
