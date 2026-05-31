use thiserror::Error;

use crate::error::{
    DatabaseError, EmailError, GitHubError, InputError, NotFoundError, RedisError, TokenError,
};

#[derive(Debug, Error)]
pub enum SessionError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Too many incorrect attempts; request a new code")]
    TooManyAttempts,

    #[error("Token expired: {0}")]
    TokenExpired(String),

    #[error("Token revoked: {0}")]
    TokenRevoked(String),

    #[error(transparent)]
    EmailError(#[from] EmailError),

    #[error(transparent)]
    GitHubError(#[from] GitHubError),

    #[error(transparent)]
    TokenError(#[from] TokenError),

    #[error(transparent)]
    CacheError(#[from] RedisError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
