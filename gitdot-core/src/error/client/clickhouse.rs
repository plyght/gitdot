use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClickHouseError {
    #[error(transparent)]
    Client(#[from] clickhouse::error::Error),
}
