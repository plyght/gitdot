use thiserror::Error;

use crate::error::ClickHouseError;

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error(transparent)]
    ClickHouse(#[from] ClickHouseError),
}
