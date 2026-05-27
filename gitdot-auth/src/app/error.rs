use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::{AuthenticationError, EmailVerificationError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Authentication(#[from] AuthenticationError),

    #[error(transparent)]
    EmailVerification(#[from] EmailVerificationError),
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            AppError::Authentication(e) => {
                let status_code = match &e {
                    AuthenticationError::Input(_) | AuthenticationError::TokenPending(_) => {
                        StatusCode::BAD_REQUEST
                    }
                    AuthenticationError::NotFound(_) => StatusCode::NOT_FOUND,
                    AuthenticationError::Extraction(_)
                    | AuthenticationError::TokenExpired(_)
                    | AuthenticationError::TokenRevoked(_)
                    | AuthenticationError::Unauthorized => StatusCode::UNAUTHORIZED,
                    AuthenticationError::TokenError(_)
                    | AuthenticationError::GitHubError(_)
                    | AuthenticationError::SlackBotError(_)
                    | AuthenticationError::EmailError(_)
                    | AuthenticationError::CacheError(_)
                    | AuthenticationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (status_code, e.to_string())
            }
            AppError::EmailVerification(e) => {
                let status_code = match &e {
                    EmailVerificationError::Input(_) | EmailVerificationError::InvalidCode => {
                        StatusCode::BAD_REQUEST
                    }
                    EmailVerificationError::NotFound(_) => StatusCode::NOT_FOUND,
                    EmailVerificationError::Conflict(_) => StatusCode::CONFLICT,
                    EmailVerificationError::EmailError(_)
                    | EmailVerificationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (status_code, e.to_string())
            }
        };
        let body = ErrorMessage { message };
        (status_code, Json(body)).into_response()
    }
}
