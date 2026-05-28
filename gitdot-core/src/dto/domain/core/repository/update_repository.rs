use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

const DESCRIPTION_MAX_LEN: usize = 255;

#[derive(Debug, Clone)]
pub struct UpdateRepositoryRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub description: Option<String>,
}

impl UpdateRepositoryRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        description: Option<String>,
    ) -> Result<Self, RepositoryError> {
        let description = description
            .map(|d| d.trim().to_string())
            .filter(|d| !d.is_empty());
        if let Some(d) = &description {
            if d.chars().count() > DESCRIPTION_MAX_LEN {
                return Err(InputError::new(
                    "description",
                    format!("must be at most {DESCRIPTION_MAX_LEN} characters"),
                )
                .into());
            }
        }

        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            description,
        })
    }
}
