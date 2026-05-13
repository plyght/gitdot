use std::time::Duration;

use async_trait::async_trait;
use redis::{AsyncCommands, aio::ConnectionManager};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::RedisError;

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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl RedisClient for RedisClientImpl {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let mut conn = self.conn.clone();
        let raw: Option<String> = conn.get(key).await?;
        Ok(raw.map(|s| serde_json::from_str(&s)).transpose()?)
    }

    async fn set_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), RedisError> {
        let mut conn = self.conn.clone();
        let raw = serde_json::to_string(value)?;
        let _: () = conn.set_ex(key, raw, ttl.as_secs()).await?;
        Ok(())
    }

    async fn set_nx_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<bool, RedisError> {
        let mut conn = self.conn.clone();
        let raw = serde_json::to_string(value)?;
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(raw)
            .arg("NX")
            .arg("EX")
            .arg(ttl.as_secs())
            .query_async(&mut conn)
            .await?;
        Ok(result.is_some())
    }

    async fn delete(&self, key: &str) -> Result<(), RedisError> {
        let mut conn = self.conn.clone();
        let _: () = conn.del(key).await?;
        Ok(())
    }

    async fn ping(&self) -> Result<(), RedisError> {
        let mut conn = self.conn.clone();
        let _: () = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(())
    }
}
