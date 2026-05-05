mod bootstrap;
mod runner;
mod settings;
mod state;

use anyhow::Context;
use rdkafka::consumer::{Consumer, StreamConsumer};
use sqlx::PgPool;

use gitdot_core::client::{GoogleSecretClient, SecretClient};

pub use settings::Settings;
pub use state::ConsumerState;

const REPO_PUSHED_TOPIC: &str = "gitdot.repo.pushed";

pub struct GitdotConsumer {
    state: ConsumerState,
    kafka: StreamConsumer,
}

impl GitdotConsumer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Settings::new()?;
        let secret_client = GoogleSecretClient::new(settings.gcp_project_id.clone()).await?;

        let database_url = match &settings.database_url {
            Some(url) => url.clone(),
            None => secret_client.get_database_url().await?,
        };
        let pool = PgPool::connect(&database_url).await?;

        let state = ConsumerState::new(settings, pool, secret_client).await?;
        let kafka = state::build_consumer(&state.settings)?;
        kafka
            .subscribe(&[REPO_PUSHED_TOPIC])
            .context("subscribe to topic")?;

        Ok(Self { state, kafka })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!(
            topic = REPO_PUSHED_TOPIC,
            group_id = %self.state.settings.kafka_consumer_group_id,
            "starting consumer",
        );
        runner::run(self.state, self.kafka).await
    }
}
