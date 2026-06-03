use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::MetricsError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Metrics(#[from] MetricsError),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

fn error_response(status: StatusCode, message: String) -> Response {
    (status, Json(ErrorMessage { message })).into_response()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Metrics(MetricsError::Input(_)) => StatusCode::BAD_REQUEST,
            AppError::Metrics(MetricsError::ClickHouse(_)) | AppError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        if status.is_server_error() {
            tracing::error!(error = %self, "request failed with internal error");
            let message = status
                .canonical_reason()
                .unwrap_or("Internal server error")
                .to_string();
            return error_response(status, message);
        }

        error_response(status, self.to_string())
    }
}
