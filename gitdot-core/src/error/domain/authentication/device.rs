use thiserror::Error;

use crate::error::{DatabaseError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("Token expired: {0}")]
    TokenExpired(String),

    #[error("Token pending: {0}")]
    TokenPending(String),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
