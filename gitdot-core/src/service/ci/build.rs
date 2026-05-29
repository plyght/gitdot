use std::collections::HashMap;

use async_trait::async_trait;
use futures::future::try_join_all;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient, S2Client, S2ClientImpl},
    dto::{
        BuildResponse, CiConfig, CreateBuildRequest, ListBuildsRequest, Page, TaskResponse,
    },
    error::{BuildError, GitError, NotFoundError, OptionNotFoundExt},
    model::{BuildStatus, TaskStatus},
    repository::{
        BuildRepository, PgBuildRepository, PgRepositoryRepository, PgTaskRepository,
        RepositoryRepository, TaskRepository,
    },
    util::{cursor, git::DEFAULT_BRANCH},
};

/// CI builds: creating a build for a commit from the repository's
/// `.gitdot-ci.toml`, fanning it out into tasks, and reporting build status
/// aggregated from its tasks.
#[async_trait]
pub trait BuildService: Send + Sync + 'static {
    /// Creates a build for `request.commit_sha` and fans it out into tasks.
    ///
    /// Resolves the commit, reads `.gitdot-ci.toml` at that commit, and parses
    /// it. The trigger is [`PushToMain`] when `request.ref_name` is (or ends in)
    /// the default branch, otherwise [`PullRequest`]; the matching build config
    /// selects which tasks to create. UUIDs are pre-generated for every task so
    /// `waits_for` dependencies can reference each other by ID. Each task gets a
    /// dedicated S2 log stream and is created [`Blocked`] if it has
    /// dependencies, otherwise [`Pending`]. The returned response always reports
    /// [`Running`] with `completed_tasks` of 0.
    ///
    /// # Errors
    /// - [`BuildError::NotFound`] if the repository, commit, or `.gitdot-ci.toml`
    ///   (missing, or a folder) is absent.
    /// - [`BuildError::InvalidConfig`] if the config cannot be parsed or has no
    ///   matching build config for the trigger.
    /// - [`BuildError::S2Error`] if a task log stream cannot be created.
    ///
    /// [`PushToMain`]: gitdot_config::ci::BuildTrigger::PushToMain
    /// [`PullRequest`]: gitdot_config::ci::BuildTrigger::PullRequest
    /// [`Blocked`]: crate::model::TaskStatus::Blocked
    /// [`Pending`]: crate::model::TaskStatus::Pending
    /// [`Running`]: crate::model::BuildStatus::Running
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError>;

    /// Lists builds for a repository, newest first, cursor-paginated.
    ///
    /// # Errors
    /// - [`BuildError::NotFound`] if the repository does not exist.
    async fn list_builds(
        &self,
        request: ListBuildsRequest,
    ) -> Result<Page<BuildResponse>, BuildError>;

    /// Fetches a single build by its per-repository `number`.
    ///
    /// Build status and `updated_at` are derived from the build's tasks:
    /// [`Failure`] if any task failed, [`Success`] if all tasks succeeded,
    /// otherwise (including a build with no tasks) [`Running`]; `updated_at` is
    /// the latest task update, falling back to the build's creation time.
    ///
    /// # Errors
    /// - [`BuildError::NotFound`] if the repository or build does not exist.
    ///
    /// [`Failure`]: crate::model::BuildStatus::Failure
    /// [`Success`]: crate::model::BuildStatus::Success
    /// [`Running`]: crate::model::BuildStatus::Running
    async fn get_build(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<BuildResponse, BuildError>;

    /// Lists all tasks belonging to a build identified by its `number`.
    ///
    /// # Errors
    /// - [`BuildError::NotFound`] if the repository or build does not exist.
    async fn list_build_tasks(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Vec<TaskResponse>, BuildError>;
}

#[derive(Debug, Clone)]
pub struct BuildServiceImpl<G, S, B, T, R>
where
    G: GitClient,
    S: S2Client,
    B: BuildRepository,
    T: TaskRepository,
    R: RepositoryRepository,
{
    git_client: G,
    s2_client: S,
    build_repo: B,
    task_repo: T,
    repo_repo: R,
}

impl
    BuildServiceImpl<
        Git2Client,
        S2ClientImpl,
        PgBuildRepository,
        PgTaskRepository,
        PgRepositoryRepository,
    >
{
    pub fn new(
        git_client: Git2Client,
        s2_client: S2ClientImpl,
        build_repo: PgBuildRepository,
        task_repo: PgTaskRepository,
        repo_repo: PgRepositoryRepository,
    ) -> Self {
        Self {
            git_client,
            s2_client,
            build_repo,
            task_repo,
            repo_repo,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<G, S, B, T, R> BuildService for BuildServiceImpl<G, S, B, T, R>
where
    G: GitClient,
    S: S2Client,
    B: BuildRepository,
    T: TaskRepository,
    R: RepositoryRepository,
{
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError> {
        let owner = request.repo_owner.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;
        let commit = self
            .git_client
            .get_repo_commit(owner, repo, &request.commit_sha)
            .await
            .map_err(BuildError::GitError)?;
        let resolved_sha = commit.sha.clone();

        let blob = self
            .git_client
            .get_repo_blob(owner, repo, &resolved_sha, ".gitdot-ci.toml")
            .await
            .map_err(|e: GitError| match e {
                GitError::NotFound(_) => {
                    BuildError::NotFound(NotFoundError::new("config", request.commit_sha.clone()))
                }
                other => BuildError::GitError(other),
            })?;

        let file_content = blob.content;

        let ci_config =
            CiConfig::new(&file_content).map_err(|e| BuildError::InvalidConfig(e.to_string()))?;
        let ci_trigger = if request.ref_name == DEFAULT_BRANCH
            || request
                .ref_name
                .rsplit('/')
                .next()
                .map(|name| name == DEFAULT_BRANCH)
                .unwrap_or(false)
        {
            gitdot_config::ci::BuildTrigger::PushToMain
        } else {
            gitdot_config::ci::BuildTrigger::PullRequest
        };

        let build_config = ci_config
            .get_build_config(&ci_trigger)
            .map_err(|e| BuildError::InvalidConfig(e.to_string()))?;
        let build = self
            .build_repo
            .create(
                repository.id,
                crate::model::BuildTrigger::from(ci_trigger),
                &resolved_sha,
                &request.ref_name,
            )
            .await?;

        // pre-generate UUIDs for all tasks so dependencies can reference each other by ID
        let mut name_to_id: HashMap<String, Uuid> = HashMap::new();
        let task_configs = ci_config.get_task_configs(build_config);
        for task_config in &task_configs {
            name_to_id.insert(task_config.name.clone(), Uuid::new_v4());
        }

        let s2_client = &self.s2_client;
        let task_repo = &self.task_repo;
        let repository_id = repository.id;

        let task_futures = task_configs.iter().map(|task_config| {
            let id = name_to_id[&task_config.name];
            let waits_for: Vec<Uuid> = task_config
                .waits_for
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .filter_map(|dep_name| name_to_id.get(dep_name).copied())
                .collect();
            let status = if waits_for.is_empty() {
                TaskStatus::Pending
            } else {
                TaskStatus::Blocked
            };

            async move {
                let s2_uri = s2_client
                    .create_stream(owner, repo, id)
                    .await
                    .map_err(BuildError::S2Error)?;

                task_repo
                    .create(
                        id,
                        repository_id,
                        &task_config.name,
                        &task_config.command,
                        build.id,
                        &s2_uri,
                        status,
                        &waits_for,
                    )
                    .await
                    .map_err(BuildError::DatabaseError)
            }
        });

        try_join_all(task_futures).await?;

        let total_tasks = task_configs.len() as i32;
        Ok(BuildResponse {
            id: build.id,
            number: build.number,
            repository_id: build.repository_id,
            ref_name: build.ref_name,
            commit_sha: build.commit_sha,
            trigger: build.trigger,
            status: BuildStatus::Running,
            total_tasks,
            completed_tasks: 0,
            created_at: build.created_at,
            updated_at: build.created_at,
        })
    }

    async fn list_builds(
        &self,
        request: ListBuildsRequest,
    ) -> Result<Page<BuildResponse>, BuildError> {
        let owner = request.repo_owner.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let (builds, next_cursor) = self
            .build_repo
            .list_by_repo(repository.id, request.cursor, request.limit as i64)
            .await?;

        Ok(Page {
            data: builds.into_iter().map(Into::into).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn get_build(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<BuildResponse, BuildError> {
        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let build = self
            .build_repo
            .get(repository.id, number)
            .await?
            .or_not_found("build", format!("{owner}/{repo}#{number}"))?;

        let tasks = self.task_repo.list_by_build_id(build.id).await?;

        let total_tasks = tasks.len() as i32;
        let completed_tasks = tasks
            .iter()
            .filter(|t| matches!(t.status, TaskStatus::Success))
            .count() as i32;
        let status = if tasks.is_empty() {
            BuildStatus::Running
        } else if tasks.iter().any(|t| t.status == TaskStatus::Failure) {
            BuildStatus::Failure
        } else if tasks.iter().all(|t| t.status == TaskStatus::Success) {
            BuildStatus::Success
        } else {
            BuildStatus::Running
        };

        let effective_updated_at = tasks
            .iter()
            .map(|t| t.updated_at)
            .max()
            .unwrap_or(build.created_at);

        Ok(BuildResponse {
            id: build.id,
            number: build.number,
            repository_id: build.repository_id,
            ref_name: build.ref_name,
            commit_sha: build.commit_sha,
            trigger: build.trigger,
            status,
            total_tasks,
            completed_tasks,
            created_at: build.created_at,
            updated_at: effective_updated_at,
        })
    }

    async fn list_build_tasks(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Vec<TaskResponse>, BuildError> {
        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let build = self
            .build_repo
            .get(repository.id, number)
            .await?
            .or_not_found("build", format!("{owner}/{repo}#{number}"))?;

        let tasks = self.task_repo.list_by_build_id(build.id).await?;

        Ok(tasks.into_iter().map(Into::into).collect())
    }
}
