use thiserror::Error;

use crate::error::{DatabaseError, SlackBotError};

#[derive(Debug, Error)]
pub enum SlackError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    SlackBotError(#[from] SlackBotError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
