use async_trait::async_trait;

use crate::{
    dto::{
        CreateWebhookRequest, DeleteWebhookRequest, GetWebhookRequest, ListWebhooksRequest, Page,
        UpdateWebhookRequest, WebhookResponse,
    },
    error::{NotFoundError, OptionNotFoundExt, WebhookError},
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, WebhookRepository, WebhookRepositoryImpl,
    },
    util::cursor,
};

/// Manages a repository's outbound HTTP webhooks (url, secret, and subscribed
/// event types). Every operation is scoped to the repository resolved from the
/// request's owner/name.
#[async_trait]
pub trait WebhookService: Send + Sync + 'static {
    /// Creates a webhook on the resolved repository with the given url, secret,
    /// and subscribed events.
    ///
    /// # Errors
    /// - [`WebhookError::NotFound`] if no repository matches owner/name.
    /// - [`WebhookError::DatabaseError`] if persisting the webhook fails.
    async fn create_webhook(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError>;

    /// Lists a repository's webhooks as a cursor-paginated page.
    ///
    /// Uses the request's cursor and limit; the returned page carries an encoded
    /// `next_cursor` when more results remain.
    ///
    /// # Errors
    /// - [`WebhookError::NotFound`] if no repository matches owner/name.
    /// - [`WebhookError::DatabaseError`] if the query fails.
    async fn list_webhooks(
        &self,
        request: ListWebhooksRequest,
    ) -> Result<Page<WebhookResponse>, WebhookError>;

    /// Fetches a single webhook by id.
    ///
    /// The webhook must belong to the resolved repository; a mismatch is
    /// reported as not found rather than leaking the webhook's existence.
    ///
    /// # Errors
    /// - [`WebhookError::NotFound`] if the repository or webhook does not exist, or the webhook belongs to another repository.
    /// - [`WebhookError::DatabaseError`] if the query fails.
    async fn get_webhook(
        &self,
        request: GetWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError>;

    /// Updates a webhook's url, secret, and/or events.
    ///
    /// Only fields present in the request are changed (others left as-is). The
    /// webhook must belong to the resolved repository, otherwise it is treated
    /// as not found.
    ///
    /// # Errors
    /// - [`WebhookError::NotFound`] if the repository or webhook does not exist, or the webhook belongs to another repository.
    /// - [`WebhookError::DatabaseError`] if persisting the update fails.
    async fn update_webhook(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError>;

    /// Deletes a webhook by id.
    ///
    /// The webhook must belong to the resolved repository, otherwise it is
    /// treated as not found.
    ///
    /// # Errors
    /// - [`WebhookError::NotFound`] if the repository or webhook does not exist, or the webhook belongs to another repository.
    /// - [`WebhookError::DatabaseError`] if deleting the webhook fails.
    async fn delete_webhook(&self, request: DeleteWebhookRequest) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone)]
pub struct WebhookServiceImpl<W, R>
where
    W: WebhookRepository,
    R: RepositoryRepository,
{
    webhook_repo: W,
    repo_repo: R,
}

impl WebhookServiceImpl<WebhookRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(webhook_repo: WebhookRepositoryImpl, repo_repo: RepositoryRepositoryImpl) -> Self {
        Self {
            webhook_repo,
            repo_repo,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<W, R> WebhookService for WebhookServiceImpl<W, R>
where
    W: WebhookRepository,
    R: RepositoryRepository,
{
    async fn create_webhook(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let webhook = self
            .webhook_repo
            .create(
                repository.id,
                &request.url,
                &request.secret,
                &request.events,
            )
            .await?;

        Ok(webhook.into())
    }

    async fn list_webhooks(
        &self,
        request: ListWebhooksRequest,
    ) -> Result<Page<WebhookResponse>, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let (webhooks, next_cursor) = self
            .webhook_repo
            .list_by_repo(repository.id, request.cursor, request.limit as i64)
            .await?;

        Ok(Page {
            data: webhooks.into_iter().map(Into::into).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn get_webhook(
        &self,
        request: GetWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let webhook = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("webhook", request.webhook_id)?;

        if webhook.repository_id != repository.id {
            return Err(NotFoundError::new("webhook", request.webhook_id).into());
        }

        Ok(webhook.into())
    }

    async fn update_webhook(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let existing = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("webhook", request.webhook_id)?;

        if existing.repository_id != repository.id {
            return Err(NotFoundError::new("webhook", request.webhook_id).into());
        }

        let webhook = self
            .webhook_repo
            .update(
                request.webhook_id,
                request.url.as_ref().map(|u| u.as_ref()),
                request.secret.as_deref(),
                request.events.as_deref(),
            )
            .await?;

        Ok(webhook.into())
    }

    async fn delete_webhook(&self, request: DeleteWebhookRequest) -> Result<(), WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let existing = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("webhook", request.webhook_id)?;

        if existing.repository_id != repository.id {
            return Err(NotFoundError::new("webhook", request.webhook_id).into());
        }

        self.webhook_repo.delete(request.webhook_id).await?;

        Ok(())
    }
}
