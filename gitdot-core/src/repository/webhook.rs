mod slack;
mod webhook;

pub use slack::{PgSlackWebhookRepository, SlackWebhookRepository};
pub use webhook::{PgWebhookRepository, WebhookRepository};
