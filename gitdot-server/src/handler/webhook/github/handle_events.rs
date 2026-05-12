use http::StatusCode;

use axum::extract::State;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::GithubSigned,
};

#[axum::debug_handler]
pub async fn handle_events(
    State(_state): State<AppState>,
    GithubSigned {
        event,
        delivery,
        body,
    }: GithubSigned,
) -> Result<AppResponse<()>, AppError> {
    tracing::info!(
        event,
        delivery,
        body_len = body.len(),
        "received github webhook event",
    );
    Ok(AppResponse::new(StatusCode::OK, ()))
}
