use std::time::Duration;

use async_trait::async_trait;
use redis::{AsyncCommands, aio::ConnectionManager};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::RedisError;

/// Max attempts for transient IO errors (broken pipe, reset by peer, etc.).
/// `ConnectionManager` reconnects in the background after a failure but does
/// not retry the failed call itself; this wrapper covers that gap.
const MAX_RETRIES: u32 = 2;

#[async_trait]
pub trait RedisClient: Send + Sync + Clone + 'static {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, RedisError>;

    async fn set_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), RedisError>;

    /// Atomic claim + cache. Returns `true` if the key was set, `false` if it
    /// already existed.
    async fn set_nx_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<bool, RedisError>;

    async fn delete(&self, key: &str) -> Result<(), RedisError>;

    async fn ping(&self) -> Result<(), RedisError>;
}

#[derive(Clone)]
pub struct RedisClientImpl {
    conn: ConnectionManager,
}

impl RedisClientImpl {
    pub async fn new(url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { conn })
    }
}

async fn with_retry<T, F, Fut>(mut op: F) -> Result<T, redis::RedisError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, redis::RedisError>>,
{
    let mut attempts: u32 = 0;
    loop {
        match op().await {
            Ok(value) => return Ok(value),
            Err(e) if attempts < MAX_RETRIES && e.is_io_error() => {
                attempts += 1;
                tracing::warn!(attempts, error = %e, "redis io error; retrying");
                tokio::time::sleep(Duration::from_millis(20 * u64::from(attempts))).await;
            }
            Err(e) => return Err(e),
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl RedisClient for RedisClientImpl {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let raw: Option<String> = with_retry(|| async {
            let mut conn = self.conn.clone();
            conn.get(key).await
        })
        .await?;
        Ok(raw.map(|s| serde_json::from_str(&s)).transpose()?)
    }

    async fn set_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), RedisError> {
        let raw = serde_json::to_string(value)?;
        with_retry(|| async {
            let mut conn = self.conn.clone();
            let _: () = conn.set_ex(key, raw.as_str(), ttl.as_secs()).await?;
            Ok(())
        })
        .await?;
        Ok(())
    }

    async fn set_nx_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<bool, RedisError> {
        let raw = serde_json::to_string(value)?;
        let result: Option<String> = with_retry(|| async {
            let mut conn = self.conn.clone();
            redis::cmd("SET")
                .arg(key)
                .arg(raw.as_str())
                .arg("NX")
                .arg("EX")
                .arg(ttl.as_secs())
                .query_async(&mut conn)
                .await
        })
        .await?;
        Ok(result.is_some())
    }

    async fn delete(&self, key: &str) -> Result<(), RedisError> {
        with_retry(|| async {
            let mut conn = self.conn.clone();
            let _: () = conn.del(key).await?;
            Ok(())
        })
        .await?;
        Ok(())
    }

    async fn ping(&self) -> Result<(), RedisError> {
        with_retry(|| async {
            let mut conn = self.conn.clone();
            let _: () = redis::cmd("PING").query_async(&mut conn).await?;
            Ok(())
        })
        .await?;
        Ok(())
    }
}
