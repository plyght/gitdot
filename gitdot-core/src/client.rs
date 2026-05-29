//! Trait-and-impl wrappers around the external services gitdot talks to.
//!
//! Each integration exposes a trait (so it can be mocked in tests) alongside a
//! concrete `Impl`: git via `git2` and the `git http-backend` CGI, GitHub
//! (Octocrab), object storage (Cloudflare R2), Redis, Kafka, ClickHouse, S2
//! durable streams, SMTP email, image processing, Google Secret Manager, the
//! Slack bot API, and JWT/token generation.

mod clickhouse;
mod email;
mod git;
mod git_http;
mod github;
mod image;
mod kafka;
mod r2;
mod redis;
mod s2;
mod secret;
mod slack_bot;
mod token;

pub use clickhouse::{ClickHouseClient, ClickHouseClientImpl};
pub use email::{EmailClient, SmtpClient};
pub use git::{Git2Client, GitClient};
pub use git_http::{GitHttpClient, GitHttpClientImpl};
pub use github::{GitHubClient, OctocrabClient};
pub use image::{ImageClient, ImageClientImpl};
pub use kafka::{GcpKafkaContext, KafkaAuthMode, KafkaClient, KafkaClientImpl};
pub use r2::{R2Client, R2ClientImpl};
pub use redis::{RedisClient, RedisClientImpl};
pub use s2::{S2Client, S2ClientImpl};
pub use secret::{GoogleSecretClient, SecretClient};
pub use slack_bot::{
    SLACK_BOT_SIGNATURE_HEADER, SLACK_BOT_TIMESTAMP_HEADER, SlackBotClient, SlackBotClientImpl,
};
pub use token::{TokenClient, TokenClientImpl};
