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
    /// SASL authzid sent on the wire. GCP Managed Kafka rejects the handshake
    /// if this doesn't match the token's bound service account email.
    principal_name: String,
}

impl GcpKafkaContext {
    pub async fn new() -> Result<Self, KafkaError> {
        let runtime = tokio::runtime::Handle::current();
        let provider = gcp_auth::provider()
            .await
            .map_err(|e| KafkaError::AuthError(format!("init gcp auth provider: {e}")))?;
        let principal_name = fetch_service_account_email().await.unwrap_or_else(|e| {
            tracing::warn!(
                ?e,
                "could not resolve SA email from metadata server; falling back"
            );
            "kafka".to_string()
        });
        Ok(Self {
            provider,
            runtime,
            principal_name,
        })
    }
}

async fn fetch_service_account_email() -> Result<String, KafkaError> {
    let response = reqwest::Client::new()
        .get(
            "http://metadata.google.internal/computeMetadata/v1/\
             instance/service-accounts/default/email",
        )
        .header("Metadata-Flavor", "Google")
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map_err(|e| KafkaError::AuthError(format!("metadata server: {e}")))?;
    let email = response
        .text()
        .await
        .map_err(|e| KafkaError::AuthError(format!("metadata server body: {e}")))?;
    Ok(email.trim().to_string())
}

impl ClientContext for GcpKafkaContext {
    const ENABLE_REFRESH_OAUTH_TOKEN: bool = true;

    fn generate_oauth_token(
        &self,
        _config: Option<&str>,
    ) -> Result<OAuthToken, Box<dyn std::error::Error>> {
        let scopes = [GCP_KAFKA_SCOPE];
        // librdkafka calls this from inside a tokio task on the same runtime,
        // so a plain `block_on` panics. `block_in_place` releases the worker
        // first, letting us drive the future synchronously without re-entry.
        let token =
            tokio::task::block_in_place(|| self.runtime.block_on(self.provider.token(&scopes)))?;
        // librdkafka expects the absolute expiry timestamp in unix-epoch ms,
        // NOT a duration. Returning a duration here makes librdkafka think the
        // token expired in 1970.
        let lifetime_ms = token.expires_at().timestamp_millis();
        Ok(OAuthToken {
            token: token.as_str().to_string(),
            principal_name: self.principal_name.clone(),
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
