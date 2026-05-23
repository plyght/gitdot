use lettre::{
    address::AddressError, error::Error as LettreError, transport::smtp::Error as SmtpError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("SMTP transport error: {0}")]
    Transport(#[from] SmtpError),

    #[error("Email build error: {0}")]
    Build(#[from] LettreError),

    #[error("Invalid email address: {0}")]
    Address(#[from] AddressError),
}
