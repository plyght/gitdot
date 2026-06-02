use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient, GitHubClient, OctocrabClient},
    dto::{
        CreateGitHubInstallationRequest, CreateGitHubInstallationResponse,
        CreateGitHubMigrationRequest, CreateGitHubMigrationResponse, GetGitHubAppInstallUrlRequest,
        GetGitHubAppInstallUrlResponse, GetMigrationRequest, GitHubInstallationResponse,
        ListGitHubInstallationRepositoriesRequest, ListGitHubInstallationRepositoriesResponse,
        ListGitHubInstallationsRequest, ListMigrationsRequest, MigrateGitHubRepositoriesRequest,
        MigrateGitHubRepositoriesResponse, MigratedRepositoryInfo, MigrationResponse, Page,
    },
    error::{ConflictError, GitHubError, InputError, MigrationError, OptionNotFoundExt},
    model::{
        GitHubInstallationType, MigrationOriginService, MigrationRepositoryStatus, MigrationStatus,
        Repository, RepositoryOwnerType, RepositoryVisibility,
    },
    repository::{
        GitHubRepository, MigrationRepository, OrganizationRepository, PgGitHubRepository,
        PgMigrationRepository, PgOrganizationRepository, PgRepositoryRepository, PgUserRepository,
        RepositoryRepository, UserRepository,
    },
    util::{
        cursor,
        git::{GitHookType, POST_RECEIVE_SCRIPT, PRE_RECEIVE_SCRIPT, PROC_RECEIVE_SCRIPT},
        github::get_github_clone_url,
    },
};

/// Importing repositories into gitdot from external services. Currently covers
/// the GitHub flow end to end: app installation, browsing installation repos,
/// planning a migration, and mirroring the selected repos into gitdot.
#[async_trait]
pub trait MigrationService: Send + Sync + 'static {
    /// Fetches a single migration by its per-user `number`, scoped to
    /// `request.user_id`.
    ///
    /// # Errors
    /// - [`MigrationError::NotFound`] if no such migration exists for the user.
    async fn get_migration(
        &self,
        request: GetMigrationRequest,
    ) -> Result<MigrationResponse, MigrationError>;

    /// Lists a user's migrations, cursor-paginated.
    async fn list_migrations(
        &self,
        request: ListMigrationsRequest,
    ) -> Result<Page<MigrationResponse>, MigrationError>;

    /// Completes a GitHub App installation OAuth callback and records it.
    ///
    /// Verifies the signed `state` and that its `user_id` matches
    /// `request.owner_id`, exchanges the OAuth `code` for an access token, and
    /// fetches the installation. For a user installation the GitHub login must
    /// match the installation account; for an organization installation the
    /// caller must be an active `admin` member. The caller's verified GitHub
    /// emails are upserted onto their gitdot user, then the installation is
    /// persisted. Returns the installation plus the original `action`.
    ///
    /// # Errors
    /// - [`GitHubError::InvalidState`] if the state's user does not match.
    /// - [`GitHubError::Unauthorized`] if the caller fails the user/org
    ///   ownership or admin checks.
    ///
    /// [`GitHubError::InvalidState`]: crate::error::GitHubError::InvalidState
    /// [`GitHubError::Unauthorized`]: crate::error::GitHubError::Unauthorized
    async fn create_github_installation(
        &self,
        request: CreateGitHubInstallationRequest,
    ) -> Result<CreateGitHubInstallationResponse, MigrationError>;

    /// Builds the GitHub App installation URL for the owner and action.
    async fn get_github_app_install_url(
        &self,
        request: GetGitHubAppInstallUrlRequest,
    ) -> Result<GetGitHubAppInstallUrlResponse, MigrationError>;

    /// Lists an owner's recorded GitHub App installations, cursor-paginated.
    async fn list_github_installations(
        &self,
        request: ListGitHubInstallationsRequest,
    ) -> Result<Page<GitHubInstallationResponse>, MigrationError>;

    /// Lists the repositories accessible to a GitHub App installation.
    ///
    /// First validates that the installation is owned by `request.owner_id`,
    /// then queries GitHub for the installation's repositories.
    ///
    /// # Errors
    /// - [`MigrationError::NotFound`] if the installation is not owned by the
    ///   caller (or does not exist).
    async fn list_github_installation_repositories(
        &self,
        request: ListGitHubInstallationRepositoriesRequest,
    ) -> Result<ListGitHubInstallationRepositoriesResponse, MigrationError>;

    /// Plans a GitHub migration, persisting the migration and its per-repo rows.
    ///
    /// Resolves the destination owner (the org by name for [`Organization`]
    /// destinations, otherwise the author). For each requested repository the
    /// matching GitHub repo determines visibility (private repos map to
    /// [`Private`]) and origin creation time, and a migration-repository row is
    /// recorded. This only records the plan; no mirroring happens yet.
    ///
    /// # Errors
    /// - [`MigrationError::NotFound`] if an organization destination does not
    ///   exist.
    /// - [`MigrationError::Input`] if a requested repository is not present in
    ///   the installation.
    ///
    /// [`Organization`]: crate::model::RepositoryOwnerType::Organization
    /// [`Private`]: crate::model::RepositoryVisibility::Private
    async fn create_github_migration(
        &self,
        request: CreateGitHubMigrationRequest,
    ) -> Result<CreateGitHubMigrationResponse, MigrationError>;

    /// Executes a planned migration, mirroring each repo into gitdot.
    ///
    /// Obtains an installation access token and marks the migration
    /// [`MigrationStatus::Running`]. Each repository is migrated concurrently in
    /// its own task: the per-repo status moves to [`Running`], the repo is
    /// mirror-cloned, git hooks are
    /// installed, and a gitdot repository row is created. On per-repo failure
    /// the partially mirrored repo is cleaned up and the row is marked [`Failed`]
    /// with the error message; on success it is marked [`Completed`] and linked
    /// to the new repository. The overall migration ends [`MigrationStatus::Completed`]
    /// only if every repo succeeded, otherwise [`MigrationStatus::Failed`]. The
    /// response contains info only for the repos that migrated successfully.
    ///
    /// A repo whose name already exists for the destination owner fails with a
    /// conflict recorded on that repo's row (it does not abort the others).
    ///
    /// [`Running`]: crate::model::MigrationRepositoryStatus::Running
    /// [`Failed`]: crate::model::MigrationRepositoryStatus::Failed
    /// [`Completed`]: crate::model::MigrationRepositoryStatus::Completed
    /// [`MigrationStatus::Running`]: crate::model::MigrationStatus::Running
    /// [`MigrationStatus::Completed`]: crate::model::MigrationStatus::Completed
    /// [`MigrationStatus::Failed`]: crate::model::MigrationStatus::Failed
    async fn migrate_github_repositories(
        &self,
        request: MigrateGitHubRepositoriesRequest,
    ) -> Result<MigrateGitHubRepositoriesResponse, MigrationError>;
}

#[derive(Debug, Clone)]
pub struct MigrationServiceImpl<G, GH, RR, MR, OR, GHR, UR>
where
    G: GitClient,
    GH: GitHubClient,
    RR: RepositoryRepository,
    MR: MigrationRepository,
    OR: OrganizationRepository,
    GHR: GitHubRepository,
    UR: UserRepository,
{
    git_client: G,
    github_client: GH,
    repo_repo: RR,
    migration_repo: MR,
    org_repo: OR,
    github_repo: GHR,
    user_repo: UR,
}

impl
    MigrationServiceImpl<
        Git2Client,
        OctocrabClient,
        PgRepositoryRepository,
        PgMigrationRepository,
        PgOrganizationRepository,
        PgGitHubRepository,
        PgUserRepository,
    >
{
    pub fn new(
        git_client: Git2Client,
        github_client: OctocrabClient,
        repo_repo: PgRepositoryRepository,
        migration_repo: PgMigrationRepository,
        org_repo: PgOrganizationRepository,
        github_repo: PgGitHubRepository,
        user_repo: PgUserRepository,
    ) -> Self {
        Self {
            git_client,
            github_client,
            repo_repo,
            migration_repo,
            org_repo,
            github_repo,
            user_repo,
        }
    }
}

impl<G, GH, RR, MR, OR, GHR, UR> MigrationServiceImpl<G, GH, RR, MR, OR, GHR, UR>
where
    G: GitClient,
    GH: GitHubClient,
    RR: RepositoryRepository,
    MR: MigrationRepository,
    OR: OrganizationRepository,
    GHR: GitHubRepository,
    UR: UserRepository,
{
    async fn migrate_single_repository(
        &self,
        owner_name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        full_name: &str,
        visibility: &RepositoryVisibility,
        token: &str,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<(Repository, Option<String>, String), MigrationError> {
        let repo_name = full_name
            .split('/')
            .nth(1)
            .ok_or_else(|| InputError::new("repository name", full_name))?;

        if self
            .repo_repo
            .get(owner_name, repo_name, None)
            .await?
            .is_some()
        {
            return Err(
                ConflictError::new("repository", format!("{}/{}", owner_name, repo_name)).into(),
            );
        }

        let clone_url = get_github_clone_url(token, full_name);
        self.git_client
            .mirror_repo(owner_name, repo_name, &clone_url)
            .await?;

        let result = self
            .setup_mirrored_repository(
                owner_name, repo_name, owner_id, owner_type, visibility, readonly, created_at,
            )
            .await;
        if result.is_err() {
            let _ = self.git_client.delete_repo(owner_name, repo_name).await;
        }

        result
    }

    async fn setup_mirrored_repository(
        &self,
        owner_name: &str,
        repo_name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<(Repository, Option<String>, String), MigrationError> {
        self.git_client.empty_hooks(owner_name, repo_name).await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::PreReceive,
                PRE_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::PostReceive,
                POST_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::ProcReceive,
                PROC_RECEIVE_SCRIPT,
            )
            .await?;

        let repository = self
            .repo_repo
            .create(
                repo_name, owner_id, owner_type, visibility, None, readonly, created_at,
            )
            .await?;

        let head_sha = self
            .git_client
            .get_repo_commit(owner_name, repo_name, "HEAD")
            .await
            .ok()
            .map(|c| c.sha);

        let default_ref_name = self
            .git_client
            .get_default_ref(owner_name, repo_name)
            .await
            .unwrap_or_else(|_| "refs/heads/main".to_string());

        Ok((repository, head_sha, default_ref_name))
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<G, GH, RR, MR, OR, GHR, UR> MigrationService
    for MigrationServiceImpl<G, GH, RR, MR, OR, GHR, UR>
where
    G: GitClient,
    GH: GitHubClient,
    RR: RepositoryRepository,
    MR: MigrationRepository,
    OR: OrganizationRepository,
    GHR: GitHubRepository,
    UR: UserRepository,
{
    async fn get_migration(
        &self,
        request: GetMigrationRequest,
    ) -> Result<MigrationResponse, MigrationError> {
        let migration = self
            .migration_repo
            .get(request.user_id, request.number)
            .await?
            .or_not_found("migration", request.number)?;

        Ok(migration.into())
    }

    async fn list_migrations(
        &self,
        request: ListMigrationsRequest,
    ) -> Result<Page<MigrationResponse>, MigrationError> {
        let (rows, next) = self
            .migration_repo
            .list(request.user_id, request.cursor, request.limit as i64)
            .await?;

        Ok(Page {
            data: rows.into_iter().map(MigrationResponse::from).collect(),
            next_cursor: next.as_ref().map(cursor::encode),
        })
    }

    async fn create_github_installation(
        &self,
        request: CreateGitHubInstallationRequest,
    ) -> Result<CreateGitHubInstallationResponse, MigrationError> {
        let payload = self.github_client.verify_install_state(&request.state)?;
        if payload.user_id != request.owner_id {
            return Err(GitHubError::InvalidState.into());
        }

        let access_token = self.github_client.exchange_code(&request.code).await?;
        let github_user_name = self.github_client.get_user(&access_token).await?.login;

        let installation = self
            .github_client
            .get_installation(request.installation_id as u64)
            .await?;
        let installation_type = match installation.target_type.as_deref() {
            Some("Organization") => GitHubInstallationType::Organization,
            _ => GitHubInstallationType::User,
        };

        match installation_type {
            GitHubInstallationType::User => {
                if github_user_name != installation.account.login {
                    return Err(GitHubError::Unauthorized.into());
                }
            }
            GitHubInstallationType::Organization => {
                let membership = self
                    .github_client
                    .get_user_membership(
                        &installation.account.login,
                        &github_user_name,
                        &access_token,
                    )
                    .await?;
                if membership.state != "active" || membership.role != "admin" {
                    return Err(GitHubError::Unauthorized.into());
                }
            }
        }

        let github_emails = self.github_client.get_user_emails(&access_token).await?;
        let verified_emails: Vec<String> = github_emails
            .into_iter()
            .filter(|e| e.verified)
            .map(|e| e.email)
            .collect();
        if !verified_emails.is_empty() {
            self.user_repo
                .upsert_verified_emails(request.owner_id, &verified_emails)
                .await?;
        }

        let installation = self
            .github_repo
            .create(
                request.installation_id,
                request.owner_id,
                installation_type,
                &installation.account.login,
            )
            .await?;

        Ok(CreateGitHubInstallationResponse {
            installation: installation.into(),
            action: payload.action,
        })
    }

    async fn get_github_app_install_url(
        &self,
        request: GetGitHubAppInstallUrlRequest,
    ) -> Result<GetGitHubAppInstallUrlResponse, MigrationError> {
        let install_url = self
            .github_client
            .get_github_app_install_url(request.owner_id, request.action)?;
        Ok(GetGitHubAppInstallUrlResponse { install_url })
    }

    async fn list_github_installations(
        &self,
        request: ListGitHubInstallationsRequest,
    ) -> Result<Page<GitHubInstallationResponse>, MigrationError> {
        let (installations, next_cursor) = self
            .github_repo
            .list_by_owner(request.owner_id, request.cursor, request.limit as i64)
            .await?;
        Ok(Page {
            data: installations.into_iter().map(Into::into).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_github_installation_repositories(
        &self,
        request: ListGitHubInstallationRepositoriesRequest,
    ) -> Result<ListGitHubInstallationRepositoriesResponse, MigrationError> {
        // validate that the installation is owned by the user
        self.github_repo
            .get(request.owner_id, request.installation_id)
            .await?
            .or_not_found("github installation", request.installation_id)?;

        let repos = self
            .github_client
            .list_installation_repositories(request.installation_id as u64)
            .await?;

        Ok(repos.repositories.into_iter().map(Into::into).collect())
    }

    async fn create_github_migration(
        &self,
        request: CreateGitHubMigrationRequest,
    ) -> Result<CreateGitHubMigrationResponse, MigrationError> {
        // validate that the installation is owned by the user
        self.github_repo
            .get(request.user_id, request.installation_id)
            .await?
            .or_not_found("github installation", request.installation_id)?;

        let owner_id = match request.destination_type {
            RepositoryOwnerType::User => request.user_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.destination.as_ref())
                    .await?
                    .or_not_found("owner", request.destination.as_ref())?;
                org.id
            }
        };

        let github_repos = self
            .github_client
            .list_installation_repositories(request.installation_id as u64)
            .await?;

        let migration = self
            .migration_repo
            .create(
                request.user_id,
                MigrationOriginService::GitHub,
                &request.origin,
                &request.origin_type,
                request.destination.as_ref(),
                &request.destination_type,
            )
            .await?;

        let mut repositories = Vec::new();
        for repo in &request.repositories {
            let name = repo.name.as_ref();
            let origin_full_name = format!("{}/{}", request.origin, name);
            let destination_full_name = format!("{}/{}", request.destination.as_ref(), name);
            let github_repo = github_repos
                .repositories
                .iter()
                .find(|r| r.full_name.as_deref() == Some(origin_full_name.as_str()))
                .ok_or_else(|| InputError::new("repository", &origin_full_name))?;
            let visibility = if github_repo.private.unwrap_or(false) {
                RepositoryVisibility::Private
            } else {
                RepositoryVisibility::Public
            };
            let origin_created_at = github_repo.created_at;
            let migration_repository = self
                .migration_repo
                .create_migration_repository(
                    migration.id,
                    &origin_full_name,
                    repo.id,
                    origin_created_at,
                    &destination_full_name,
                    &visibility,
                )
                .await?;
            repositories.push(migration_repository);
        }

        let mut migration = migration;
        migration.repositories = Some(repositories);

        Ok(CreateGitHubMigrationResponse {
            migration,
            owner_id,
            owner_name: request.destination,
            owner_type: request.destination_type,
        })
    }

    async fn migrate_github_repositories(
        &self,
        request: MigrateGitHubRepositoriesRequest,
    ) -> Result<MigrateGitHubRepositoriesResponse, MigrationError> {
        let token = self
            .github_client
            .get_installation_access_token(request.installation_id as u64)
            .await?;

        self.migration_repo
            .update_status(request.migration_id, MigrationStatus::Running)
            .await?;

        let readonly = request.readonly;
        let mut handles = Vec::new();
        for migration_repo_entry in request.migration_repositories {
            let service = self.clone();
            let token = token.clone();
            let owner_name = request.owner_name.to_string();
            let owner_id = request.owner_id;
            let owner_type = request.owner_type.clone();
            let migration_repo_id = migration_repo_entry.id;
            let full_name = migration_repo_entry.origin_full_name.clone();
            let visibility = migration_repo_entry.visibility.clone();
            let created_at = migration_repo_entry.origin_created_at;

            let handle = tokio::spawn(async move {
                let _ = service
                    .migration_repo
                    .update_migration_repository_status(
                        migration_repo_id,
                        MigrationRepositoryStatus::Running,
                        None,
                    )
                    .await;

                match service
                    .migrate_single_repository(
                        &owner_name,
                        owner_id,
                        &owner_type,
                        &full_name,
                        &visibility,
                        &token,
                        readonly,
                        created_at,
                    )
                    .await
                {
                    Ok((repository, head_sha, default_ref_name)) => {
                        let _ = service
                            .migration_repo
                            .update_migration_repository_status(
                                migration_repo_id,
                                MigrationRepositoryStatus::Completed,
                                None,
                            )
                            .await;
                        let _ = service
                            .migration_repo
                            .set_destination_repository_id(migration_repo_id, repository.id)
                            .await;

                        Some(MigratedRepositoryInfo {
                            owner_name: owner_name.clone(),
                            repo_name: repository.name,
                            default_ref_name,
                            head_sha,
                        })
                    }
                    Err(e) => {
                        let _ = service
                            .migration_repo
                            .update_migration_repository_status(
                                migration_repo_id,
                                MigrationRepositoryStatus::Failed,
                                Some(&e.to_string()),
                            )
                            .await;
                        None
                    }
                }
            });

            handles.push(handle);
        }

        let mut migrated = Vec::new();
        let mut all_succeeded = true;
        for handle in handles {
            match handle.await {
                Ok(Some(info)) => migrated.push(info),
                Ok(None) => all_succeeded = false,
                Err(_) => all_succeeded = false,
            }
        }

        let final_status = if all_succeeded {
            MigrationStatus::Completed
        } else {
            MigrationStatus::Failed
        };

        self.migration_repo
            .update_status(request.migration_id, final_status)
            .await?;

        Ok(MigrateGitHubRepositoriesResponse {
            migrated_repositories: migrated,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::json;
    use uuid::Uuid;

    use super::{MigrationService, MigrationServiceImpl};
    use crate::{
        dto::{
            CreateGitHubInstallationRequest, CreateGitHubMigrationRequest, Cursor,
            GetGitHubAppInstallUrlRequest, GetMigrationRequest, GitHubAppInstallAction,
            GitHubEmail, GitHubMembership, GitHubUser, InstallStatePayload,
            ListGitHubInstallationRepositoriesRequest, ListGitHubInstallationsRequest,
            ListMigrationsRequest,
        },
        error::{GitHubError, MigrationError},
        model::{
            GitHubInstallationType, MigrationStatus, RepositoryOwnerType, RepositoryVisibility,
        },
        service::{
            test_client::{MockGitClient, MockGitHubClient},
            test_common::{create_github_installation, create_migration},
            test_repository::{
                MockGitHubRepository, MockMigrationRepository, MockOrganizationRepository,
                MockRepositoryRepository, MockUserRepository,
            },
        },
    };

    type Service = MigrationServiceImpl<
        MockGitClient,
        MockGitHubClient,
        MockRepositoryRepository,
        MockMigrationRepository,
        MockOrganizationRepository,
        MockGitHubRepository,
        MockUserRepository,
    >;

    fn create_service() -> Service {
        MigrationServiceImpl {
            git_client: MockGitClient::default(),
            github_client: MockGitHubClient::new(),
            repo_repo: MockRepositoryRepository::new(),
            migration_repo: MockMigrationRepository::default(),
            org_repo: MockOrganizationRepository::new(),
            github_repo: MockGitHubRepository::new(),
            user_repo: MockUserRepository::new(),
        }
    }

    fn cursor() -> Cursor {
        Cursor {
            created_at: Utc::now(),
            id: Uuid::new_v4(),
        }
    }

    fn octocrab_installation(
        account_login: &str,
        target_type: Option<&str>,
    ) -> octocrab::models::Installation {
        let account = json!({
            "login": account_login,
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "avatar_url": "https://example.com/a.png",
            "gravatar_id": "",
            "url": "https://api.github.com/users/x",
            "html_url": "https://github.com/x",
            "followers_url": "https://api.github.com/users/x/followers",
            "following_url": "https://api.github.com/users/x/following",
            "gists_url": "https://api.github.com/users/x/gists",
            "starred_url": "https://api.github.com/users/x/starred",
            "subscriptions_url": "https://api.github.com/users/x/subscriptions",
            "organizations_url": "https://api.github.com/users/x/orgs",
            "repos_url": "https://api.github.com/users/x/repos",
            "events_url": "https://api.github.com/users/x/events",
            "received_events_url": "https://api.github.com/users/x/received_events",
            "type": "User",
            "site_admin": false,
        });
        serde_json::from_value(json!({
            "id": 12345,
            "account": account,
            "permissions": {},
            "events": [],
            "target_type": target_type,
        }))
        .unwrap()
    }

    fn installation_repositories(
        repos: &[(&str, bool)],
    ) -> octocrab::models::InstallationRepositories {
        let repositories: Vec<_> = repos
            .iter()
            .map(|&(full_name, private)| {
                let name = full_name.split('/').nth(1).unwrap_or(full_name);
                json!({
                    "id": 100,
                    "name": name,
                    "url": "https://api.github.com/repos/o/r",
                    "full_name": full_name,
                    "private": private,
                    "default_branch": "main",
                })
            })
            .collect();
        serde_json::from_value(json!({
            "total_count": repositories.len(),
            "repositories": repositories,
        }))
        .unwrap()
    }

    fn install_request(owner_id: Uuid) -> CreateGitHubInstallationRequest {
        CreateGitHubInstallationRequest::new(12345, owner_id, "state".into(), "code".into())
    }

    mod get_migration {
        use super::*;

        #[tokio::test]
        async fn found_returns_response() {
            let mut service = create_service();
            let migration = create_migration(MigrationStatus::Completed);
            let id = migration.id;
            service.migration_repo =
                MockMigrationRepository::default().with_migration(Some(migration));

            let response = service
                .get_migration(GetMigrationRequest::new(Uuid::new_v4(), 1))
                .await
                .unwrap();

            assert_eq!(response.id, id);
            assert_eq!(response.number, 1);
        }

        #[tokio::test]
        async fn missing_is_not_found() {
            // The default mock returns no migration.
            let service = create_service();

            let err = service
                .get_migration(GetMigrationRequest::new(Uuid::new_v4(), 99))
                .await
                .unwrap_err();

            assert!(matches!(err, MigrationError::NotFound(_)));
        }
    }

    mod list_migrations {
        use super::*;

        #[tokio::test]
        async fn rows_with_next_cursor() {
            let mut service = create_service();
            service.migration_repo = MockMigrationRepository::default().with_list(
                vec![create_migration(MigrationStatus::Pending)],
                Some(cursor()),
            );

            let response = service
                .list_migrations(ListMigrationsRequest::new(Uuid::new_v4(), None, None).unwrap())
                .await
                .unwrap();

            assert_eq!(response.data.len(), 1);
            assert!(response.next_cursor.is_some());
        }

        #[tokio::test]
        async fn empty_page() {
            // The default mock returns an empty list with no cursor.
            let service = create_service();

            let response = service
                .list_migrations(ListMigrationsRequest::new(Uuid::new_v4(), None, None).unwrap())
                .await
                .unwrap();

            assert!(response.data.is_empty());
            assert!(response.next_cursor.is_none());
        }
    }

    mod get_github_app_install_url {
        use super::*;

        #[tokio::test]
        async fn delegates_to_client() {
            let url = "https://github.com/apps/gitdot/installations/new";
            let mut service = create_service();
            service
                .github_client
                .expect_get_github_app_install_url()
                .returning(move |_, _| Ok(url.to_string()));

            let response = service
                .get_github_app_install_url(
                    GetGitHubAppInstallUrlRequest::new(Uuid::new_v4(), "migration").unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.install_url, url);
        }
    }

    mod list_github_installations {
        use super::*;

        #[tokio::test]
        async fn rows_with_next_cursor() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            service
                .github_repo
                .expect_list_by_owner()
                .returning(move |_, _, _| {
                    Ok((
                        vec![create_github_installation(
                            owner_id,
                            "octocat",
                            GitHubInstallationType::User,
                        )],
                        Some(cursor()),
                    ))
                });

            let response = service
                .list_github_installations(
                    ListGitHubInstallationsRequest::new(owner_id, None, None).unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.data.len(), 1);
            assert!(response.next_cursor.is_some());
        }
    }

    mod create_github_installation {
        use super::*;

        #[tokio::test]
        async fn state_user_mismatch_is_invalid_state() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            // payload binds the flow to a *different* user.
            service
                .github_client
                .expect_verify_install_state()
                .returning(|_| {
                    Ok(InstallStatePayload {
                        user_id: Uuid::new_v4(),
                        action: GitHubAppInstallAction::Migration,
                        exp: 9_999_999_999,
                    })
                });

            let err = service
                .create_github_installation(install_request(owner_id))
                .await
                .unwrap_err();

            assert!(matches!(
                err,
                MigrationError::GitHubError(GitHubError::InvalidState)
            ));
        }

        #[tokio::test]
        async fn user_install_login_mismatch_is_unauthorized() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            service
                .github_client
                .expect_verify_install_state()
                .returning(move |_| {
                    Ok(InstallStatePayload {
                        user_id: owner_id,
                        action: GitHubAppInstallAction::Migration,
                        exp: 9_999_999_999,
                    })
                });
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("token".to_string()));
            service.github_client.expect_get_user().returning(|_| {
                Ok(GitHubUser {
                    login: "alice".to_string(),
                })
            });
            // installation account is someone else.
            service
                .github_client
                .expect_get_installation()
                .returning(|_| Ok(octocrab_installation("bob", None)));

            let err = service
                .create_github_installation(install_request(owner_id))
                .await
                .unwrap_err();

            assert!(matches!(
                err,
                MigrationError::GitHubError(GitHubError::Unauthorized)
            ));
        }

        #[tokio::test]
        async fn org_install_non_admin_is_unauthorized() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            service
                .github_client
                .expect_verify_install_state()
                .returning(move |_| {
                    Ok(InstallStatePayload {
                        user_id: owner_id,
                        action: GitHubAppInstallAction::Migration,
                        exp: 9_999_999_999,
                    })
                });
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("token".to_string()));
            service.github_client.expect_get_user().returning(|_| {
                Ok(GitHubUser {
                    login: "alice".to_string(),
                })
            });
            service
                .github_client
                .expect_get_installation()
                .returning(|_| Ok(octocrab_installation("acme", Some("Organization"))));
            service
                .github_client
                .expect_get_user_membership()
                .returning(|_, _, _| {
                    Ok(GitHubMembership {
                        state: "active".to_string(),
                        role: "member".to_string(),
                    })
                });

            let err = service
                .create_github_installation(install_request(owner_id))
                .await
                .unwrap_err();

            assert!(matches!(
                err,
                MigrationError::GitHubError(GitHubError::Unauthorized)
            ));
        }

        #[tokio::test]
        async fn user_install_with_verified_emails_persists() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            service
                .github_client
                .expect_verify_install_state()
                .returning(move |_| {
                    Ok(InstallStatePayload {
                        user_id: owner_id,
                        action: GitHubAppInstallAction::Migration,
                        exp: 9_999_999_999,
                    })
                });
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("token".to_string()));
            service.github_client.expect_get_user().returning(|_| {
                Ok(GitHubUser {
                    login: "octocat".to_string(),
                })
            });
            service
                .github_client
                .expect_get_installation()
                .returning(|_| Ok(octocrab_installation("octocat", None)));
            service
                .github_client
                .expect_get_user_emails()
                .returning(|_| {
                    Ok(vec![GitHubEmail {
                        email: "octocat@example.com".to_string(),
                        primary: true,
                        verified: true,
                    }])
                });
            service
                .user_repo
                .expect_upsert_verified_emails()
                .times(1)
                .returning(|_, _| Ok(()));
            service
                .github_repo
                .expect_create()
                .returning(move |_, _, _, _| {
                    Ok(create_github_installation(
                        owner_id,
                        "octocat",
                        GitHubInstallationType::User,
                    ))
                });

            let response = service
                .create_github_installation(install_request(owner_id))
                .await
                .unwrap();

            assert_eq!(response.installation.github_login, "octocat");
            assert!(matches!(response.action, GitHubAppInstallAction::Migration));
        }

        #[tokio::test]
        async fn no_verified_emails_skips_upsert() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            service
                .github_client
                .expect_verify_install_state()
                .returning(move |_| {
                    Ok(InstallStatePayload {
                        user_id: owner_id,
                        action: GitHubAppInstallAction::Migration,
                        exp: 9_999_999_999,
                    })
                });
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("token".to_string()));
            service.github_client.expect_get_user().returning(|_| {
                Ok(GitHubUser {
                    login: "octocat".to_string(),
                })
            });
            service
                .github_client
                .expect_get_installation()
                .returning(|_| Ok(octocrab_installation("octocat", None)));
            // all unverified → upsert must not be called (no expectation set;
            // mockall panics on an unexpected call).
            service
                .github_client
                .expect_get_user_emails()
                .returning(|_| {
                    Ok(vec![GitHubEmail {
                        email: "octocat@example.com".to_string(),
                        primary: true,
                        verified: false,
                    }])
                });
            service
                .github_repo
                .expect_create()
                .returning(move |_, _, _, _| {
                    Ok(create_github_installation(
                        owner_id,
                        "octocat",
                        GitHubInstallationType::User,
                    ))
                });

            service
                .create_github_installation(install_request(owner_id))
                .await
                .unwrap();
        }
    }

    mod list_github_installation_repositories {
        use super::*;

        #[tokio::test]
        async fn not_owned_is_not_found() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            // github_client is never reached.
            service.github_repo.expect_get().returning(|_, _| Ok(None));

            let err = service
                .list_github_installation_repositories(ListGitHubInstallationRepositoriesRequest {
                    owner_id,
                    installation_id: 12345,
                })
                .await
                .unwrap_err();

            assert!(matches!(err, MigrationError::NotFound(_)));
        }

        #[tokio::test]
        async fn owned_maps_repositories() {
            let owner_id = Uuid::new_v4();
            let mut service = create_service();
            service.github_repo.expect_get().returning(move |_, _| {
                Ok(Some(create_github_installation(
                    owner_id,
                    "octocat",
                    GitHubInstallationType::User,
                )))
            });
            service
                .github_client
                .expect_list_installation_repositories()
                .returning(|_| Ok(installation_repositories(&[("octocat/hello", false)])));

            let response = service
                .list_github_installation_repositories(ListGitHubInstallationRepositoriesRequest {
                    owner_id,
                    installation_id: 12345,
                })
                .await
                .unwrap();

            assert_eq!(response.len(), 1);
            assert_eq!(response[0].name, "hello");
            assert_eq!(response[0].full_name, "octocat/hello");
            assert!(!response[0].private);
        }
    }

    mod create_github_migration {
        use super::*;

        fn migration_request(
            user_id: Uuid,
            destination: &str,
            destination_type: &str,
        ) -> CreateGitHubMigrationRequest {
            CreateGitHubMigrationRequest::new(
                user_id,
                12345,
                "octocat",
                "user",
                destination,
                destination_type,
                vec![("myrepo".to_string(), 100)],
            )
            .unwrap()
        }

        #[tokio::test]
        async fn installation_not_owned_is_not_found() {
            let mut service = create_service();
            service.github_repo.expect_get().returning(|_, _| Ok(None));

            let err = service
                .create_github_migration(migration_request(Uuid::new_v4(), "octocat", "user"))
                .await
                .unwrap_err();

            assert!(matches!(err, MigrationError::NotFound(_)));
        }

        #[tokio::test]
        async fn missing_org_destination_is_not_found() {
            let user_id = Uuid::new_v4();
            let mut service = create_service();
            service.github_repo.expect_get().returning(move |_, _| {
                Ok(Some(create_github_installation(
                    user_id,
                    "octocat",
                    GitHubInstallationType::User,
                )))
            });
            service.org_repo.expect_get().returning(|_| Ok(None));

            let err = service
                .create_github_migration(migration_request(user_id, "acme", "organization"))
                .await
                .unwrap_err();

            assert!(matches!(err, MigrationError::NotFound(_)));
        }

        #[tokio::test]
        async fn repo_absent_from_installation_is_input() {
            let user_id = Uuid::new_v4();
            let mut service = create_service();
            service.github_repo.expect_get().returning(move |_, _| {
                Ok(Some(create_github_installation(
                    user_id,
                    "octocat",
                    GitHubInstallationType::User,
                )))
            });
            // installation has a different repo than the one requested.
            // (the migration row is created before the per-repo loop runs; the
            // default mock's `create` handles that.)
            service
                .github_client
                .expect_list_installation_repositories()
                .returning(|_| Ok(installation_repositories(&[("octocat/other", false)])));

            let err = service
                .create_github_migration(migration_request(user_id, "octocat", "user"))
                .await
                .unwrap_err();

            assert!(matches!(err, MigrationError::Input(_)));
        }

        #[tokio::test]
        async fn private_repo_plans_migration_with_private_visibility() {
            let user_id = Uuid::new_v4();
            let mut service = create_service();
            service.github_repo.expect_get().returning(move |_, _| {
                Ok(Some(create_github_installation(
                    user_id,
                    "octocat",
                    GitHubInstallationType::User,
                )))
            });
            // The default mock's `create` + `create_migration_repository` handle
            // persistence; the latter records the derived visibility.
            service
                .github_client
                .expect_list_installation_repositories()
                .returning(|_| Ok(installation_repositories(&[("octocat/myrepo", true)])));

            let response = service
                .create_github_migration(migration_request(user_id, "octocat", "user"))
                .await
                .unwrap();

            assert_eq!(response.owner_id, user_id);
            assert_eq!(response.owner_type, RepositoryOwnerType::User);
            assert_eq!(response.migration.repositories.unwrap().len(), 1);
            // GitHub-private repo → gitdot `Private` visibility.
            assert_eq!(
                service.migration_repo.created_visibility(),
                Some(RepositoryVisibility::Private)
            );
        }
    }
}
