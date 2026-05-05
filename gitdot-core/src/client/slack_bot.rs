use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Serialize;
use sha2::Sha256;
use uuid::Uuid;

use crate::{dto::SlackStatePayload, error::SlackBotError, model::WebhookEventType};

pub const SLACK_BOT_TIMESTAMP_HEADER: &str = "x-gitdot-timestamp";
pub const SLACK_BOT_SIGNATURE_HEADER: &str = "x-gitdot-signature";
pub const SLACK_BOT_EVENT_TYPE_HEADER: &str = "x-gitdot-event-type";

const FINALIZE_LOGIN_PATH: &str = "/gitdot/auth/finalize";
const EVENTS_PATH: &str = "/gitdot/events";

// Intentionally set tight time window to prevent replay attacks
const MAX_REQUEST_AGE_IN_SECONDS: i64 = 10;

#[async_trait]
pub trait SlackBotClient: Send + Sync + Clone + 'static {
    fn verify_slack_state(&self, state: &str) -> Result<SlackStatePayload, String>;

    fn verify_request_signature(
        &self,
        timestamp: &str,
        body: &[u8],
        signature: &str,
    ) -> Result<(), SlackBotError>;

    async fn notify_link_completed(
        &self,
        gitdot_user_id: Uuid,
        channel_id: &str,
    ) -> Result<(), SlackBotError>;

    async fn notify_event<T: Serialize + Send + Sync>(
        &self,
        event_type: WebhookEventType,
        body: &T,
    ) -> Result<(), SlackBotError>;
}

#[derive(Debug, Clone)]
pub struct SlackBotClientImpl {
    http: Client,
    server_url: String,
    slack_secret: String,
}

impl SlackBotClientImpl {
    pub fn new(server_url: String, slack_secret: String) -> Self {
        Self {
            http: Client::new(),
            server_url,
            slack_secret,
        }
    }

    fn sign(&self, timestamp: i64, body: &[u8]) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.slack_secret.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(b"v0:");
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b":");
        mac.update(body);
        format!("v0={}", hex::encode(mac.finalize().into_bytes()))
    }

    async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<(), SlackBotError> {
        self.post_with(path, None, body).await
    }

    async fn post_with<T: Serialize>(
        &self,
        path: &str,
        event_type: Option<&str>,
        body: &T,
    ) -> Result<(), SlackBotError> {
        let body_bytes = serde_json::to_vec(body)?;
        let timestamp = Utc::now().timestamp();
        let signature = self.sign(timestamp, &body_bytes);

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            SLACK_BOT_TIMESTAMP_HEADER,
            HeaderValue::from_str(&timestamp.to_string())
                .expect("ascii numeric timestamp is a valid header value"),
        );
        headers.insert(
            SLACK_BOT_SIGNATURE_HEADER,
            HeaderValue::from_str(&signature)
                .expect("hex-encoded signature is a valid header value"),
        );
        if let Some(event_type) = event_type {
            headers.insert(
                SLACK_BOT_EVENT_TYPE_HEADER,
                HeaderValue::from_str(event_type).expect("event type names are ascii safe"),
            );
        }

        let url = format!("{}{}", self.server_url, path);
        let response = self
            .http
            .post(&url)
            .headers(headers)
            .body(body_bytes)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            tracing::warn!(
                status = status.as_u16(),
                url = %url,
                body = %body,
                "slack_bot: non-success response",
            );
            return Err(SlackBotError::NonSuccessStatus {
                status: status.as_u16(),
                body,
            });
        }

        Ok(())
    }
}

#[derive(Serialize)]
struct FinalizeLoginRequest<'a> {
    gitdot_user_id: Uuid,
    channel_id: &'a str,
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SlackBotClient for SlackBotClientImpl {
    fn verify_slack_state(&self, state: &str) -> Result<SlackStatePayload, String> {
        let (payload_b64, sig_b64) = state.split_once('.').ok_or("Invalid state format")?;

        let mut mac = Hmac::<Sha256>::new_from_slice(self.slack_secret.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(payload_b64.as_bytes());

        let sig = URL_SAFE_NO_PAD
            .decode(sig_b64)
            .map_err(|_| "Invalid signature encoding")?;
        mac.verify_slice(&sig).map_err(|_| "Invalid signature")?;

        let payload_json = URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|_| "Invalid payload encoding")?;
        let payload: SlackStatePayload =
            serde_json::from_slice(&payload_json).map_err(|_| "Invalid payload")?;

        if payload.exp < Utc::now().timestamp() as u64 {
            return Err("State expired".to_string());
        }

        Ok(payload)
    }

    fn verify_request_signature(
        &self,
        timestamp: &str,
        body: &[u8],
        signature: &str,
    ) -> Result<(), SlackBotError> {
        let ts: i64 = timestamp
            .parse()
            .map_err(|_| SlackBotError::InvalidSignature("malformed timestamp".to_string()))?;

        if (Utc::now().timestamp() - ts).abs() > MAX_REQUEST_AGE_IN_SECONDS {
            return Err(SlackBotError::InvalidSignature("stale request".to_string()));
        }

        let sig_hex = signature
            .strip_prefix("v0=")
            .ok_or_else(|| SlackBotError::InvalidSignature("malformed signature".to_string()))?;
        let sig_bytes = hex::decode(sig_hex).map_err(|_| {
            SlackBotError::InvalidSignature("invalid signature encoding".to_string())
        })?;

        let mut mac = Hmac::<Sha256>::new_from_slice(self.slack_secret.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(b"v0:");
        mac.update(timestamp.as_bytes());
        mac.update(b":");
        mac.update(body);
        mac.verify_slice(&sig_bytes)
            .map_err(|_| SlackBotError::InvalidSignature("signature mismatch".to_string()))?;

        Ok(())
    }

    async fn notify_link_completed(
        &self,
        gitdot_user_id: Uuid,
        channel_id: &str,
    ) -> Result<(), SlackBotError> {
        self.post(
            FINALIZE_LOGIN_PATH,
            &FinalizeLoginRequest {
                gitdot_user_id,
                channel_id,
            },
        )
        .await
    }

    async fn notify_event<T: Serialize + Send + Sync>(
        &self,
        event_type: WebhookEventType,
        body: &T,
    ) -> Result<(), SlackBotError> {
        self.post_with(EVENTS_PATH, Some(event_type.as_str()), body)
            .await
    }
}
