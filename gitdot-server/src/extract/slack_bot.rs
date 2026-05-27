use axum::{
    body::Body,
    extract::{FromRef, FromRequest, Request},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use gitdot_core::{
    client::{SLACK_BOT_SIGNATURE_HEADER, SLACK_BOT_TIMESTAMP_HEADER},
    dto::VerifySlackBotSignatureRequest,
    error::TokenExtractionError,
};

use crate::app::{AppError, AppState};

const MAX_BODY_BYTES: usize = 64 * 1024;

pub struct SlackBotSigned<T>(pub T);

impl<S, T> FromRequest<S> for SlackBotSigned<T>
where
    AppState: FromRef<S>,
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let (parts, body) = req.into_parts();

        let timestamp = header_value(&parts, SLACK_BOT_TIMESTAMP_HEADER)?;
        let signature = header_value(&parts, SLACK_BOT_SIGNATURE_HEADER)?;

        let body_bytes = read_body(body).await?;

        let parsed = serde_json::from_slice::<T>(&body_bytes)
            .map_err(|_| TokenExtractionError::Unauthorized)?;

        app_state
            .token_service
            .verify_slack_bot_signature(VerifySlackBotSignatureRequest::new(
                timestamp, body_bytes, signature,
            ))
            .map_err(|_| TokenExtractionError::Unauthorized)?;

        Ok(SlackBotSigned(parsed))
    }
}

fn header_value(parts: &Parts, name: &'static str) -> Result<String, AppError> {
    parts
        .headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| TokenExtractionError::Unauthorized.into())
}

async fn read_body(body: Body) -> Result<Vec<u8>, AppError> {
    let bytes = axum::body::to_bytes(body, MAX_BODY_BYTES)
        .await
        .map_err(|_| TokenExtractionError::Unauthorized)?;
    Ok(bytes.to_vec())
}
