use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Redis error: {0}")]
    Connection(#[from] redis::RedisError),

    #[error("Failed to serialize Redis payload: {0}")]
    Serialization(#[from] serde_json::Error),
}
