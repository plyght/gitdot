use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_axum::error::TokenExtractionError;
use gitdot_core::error::{AccountError, DeviceError, SessionError, SlackError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Account(#[from] AccountError),

    #[error(transparent)]
    Device(#[from] DeviceError),

    #[error(transparent)]
    Session(#[from] SessionError),

    #[error(transparent)]
    Slack(#[from] SlackError),

    #[error(transparent)]
    TokenExtraction(#[from] TokenExtractionError),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

trait HttpStatus {
    fn status_code(&self) -> StatusCode;
}

impl HttpStatus for AccountError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) | Self::InvalidCode => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::TooManyAttempts => StatusCode::TOO_MANY_REQUESTS,
            Self::EmailError(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for DeviceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) | Self::TokenPending(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::TokenExpired(_) => StatusCode::UNAUTHORIZED,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for SessionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized | Self::TokenExpired(_) | Self::TokenRevoked(_) => {
                StatusCode::UNAUTHORIZED
            }
            Self::TooManyAttempts => StatusCode::TOO_MANY_REQUESTS,
            Self::EmailError(_)
            | Self::GitHubError(_)
            | Self::TokenError(_)
            | Self::CacheError(_)
            | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for SlackError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::SlackBotError(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for TokenExtractionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingHeader
            | Self::InvalidHeaderFormat
            | Self::InvalidPublicKey(_)
            | Self::InvalidToken(_)
            | Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl HttpStatus for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Account(e) => e.status_code(),
            AppError::Device(e) => e.status_code(),
            AppError::Session(e) => e.status_code(),
            AppError::Slack(e) => e.status_code(),
            AppError::TokenExtraction(e) => e.status_code(),
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn error_response(status_code: StatusCode, message: String) -> Response {
    (status_code, Json(ErrorMessage { message })).into_response()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

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
