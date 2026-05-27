use crate::{
    dto::Email,
    error::{InputError, SessionError},
};

#[derive(Debug, Clone)]
pub struct SendAuthEmailRequest {
    pub email: Email,
}

impl SendAuthEmailRequest {
    pub fn new(email: &str) -> Result<Self, SessionError> {
        Ok(Self {
            email: Email::try_new(email).map_err(|e| InputError::new("email", e))?,
        })
    }
}
