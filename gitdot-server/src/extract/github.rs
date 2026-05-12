use axum::{
    body::Body,
    extract::{FromRef, FromRequest, Request},
    http::request::Parts,
};

use gitdot_core::{dto::VerifyGithubSignatureRequest, error::AuthenticationError};

use crate::app::{AppError, AppState};

const GITHUB_EVENT_HEADER: &str = "X-GitHub-Event";
const GITHUB_DELIVERY_HEADER: &str = "X-GitHub-Delivery";
const GITHUB_SIGNATURE_HEADER: &str = "X-Hub-Signature-256";

const MAX_BODY_BYTES: usize = 5 * 1024 * 1024;

pub struct GithubSigned {
    pub event: String,
    pub delivery: String,
    pub body: Vec<u8>,
}

impl<S> FromRequest<S> for GithubSigned
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let (parts, body) = req.into_parts();

        let event = header_value(&parts, GITHUB_EVENT_HEADER)?;
        let delivery = header_value(&parts, GITHUB_DELIVERY_HEADER)?;
        let signature = header_value(&parts, GITHUB_SIGNATURE_HEADER)?;
        let body_bytes = read_body(body).await?;
        let request = VerifyGithubSignatureRequest::new(body_bytes.clone(), signature);
        app_state
            .authentication_service
            .verify_github_signature(request)?;

        Ok(GithubSigned {
            event,
            delivery,
            body: body_bytes,
        })
    }
}

fn header_value(parts: &Parts, name: &'static str) -> Result<String, AppError> {
    parts
        .headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AuthenticationError::Unauthorized.into())
}

async fn read_body(body: Body) -> Result<Vec<u8>, AppError> {
    let bytes = axum::body::to_bytes(body, MAX_BODY_BYTES)
        .await
        .map_err(|_| AuthenticationError::Unauthorized)?;
    Ok(bytes.to_vec())
}
