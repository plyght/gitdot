use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient, GitHubClient, OctocrabClient},
    dto::{
        ProcessGithubInstallationRequest, ProcessGithubPushRequest, ProcessGithubPushResponse,
        SyncedRepositoryInfo,
    },
    error::WebhookError,
    repository::{
        GitHubRepository, MigrationRepository, PgGitHubRepository, PgMigrationRepository,
        PgRepositoryRepository, RepositoryRepository,
    },
    util::{git::ZERO_SHA, github::get_github_clone_url},
};

/// Handles inbound GitHub App webhook events that keep migrated repositories in
/// sync with their GitHub origin and track the App installation lifecycle.
#[async_trait]
pub trait GithubWebhookService: Send + Sync + 'static {
    /// Mirrors a GitHub push into any readonly destination repositories migrated
    /// from the origin.
    ///
    /// Only acts on pushes to the origin's default branch with a non-zero
    /// `after` sha; anything else returns an empty response without side
    /// effects. For each migration of the origin repository it resolves the
    /// destination, skips it unless it exists and is readonly, then fetches the
    /// pushed ref into the destination using an installation access token to
    /// authenticate the clone URL. Per-destination fetch failures are logged and
    /// skipped (not propagated); only successfully synced repositories are
    /// returned.
    ///
    /// # Errors
    /// - [`WebhookError::GitHubError`] if obtaining the installation access token fails.
    /// - [`WebhookError::DatabaseError`] if listing migrations or loading destinations fails.
    async fn process_github_push(
        &self,
        request: ProcessGithubPushRequest,
    ) -> Result<ProcessGithubPushResponse, WebhookError>;

    /// Reacts to GitHub App installation events.
    ///
    /// On the `deleted` action, removes all GitHub repository records tied to the
    /// installation id. Any other action is logged and ignored.
    ///
    /// # Errors
    /// - [`WebhookError::DatabaseError`] if deleting the installation's records fails.
    async fn process_github_installation(
        &self,
        request: ProcessGithubInstallationRequest,
    ) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone)]
pub struct GithubWebhookServiceImpl<R, MR, GHR, G, GH>
where
    R: RepositoryRepository,
    MR: MigrationRepository,
    GHR: GitHubRepository,
    G: GitClient,
    GH: GitHubClient,
{
    repo_repo: R,
    migration_repo: MR,
    github_repo: GHR,
    git_client: G,
    github_client: GH,
}

impl
    GithubWebhookServiceImpl<
        PgRepositoryRepository,
        PgMigrationRepository,
        PgGitHubRepository,
        Git2Client,
        OctocrabClient,
    >
{
    pub fn new(
        repo_repo: PgRepositoryRepository,
        migration_repo: PgMigrationRepository,
        github_repo: PgGitHubRepository,
        git_client: Git2Client,
        github_client: OctocrabClient,
    ) -> Self {
        Self {
            repo_repo,
            migration_repo,
            github_repo,
            git_client,
            github_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<R, MR, GHR, G, GH> GithubWebhookService for GithubWebhookServiceImpl<R, MR, GHR, G, GH>
where
    R: RepositoryRepository,
    MR: MigrationRepository,
    GHR: GitHubRepository,
    G: GitClient,
    GH: GitHubClient,
{
    async fn process_github_push(
        &self,
        request: ProcessGithubPushRequest,
    ) -> Result<ProcessGithubPushResponse, WebhookError> {
        let empty_response = ProcessGithubPushResponse {
            synced_repositories: Vec::new(),
        };

        let default_ref = format!("refs/heads/{}", request.repository.default_branch);
        if request.ref_name != default_ref {
            // only sync pushes to the default branch
            return Ok(empty_response);
        }
        if request.after == ZERO_SHA {
            return Ok(empty_response);
        }

        let origin_full_name = format!(
            "{}/{}",
            request.repository.owner.login, request.repository.name
        );
        let migration_repositories = self
            .migration_repo
            .list_by_origin_repository_id(request.repository.id)
            .await?;
        if migration_repositories.is_empty() {
            return Ok(empty_response);
        }

        let token = self
            .github_client
            .get_installation_access_token(request.installation.id as u64)
            .await?;
        let url = get_github_clone_url(&token, &origin_full_name);

        let mut synced_repositories = Vec::new();
        for migration_repository in migration_repositories {
            let Some(dest_id) = migration_repository.destination_repository_id else {
                continue;
            };
            let Some(dest) = self.repo_repo.get_by_id(dest_id, None).await? else {
                continue;
            };
            if !dest.readonly {
                continue;
            }

            match self
                .git_client
                .fetch_ref(
                    &dest.owner_name,
                    &dest.name,
                    &url,
                    &request.ref_name,
                    &request.after,
                )
                .await
            {
                Ok(()) => synced_repositories.push(SyncedRepositoryInfo {
                    owner_name: dest.owner_name,
                    repo_name: dest.name,
                    head_sha: request.after.clone(),
                }),
                Err(e) => {
                    tracing::error!(
                        ?e,
                        owner = %dest.owner_name,
                        repo = %dest.name,
                        "failed to fetch ref into readonly mirror",
                    );
                }
            }
        }

        Ok(ProcessGithubPushResponse {
            synced_repositories,
        })
    }

    async fn process_github_installation(
        &self,
        request: ProcessGithubInstallationRequest,
    ) -> Result<(), WebhookError> {
        match request.action.as_str() {
            "deleted" => {
                self.github_repo
                    .delete_by_installation_id(request.installation.id)
                    .await?;
            }
            other => {
                tracing::debug!(
                    action = other,
                    installation_id = request.installation.id,
                    "github installation event ignored",
                );
            }
        }
        Ok(())
    }
}
