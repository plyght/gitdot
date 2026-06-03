use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use gitdot_api::ApiResource;
use gitdot_axum::error::TokenExtractionError;
use gitdot_core::error::{
    AuthorizationError, BuildError, CommitError, GitHttpError, MigrationError, OrganizationError,
    QuestionError, RepositoryError, ReviewError, RunnerError, TaskError, TokenServiceError,
    UserError, WebhookError,
};

use super::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    TokenExtraction(#[from] TokenExtractionError),

    #[error(transparent)]
    TokenService(#[from] TokenServiceError),

    #[error(transparent)]
    Authorization(#[from] AuthorizationError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Commit(#[from] CommitError),

    #[error(transparent)]
    Question(#[from] QuestionError),

    #[error(transparent)]
    Review(#[from] ReviewError),

    #[error(transparent)]
    Migration(#[from] MigrationError),

    #[error(transparent)]
    GitHttp(#[from] GitHttpError),

    #[error(transparent)]
    Runner(#[from] RunnerError),

    #[error(transparent)]
    Build(#[from] BuildError),

    #[error(transparent)]
    Task(#[from] TaskError),

    #[error(transparent)]
    Webhook(#[from] WebhookError),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppErrorMessage {
    pub message: String,
}

impl ApiResource for AppErrorMessage {}

pub trait HttpStatus {
    fn status_code(&self) -> StatusCode;
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

impl HttpStatus for TokenServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::TokenError(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::ReadonlyRepository => StatusCode::FORBIDDEN,
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::InvalidImage(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::R2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for OrganizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::InvalidImage(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::R2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for RepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) | Self::TooManyPaths => StatusCode::BAD_REQUEST,
            Self::NotFound(_) | Self::NotAFile(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::GitError(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for CommitError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::GitError(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for QuestionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for ReviewError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_)
            | Self::CannotReviewOwnReview(_)
            | Self::CannotRemoveReviewAuthor(_)
            | Self::CannotReviewOwnDiff
            | Self::DiffAlreadyMerged
            | Self::ReviewNotPublishable(_)
            | Self::DiffNotPublishable(_)
            | Self::CommitsNotFound
            | Self::InvalidIdentifier => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) | Self::DiffNotMergeable(_) => StatusCode::CONFLICT,
            Self::NotOrgAdmin(_) => StatusCode::FORBIDDEN,
            Self::GitError(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for MigrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::GitError(_) | Self::GitHubError(_) | Self::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for GitHttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for RunnerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for BuildError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidConfig(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::GitError(_) | Self::JoinError(_) | Self::DatabaseError(_) | Self::S2Error(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::NoBuildConfig => StatusCode::UNPROCESSABLE_ENTITY,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for WebhookError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::GitError(_)
            | Self::GitHubError(_)
            | Self::KafkaError(_)
            | Self::SlackBotError(_)
            | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::TokenExtraction(e) => e.status_code(),
            AppError::TokenService(e) => e.status_code(),
            AppError::Authorization(e) => e.status_code(),
            AppError::User(e) => e.status_code(),
            AppError::Organization(e) => e.status_code(),
            AppError::Repository(e) => e.status_code(),
            AppError::Commit(e) => e.status_code(),
            AppError::Question(e) => e.status_code(),
            AppError::Review(e) => e.status_code(),
            AppError::Migration(e) => e.status_code(),
            AppError::GitHttp(e) => e.status_code(),
            AppError::Runner(e) => e.status_code(),
            AppError::Build(e) => e.status_code(),
            AppError::Task(e) => e.status_code(),
            AppError::Webhook(e) => e.status_code(),
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn error_response(status_code: StatusCode, message: String) -> Response {
    AppResponse::new(status_code, AppErrorMessage { message }).into_response()
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
