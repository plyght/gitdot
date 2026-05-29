use async_trait::async_trait;

use crate::{
    client::{TokenClient, TokenClientImpl},
    dto::{
        CreateRunnerRequest, CreateRunnerResponse, CreateRunnerTokenRequest,
        CreateRunnerTokenResponse, DeleteRunnerRequest, GetRunnerRequest, GetRunnerResponse,
        ListRunnersRequest, Page, RunnerResponse, VerifyRunnerRequest,
    },
    error::{NotFoundExt, OptionNotFoundExt, RunnerError},
    model::{RunnerOwnerType, TokenType},
    repository::{
        OrganizationRepository, PgOrganizationRepository, PgRunnerRepository, PgTokenRepository,
        RunnerRepository, TokenRepository,
    },
    util::cursor,
};

/// CI runners: registering runners under a user or organization, liveness
/// checks, and minting/rotating their access tokens.
#[async_trait]
pub trait RunnerService: Send + Sync + 'static {
    /// Registers a runner owned by a user or organization.
    ///
    /// For [`Organization`] owners the org is resolved by name to its ID; for
    /// [`User`] owners the request's `user_id` is used directly.
    ///
    /// # Errors
    /// - [`RunnerError::NotFound`] if an organization owner does not exist.
    ///
    /// [`Organization`]: crate::model::RunnerOwnerType::Organization
    /// [`User`]: crate::model::RunnerOwnerType::User
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse, RunnerError>;

    /// Records a liveness heartbeat for a runner, updating its `last_active`.
    ///
    /// # Errors
    /// - [`RunnerError::NotFound`] if no runner has `request.runner_id`.
    async fn verify_runner(&self, request: VerifyRunnerRequest) -> Result<(), RunnerError>;

    /// Fetches a runner by owner name and runner name.
    ///
    /// # Errors
    /// - [`RunnerError::NotFound`] if the runner does not exist.
    async fn get_runner(&self, request: GetRunnerRequest)
    -> Result<GetRunnerResponse, RunnerError>;

    /// Deletes a runner identified by owner name and runner name.
    ///
    /// # Errors
    /// - [`RunnerError::NotFound`] if the runner does not exist.
    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError>;

    /// Rotates a runner's access token, returning the new raw token.
    ///
    /// Any existing token for the runner is deleted before a fresh [`Runner`]
    /// token is generated and persisted (only its hash is stored). The raw token
    /// is returned once and cannot be retrieved again.
    ///
    /// # Errors
    /// - [`RunnerError::NotFound`] if the runner does not exist.
    ///
    /// [`Runner`]: crate::model::TokenType::Runner
    async fn refresh_runner_token(
        &self,
        request: CreateRunnerTokenRequest,
    ) -> Result<CreateRunnerTokenResponse, RunnerError>;

    /// Lists runners for an owner, cursor-paginated.
    async fn list_runners(
        &self,
        request: ListRunnersRequest,
    ) -> Result<Page<RunnerResponse>, RunnerError>;
}

#[derive(Debug, Clone)]
pub struct RunnerServiceImpl<R, O, T, TC>
where
    R: RunnerRepository,
    O: OrganizationRepository,
    T: TokenRepository,
    TC: TokenClient,
{
    runner_repo: R,
    org_repo: O,
    token_repo: T,
    token_client: TC,
}

impl
    RunnerServiceImpl<
        PgRunnerRepository,
        PgOrganizationRepository,
        PgTokenRepository,
        TokenClientImpl,
    >
{
    pub fn new(
        runner_repo: PgRunnerRepository,
        org_repo: PgOrganizationRepository,
        token_repo: PgTokenRepository,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            runner_repo,
            org_repo,
            token_repo,
            token_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<R, O, T, TC> RunnerService for RunnerServiceImpl<R, O, T, TC>
where
    R: RunnerRepository,
    O: OrganizationRepository,
    T: TokenRepository,
    TC: TokenClient,
{
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse, RunnerError> {
        let owner_id = match request.owner_type {
            RunnerOwnerType::User => request.user_id,
            RunnerOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.owner_name.as_ref())
                    .await?
                    .or_not_found("owner", request.owner_name.as_ref())?;
                org.id
            }
        };

        let runner = self
            .runner_repo
            .create(
                request.name.as_ref(),
                owner_id,
                request.owner_name.as_ref(),
                &request.owner_type,
            )
            .await?;

        Ok(runner.into())
    }

    async fn verify_runner(&self, request: VerifyRunnerRequest) -> Result<(), RunnerError> {
        self.runner_repo
            .touch(request.runner_id)
            .await
            .or_not_found::<RunnerError>("runner", request.runner_id)?;

        Ok(())
    }

    async fn get_runner(
        &self,
        request: GetRunnerRequest,
    ) -> Result<GetRunnerResponse, RunnerError> {
        let runner = self
            .runner_repo
            .get(request.owner_name.as_ref(), request.name.as_ref())
            .await?
            .or_not_found("runner", request.name.as_ref())?;

        Ok(runner.into())
    }

    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError> {
        let runner = self
            .runner_repo
            .get(request.owner_name.as_ref(), request.name.as_ref())
            .await?
            .or_not_found("runner", request.name.as_ref())?;

        self.runner_repo
            .delete(runner.id)
            .await
            .or_not_found::<RunnerError>("runner", request.name.as_ref())?;

        Ok(())
    }

    async fn list_runners(
        &self,
        request: ListRunnersRequest,
    ) -> Result<Page<RunnerResponse>, RunnerError> {
        let (runners, next_cursor) = self
            .runner_repo
            .list_by_owner(
                request.owner_name.as_ref(),
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: runners.into_iter().map(Into::into).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn refresh_runner_token(
        &self,
        request: CreateRunnerTokenRequest,
    ) -> Result<CreateRunnerTokenResponse, RunnerError> {
        let runner = self
            .runner_repo
            .get(request.owner_name.as_ref(), request.runner_name.as_ref())
            .await?
            .or_not_found("runner", request.runner_name.as_ref())?;

        self.token_repo.delete_token_by_principal(runner.id).await?;

        let (raw_token, token_hash) = self.token_client.generate_access_token(&TokenType::Runner);

        let client_id = format!(
            "gitdot-runner/{}/{}",
            request.owner_name.as_ref(),
            request.runner_name.as_ref()
        );

        self.token_repo
            .create_token(runner.id, &client_id, &token_hash, TokenType::Runner)
            .await?;

        Ok(CreateRunnerTokenResponse { token: raw_token })
    }
}
