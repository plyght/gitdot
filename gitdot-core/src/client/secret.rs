use async_trait::async_trait;
use google_cloud_secretmanager_v1::client::SecretManagerService;

use crate::error::SecretError;

/// Reads secrets from Google Cloud Secret Manager.
#[async_trait]
pub trait SecretClient: Send + Sync + Clone + 'static {
    /// Fetches the `latest` version of `secret_name` within the configured GCP
    /// project and returns its payload as a UTF-8 string.
    ///
    /// # Errors
    /// - [`SecretError::SecretManagerError`] — the access request failed (e.g.
    ///   missing secret or permissions).
    /// - [`SecretError::MissingPayload`] — the version has no payload.
    /// - [`SecretError::InvalidUtf8`] — the payload is not valid UTF-8.
    async fn get_secret(&self, secret_name: &str) -> Result<String, SecretError>;
}

#[derive(Clone)]
pub struct GoogleSecretClient {
    client: SecretManagerService,
    project_id: String,
}

impl GoogleSecretClient {
    pub async fn new(project_id: Option<String>) -> Result<Self, SecretError> {
        let client = SecretManagerService::builder().build().await?;
        let project_id = match project_id {
            Some(id) => id,
            None => google_cloud_metadata::project_id().await,
        };
        Ok(Self { client, project_id })
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SecretClient for GoogleSecretClient {
    async fn get_secret(&self, secret_name: &str) -> Result<String, SecretError> {
        let name = format!(
            "projects/{}/secrets/{}/versions/latest",
            self.project_id, secret_name
        );

        let response = self
            .client
            .access_secret_version()
            .set_name(&name)
            .send()
            .await?;

        let payload = response
            .payload
            .ok_or_else(|| SecretError::MissingPayload(secret_name.to_string()))?;

        Ok(String::from_utf8(payload.data.to_vec())?)
    }
}
