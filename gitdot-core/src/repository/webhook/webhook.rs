use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Webhook, WebhookEventType},
};

/// sqlx data-access layer for the `webhook.webhooks` table, which stores a
/// repository's outbound HTTP webhook destinations (url, signing secret, events).
#[async_trait]
pub trait WebhookRepository: Send + Sync + Clone + 'static {
    /// Inserts a row into `webhook.webhooks` and returns the created webhook via
    /// `RETURNING`.
    async fn create(
        &self,
        repository_id: Uuid,
        url: &str,
        secret: &str,
        events: &[WebhookEventType],
    ) -> Result<Webhook, DatabaseError>;

    /// Returns the `webhook.webhooks` row with the given `id`, or `Ok(None)` if
    /// no such row exists.
    async fn get(&self, id: Uuid) -> Result<Option<Webhook>, DatabaseError>;

    /// Lists `webhook.webhooks` rows for `repository_id`, keyset-paginated by
    /// `(created_at, id)` descending. Fetches `limit + 1` rows to detect a next
    /// page; returns the page (capped at `limit`) plus `Some(Cursor)` of the
    /// last returned row when more rows exist, else `None`.
    async fn list_by_repo(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Webhook>, Option<Cursor>), DatabaseError>;

    /// Updates the `webhook.webhooks` row with the given `id`, applying only the
    /// `Some` fields via `COALESCE` (each `None` leaves the column unchanged) and
    /// bumping `updated_at` to `now()`. Returns the updated row via `RETURNING`.
    async fn update(
        &self,
        id: Uuid,
        url: Option<&str>,
        secret: Option<&str>,
        events: Option<&[WebhookEventType]>,
    ) -> Result<Webhook, DatabaseError>;

    /// Hard-deletes the `webhook.webhooks` row with the given `id`. Succeeds even
    /// if no row matched.
    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgWebhookRepository {
    pool: PgPool,
}

impl PgWebhookRepository {
    pub fn new(pool: PgPool) -> PgWebhookRepository {
        PgWebhookRepository { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl WebhookRepository for PgWebhookRepository {
    async fn create(
        &self,
        repository_id: Uuid,
        url: &str,
        secret: &str,
        events: &[WebhookEventType],
    ) -> Result<Webhook, DatabaseError> {
        let webhook = sqlx::query_as::<_, Webhook>(
            r#"
            INSERT INTO webhook.webhooks (repository_id, url, secret, events)
            VALUES ($1, $2, $3, $4)
            RETURNING id, repository_id, url, secret, events, created_at, updated_at
            "#,
        )
        .bind(repository_id)
        .bind(url)
        .bind(secret)
        .bind(events)
        .fetch_one(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn get(&self, id: Uuid) -> Result<Option<Webhook>, DatabaseError> {
        let webhook = sqlx::query_as::<_, Webhook>(
            r#"
            SELECT id, repository_id, url, secret, events, created_at, updated_at
            FROM webhook.webhooks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn list_by_repo(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Webhook>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut webhooks = sqlx::query_as::<_, Webhook>(
            r#"
            SELECT id, repository_id, url, secret, events, created_at, updated_at
            FROM webhook.webhooks
            WHERE repository_id = $1
              AND ($2::timestamptz IS NULL OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC
            LIMIT $4
            "#,
        )
        .bind(repository_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if webhooks.len() as i64 > limit {
            webhooks.pop();
            webhooks.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((webhooks, next_cursor))
    }

    async fn update(
        &self,
        id: Uuid,
        url: Option<&str>,
        secret: Option<&str>,
        events: Option<&[WebhookEventType]>,
    ) -> Result<Webhook, DatabaseError> {
        let webhook = sqlx::query_as::<_, Webhook>(
            r#"
            UPDATE webhook.webhooks
            SET url = COALESCE($2, url),
                secret = COALESCE($3, secret),
                events = COALESCE($4, events),
                updated_at = now()
            WHERE id = $1
            RETURNING id, repository_id, url, secret, events, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(url)
        .bind(secret)
        .bind(events)
        .fetch_one(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM webhook.webhooks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
