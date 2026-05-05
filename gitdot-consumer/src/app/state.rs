use std::sync::Arc;

use anyhow::Context;
use rdkafka::{ClientConfig, consumer::StreamConsumer};
use sqlx::PgPool;

use gitdot_core::{
    client::{Git2Client, KafkaClientImpl, SecretClient, SlackBotClientImpl, TokenClientImpl},
    repository::{
        RepositoryRepositoryImpl, SlackWebhookRepositoryImpl, UserRepositoryImpl,
        WebhookRepositoryImpl,
    },
    service::{WebhookService, WebhookServiceImpl},
};

use super::Settings;

#[derive(Clone)]
pub struct ConsumerState {
    pub settings: Settings,
    pub webhook_service: Arc<dyn WebhookService>,
}

impl ConsumerState {
    pub async fn new(
        settings: Settings,
        pool: PgPool,
        secret_client: impl SecretClient,
    ) -> anyhow::Result<Self> {
        let webhook_repo = WebhookRepositoryImpl::new(pool.clone());
        let slack_webhook_repo = SlackWebhookRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());

        let git_client = Git2Client::new("".to_string());
        let kafka_client = KafkaClientImpl::new(&settings.kafka_bootstrap_servers)?;
        let slack_bot_client = SlackBotClientImpl::new(
            settings.gitdot_slack_bot_server_url.clone(),
            secret_client.get_gitdot_slack_secret().await?,
        );

        // TokenClientImpl is unused on this path, but referenced indirectly by core
        // wiring; the consumer passes nothing into WebhookServiceImpl that needs it.
        let _ = TokenClientImpl::new(String::new());

        let webhook_service = Arc::new(WebhookServiceImpl::new(
            webhook_repo,
            slack_webhook_repo,
            repo_repo,
            user_repo,
            git_client,
            kafka_client,
            slack_bot_client,
        ));

        Ok(Self {
            settings,
            webhook_service,
        })
    }
}

pub fn build_consumer(settings: &Settings) -> anyhow::Result<StreamConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &settings.kafka_bootstrap_servers)
        .set("group.id", &settings.kafka_consumer_group_id)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "10000")
        .create()
        .context("create kafka consumer")?;
    Ok(consumer)
}
