use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{InputError, MigrationError};

#[derive(Debug, Clone)]
pub struct GetGitHubAppInstallUrlRequest {
    pub owner_id: Uuid,
    pub action: GitHubAppInstallAction,
}

impl GetGitHubAppInstallUrlRequest {
    pub fn new(owner_id: Uuid, action: &str) -> Result<Self, MigrationError> {
        Ok(Self {
            owner_id,
            action: GitHubAppInstallAction::try_from_str(action)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GitHubAppInstallAction {
    Migration,
    Onboarding,
}

impl GitHubAppInstallAction {
    pub fn try_from_str(s: &str) -> Result<Self, MigrationError> {
        match s {
            "migration" => Ok(Self::Migration),
            "onboarding" => Ok(Self::Onboarding),
            other => Err(InputError::new("action", format!("invalid: {other}")).into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetGitHubAppInstallUrlResponse {
    pub install_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallStatePayload {
    pub user_id: Uuid,
    pub action: GitHubAppInstallAction,
    pub exp: i64,
}
