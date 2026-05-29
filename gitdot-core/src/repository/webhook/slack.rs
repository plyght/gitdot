use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{SlackWebhook, WebhookEventType},
};

/// sqlx data-access layer for the `webhook.slack_webhooks` table, which binds a
/// repository's webhook events to a Slack user/team/channel destination.
#[async_trait]
pub trait SlackWebhookRepository: Send + Sync + Clone + 'static {
    /// Inserts a row into `webhook.slack_webhooks` and returns the created
    /// webhook via `RETURNING`.
    async fn create(
        &self,
        user_id: Uuid,
        repository_id: Uuid,
        events: &[WebhookEventType],
        slack_user_id: &str,
        slack_team_id: &str,
        slack_channel_id: &str,
    ) -> Result<SlackWebhook, DatabaseError>;

    /// Returns the `webhook.slack_webhooks` row with the given `id`, or
    /// `Ok(None)` if no such row exists.
    async fn get(&self, id: Uuid) -> Result<Option<SlackWebhook>, DatabaseError>;

    /// Lists `webhook.slack_webhooks` rows for `repository_id` whose `events`
    /// array contains `event` (`$2 = ANY(events)`), ordered by `created_at` ASC.
    /// Returns an empty `Vec` when none match.
    async fn list_by_repository_and_event(
        &self,
        repository_id: Uuid,
        event: WebhookEventType,
    ) -> Result<Vec<SlackWebhook>, DatabaseError>;

    /// Hard-deletes the `webhook.slack_webhooks` row with the given `id`.
    /// Succeeds even if no row matched.
    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct SlackWebhookRepositoryImpl {
    pool: PgPool,
}

impl SlackWebhookRepositoryImpl {
    pub fn new(pool: PgPool) -> SlackWebhookRepositoryImpl {
        SlackWebhookRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SlackWebhookRepository for SlackWebhookRepositoryImpl {
    async fn create(
        &self,
        user_id: Uuid,
        repository_id: Uuid,
        events: &[WebhookEventType],
        slack_user_id: &str,
        slack_team_id: &str,
        slack_channel_id: &str,
    ) -> Result<SlackWebhook, DatabaseError> {
        let webhook = sqlx::query_as::<_, SlackWebhook>(
            r#"
            INSERT INTO webhook.slack_webhooks
                (user_id, repository_id, events, slack_user_id, slack_team_id, slack_channel_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, repository_id, events,
                slack_user_id, slack_team_id, slack_channel_id, created_at
            "#,
        )
        .bind(user_id)
        .bind(repository_id)
        .bind(events)
        .bind(slack_user_id)
        .bind(slack_team_id)
        .bind(slack_channel_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn get(&self, id: Uuid) -> Result<Option<SlackWebhook>, DatabaseError> {
        let webhook = sqlx::query_as::<_, SlackWebhook>(
            r#"
            SELECT id, user_id, repository_id, events,
                slack_user_id, slack_team_id, slack_channel_id, created_at
            FROM webhook.slack_webhooks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn list_by_repository_and_event(
        &self,
        repository_id: Uuid,
        event: WebhookEventType,
    ) -> Result<Vec<SlackWebhook>, DatabaseError> {
        let webhooks = sqlx::query_as::<_, SlackWebhook>(
            r#"
            SELECT id, user_id, repository_id, events,
                slack_user_id, slack_team_id, slack_channel_id, created_at
            FROM webhook.slack_webhooks
            WHERE repository_id = $1 AND $2 = ANY(events)
            ORDER BY created_at ASC
            "#,
        )
        .bind(repository_id)
        .bind(event)
        .fetch_all(&self.pool)
        .await?;

        Ok(webhooks)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM webhook.slack_webhooks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
