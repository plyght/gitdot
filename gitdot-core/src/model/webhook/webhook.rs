use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Webhook {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEventType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "webhook.webhook_event_type", rename_all = "snake_case")]
pub enum WebhookEventType {
    Push,
    ReviewPublish,
    ReviewUpdate,
}

impl WebhookEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebhookEventType::Push => "push",
            WebhookEventType::ReviewPublish => "review_publish",
            WebhookEventType::ReviewUpdate => "review_update",
        }
    }
}

impl From<WebhookEventType> for String {
    fn from(event: WebhookEventType) -> Self {
        event.as_str().to_string()
    }
}

impl TryFrom<&str> for WebhookEventType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "push" => Ok(WebhookEventType::Push),
            "review_publish" => Ok(WebhookEventType::ReviewPublish),
            "review_update" => Ok(WebhookEventType::ReviewUpdate),
            _ => Err(format!("Invalid webhook event type: {value}")),
        }
    }
}
