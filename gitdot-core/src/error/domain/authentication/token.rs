use thiserror::Error;

use crate::error::{DatabaseError, TokenError};

#[derive(Debug, Error)]
pub enum TokenServiceError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    TokenError(#[from] TokenError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
