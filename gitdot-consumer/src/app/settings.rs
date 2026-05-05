use std::env;

use gitdot_core::client::KafkaAuthMode;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: Option<String>,
    pub gcp_project_id: Option<String>,

    pub kafka_bootstrap_servers: String,
    pub kafka_consumer_group_id: String,
    pub kafka_auth: KafkaAuthMode,

    pub gitdot_slack_bot_server_url: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").ok(),
            gcp_project_id: env::var("GCP_PROJECT_ID").ok(),

            kafka_bootstrap_servers: env::var("KAFKA_BOOTSTRAP_SERVERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            kafka_consumer_group_id: env::var("KAFKA_CONSUMER_GROUP_ID")
                .unwrap_or_else(|_| "gitdot-consumer-3".to_string()),
            kafka_auth: env::var("KAFKA_AUTH")
                .map(|s| KafkaAuthMode::from_env_str(&s))
                .unwrap_or_default(),

            gitdot_slack_bot_server_url: env::var("GITDOT_SLACK_BOT_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
        })
    }
}
