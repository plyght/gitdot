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
            Self::TooManyAttempts => StatusCode::LOCKED,
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

fn error_response(status_code: StatusCode, message: String) -> Response {
    (status_code, Json(ErrorMessage { message })).into_response()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Account(e) => error_response(e.status_code(), e.to_string()),
            AppError::Device(e) => error_response(e.status_code(), e.to_string()),
            AppError::Session(e) => error_response(e.status_code(), e.to_string()),
            AppError::Slack(e) => error_response(e.status_code(), e.to_string()),
            AppError::TokenExtraction(e) => error_response(e.status_code(), e.to_string()),
        }
    }
}
