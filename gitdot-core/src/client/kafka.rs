use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use rdkafka::{
    ClientConfig,
    client::{ClientContext, OAuthToken},
    consumer::ConsumerContext,
    producer::{DeliveryResult, FutureProducer, FutureRecord, Producer, ProducerContext},
    util::Timeout,
};

use crate::{dto::RepoPushEvent, error::KafkaError};

// TODO: REWRITE THIS CLIENT

const REPO_PUSHED_TOPIC: &str = "gitdot.repo.pushed";

const SEND_TIMEOUT: Duration = Duration::from_secs(5);

const GCP_KAFKA_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KafkaAuthMode {
    #[default]
    Local,
    GcpOauthbearer,
}

impl KafkaAuthMode {
    pub fn from_env_str(s: &str) -> Self {
        match s {
            "gcp_oauthbearer" => Self::GcpOauthbearer,
            _ => Self::Local,
        }
    }

    /// Apply the SASL/TLS settings that this auth mode requires onto a
    /// `ClientConfig`. Caller is still responsible for `bootstrap.servers`
    /// and the per-role settings.
    pub fn apply_security_config(&self, config: &mut ClientConfig) {
        if matches!(self, Self::GcpOauthbearer) {
            config
                .set("security.protocol", "SASL_SSL")
                .set("sasl.mechanism", "OAUTHBEARER");
        }
    }
}

#[async_trait]
pub trait KafkaClient: Send + Sync + Clone + 'static {
    async fn publish_repo_push(&self, event: RepoPushEvent) -> Result<(), KafkaError>;
}

#[derive(Clone)]
pub struct KafkaClientImpl {
    producer: ProducerHandle,
}

#[derive(Clone)]
enum ProducerHandle {
    Plain(FutureProducer),
    Gcp(FutureProducer<GcpKafkaContext>),
}

#[derive(Clone)]
pub struct GcpKafkaContext {
    provider: Arc<dyn gcp_auth::TokenProvider>,
    runtime: tokio::runtime::Handle,
}

impl GcpKafkaContext {
    pub async fn new() -> Result<Self, KafkaError> {
        let runtime = tokio::runtime::Handle::current();
        let provider = gcp_auth::provider()
            .await
            .map_err(|e| KafkaError::AuthError(format!("init gcp auth provider: {e}")))?;
        Ok(Self { provider, runtime })
    }
}

impl ClientContext for GcpKafkaContext {
    const ENABLE_REFRESH_OAUTH_TOKEN: bool = true;

    fn generate_oauth_token(
        &self,
        _config: Option<&str>,
    ) -> Result<OAuthToken, Box<dyn std::error::Error>> {
        let scopes = [GCP_KAFKA_SCOPE];
        let token = self.runtime.block_on(self.provider.token(&scopes))?;
        let lifetime_ms = (token.expires_at() - chrono::Utc::now())
            .num_milliseconds()
            .max(0);
        Ok(OAuthToken {
            token: token.as_str().to_string(),
            principal_name: "kafka".to_string(),
            lifetime_ms,
        })
    }
}

impl ProducerContext for GcpKafkaContext {
    type DeliveryOpaque = ();
    fn delivery(&self, _result: &DeliveryResult, _opaque: Self::DeliveryOpaque) {}
}

impl ConsumerContext for GcpKafkaContext {}

impl KafkaClientImpl {
    pub async fn new(
        bootstrap_servers: &str,
        auth_mode: KafkaAuthMode,
    ) -> Result<Self, KafkaError> {
        let mut config = ClientConfig::new();
        config
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "30000")
            .set("acks", "all")
            .set("enable.idempotence", "true");

        auth_mode.apply_security_config(&mut config);

        let producer = match auth_mode {
            KafkaAuthMode::GcpOauthbearer => {
                let context = GcpKafkaContext::new().await?;
                ProducerHandle::Gcp(config.create_with_context(context)?)
            }
            KafkaAuthMode::Local => ProducerHandle::Plain(config.create()?),
        };

        Ok(Self { producer })
    }
}

impl std::fmt::Debug for KafkaClientImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KafkaClientImpl").finish_non_exhaustive()
    }
}

impl Drop for KafkaClientImpl {
    fn drop(&mut self) {
        // Block briefly so any in-flight messages get delivered before shutdown.
        match &self.producer {
            ProducerHandle::Plain(p) => {
                p.flush(Timeout::After(SEND_TIMEOUT)).ok();
            }
            ProducerHandle::Gcp(p) => {
                p.flush(Timeout::After(SEND_TIMEOUT)).ok();
            }
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl KafkaClient for KafkaClientImpl {
    async fn publish_repo_push(&self, event: RepoPushEvent) -> Result<(), KafkaError> {
        let key = format!("{}/{}", event.owner, event.repo);
        let payload = serde_json::to_vec(&event)?;
        let record = FutureRecord::to(REPO_PUSHED_TOPIC)
            .key(&key)
            .payload(&payload);

        match &self.producer {
            ProducerHandle::Plain(p) => p
                .send(record, Timeout::After(SEND_TIMEOUT))
                .await
                .map_err(|(e, _)| KafkaError::from(e))?,
            ProducerHandle::Gcp(p) => p
                .send(record, Timeout::After(SEND_TIMEOUT))
                .await
                .map_err(|(e, _)| KafkaError::from(e))?,
        };

        Ok(())
    }
}
