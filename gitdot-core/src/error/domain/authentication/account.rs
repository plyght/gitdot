use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, EmailError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("Invalid or expired code")]
    InvalidCode,

    #[error("Too many incorrect attempts; request a new code")]
    TooManyAttempts,

    #[error(transparent)]
    EmailError(#[from] EmailError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
