use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, EmailError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum EmailVerificationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("Invalid or expired code")]
    InvalidCode,

    #[error(transparent)]
    EmailError(#[from] EmailError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
