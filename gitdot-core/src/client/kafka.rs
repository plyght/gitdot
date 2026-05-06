use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rdkafka::{
    ClientConfig,
    client::{ClientContext, OAuthToken},
    consumer::ConsumerContext,
    producer::{DeliveryResult, FutureProducer, FutureRecord, Producer, ProducerContext},
    util::Timeout,
};
use serde_json::json;

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
        // Hard fail: without the SA email we cannot authenticate to Managed Kafka.
        let principal_name = fetch_service_account_email()
            .await
            .map_err(|e| KafkaError::AuthError(format!("resolve SA email from metadata: {e}")))?;
        tracing::info!(%principal_name, "resolved kafka SASL principal");
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
        let token =
            tokio::task::block_in_place(|| self.runtime.block_on(self.provider.token(&scopes)))?;

        // Build the GOOG_OAUTH2_TOKEN-flavored unsigned JWT that GCP's broker expects.
        // The actual access token is base64url-encoded into the signature segment;
        // the broker validates it against Google's auth servers and uses `sub` as
        // the Kafka principal.
        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let exp_secs = token.expires_at().timestamp();

        let header = json!({ "typ": "JWT", "alg": "GOOG_OAUTH2_TOKEN" }).to_string();
        let payload = json!({
            "exp": exp_secs,
            "iat": now_secs,
            "iss": "Google",
            "scope": "kafka",
            "sub": &self.principal_name,
        })
        .to_string();

        let kafka_token = format!(
            "{}.{}.{}",
            URL_SAFE_NO_PAD.encode(header),
            URL_SAFE_NO_PAD.encode(payload),
            URL_SAFE_NO_PAD.encode(token.as_str()),
        );

        Ok(OAuthToken {
            token: kafka_token,
            principal_name: self.principal_name.clone(),
            lifetime_ms: exp_secs * 1000,
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
