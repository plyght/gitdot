mod slack;
mod webhook;

pub use slack::{SlackWebhookRepository, SlackWebhookRepositoryImpl};
pub use webhook::{WebhookRepository, WebhookRepositoryImpl};
