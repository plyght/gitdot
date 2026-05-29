use async_trait::async_trait;
use clickhouse::Client;

use crate::error::ClickHouseError;

/// Wraps a ClickHouse connection for batch row inserts and health checks.
///
/// The underlying client is configured for fire-and-forget async inserts
/// (`async_insert=1`, `wait_for_async_insert=0`), so a successful insert means
/// the rows were accepted for buffering, not yet durably committed.
#[async_trait]
pub trait ClickHouseClient: Send + Sync + Clone + 'static {
    /// Inserts `rows` into `table`. A call with no rows is a no-op that returns
    /// `Ok(())` without touching the server.
    ///
    /// # Errors
    /// - [`ClickHouseError::Client`] — the server rejected the insert or the
    ///   connection failed.
    async fn insert<T>(&self, table: &str, rows: &[T]) -> Result<(), ClickHouseError>
    where
        T: clickhouse::RowOwned + clickhouse::RowWrite + Send + Sync;

    /// Verifies connectivity by running `SELECT 1`.
    ///
    /// # Errors
    /// - [`ClickHouseError::Client`] — the server is unreachable or returned an
    ///   error.
    async fn ping(&self) -> Result<(), ClickHouseError>;
}

#[derive(Clone)]
pub struct ClickHouseClientImpl {
    client: Client,
}

impl ClickHouseClientImpl {
    pub fn new(url: &str, user: &str, password: &str, database: &str) -> Self {
        let client = Client::default()
            .with_url(url)
            .with_user(user)
            .with_password(password)
            .with_database(database)
            .with_setting("async_insert", "1")
            .with_setting("wait_for_async_insert", "0");
        Self { client }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl ClickHouseClient for ClickHouseClientImpl {
    async fn insert<T>(&self, table: &str, rows: &[T]) -> Result<(), ClickHouseError>
    where
        T: clickhouse::RowOwned + clickhouse::RowWrite + Send + Sync,
    {
        if rows.is_empty() {
            return Ok(());
        }
        let mut insert = self.client.insert::<T>(table).await?;
        for row in rows {
            insert.write(row).await?;
        }
        insert.end().await?;
        Ok(())
    }

    async fn ping(&self) -> Result<(), ClickHouseError> {
        self.client.query("SELECT 1").execute().await?;
        Ok(())
    }
}
