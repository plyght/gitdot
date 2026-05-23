use async_trait::async_trait;
use clickhouse::Client;

use crate::error::ClickHouseError;

#[async_trait]
pub trait ClickHouseClient: Send + Sync + Clone + 'static {
    async fn insert<T>(&self, table: &str, rows: &[T]) -> Result<(), ClickHouseError>
    where
        T: clickhouse::RowOwned + clickhouse::RowWrite + Send + Sync;

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
