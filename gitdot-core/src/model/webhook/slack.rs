use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::WebhookEventType;

/// Special type of webhook owned by gitdot.
#[derive(Debug, Clone, FromRow)]
pub struct SlackWebhook {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub events: Vec<WebhookEventType>,

    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,

    pub created_at: DateTime<Utc>,
}
