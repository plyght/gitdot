use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Runner, RunnerOwnerType},
};

/// sqlx data-access layer for the `ci.runners` table (CI runners owned by a user
/// or organization).
#[async_trait]
pub trait RunnerRepository: Send + Sync + Clone + 'static {
    /// Inserts a runner into `ci.runners` and returns the inserted row via
    /// `RETURNING`.
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_name: &str,
        owner_type: &RunnerOwnerType,
    ) -> Result<Runner, DatabaseError>;

    /// Returns the runner matching `(owner_name, name)`, or `Ok(None)` if none
    /// exists.
    async fn get(
        &self,
        owner_name: &str,
        runner_name: &str,
    ) -> Result<Option<Runner>, DatabaseError>;

    /// Hard-deletes the runner with the given id. Returns
    /// `DatabaseError::RowNotFound` when no row was affected.
    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Returns the runner with the given id, or `Ok(None)` if none exists.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Runner>, DatabaseError>;

    /// Updates the runner's `last_active` to `NOW()` (heartbeat). Returns
    /// `DatabaseError::RowNotFound` when no row was affected.
    async fn touch(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Lists runners for an owner, newest first (`created_at DESC, id DESC`),
    /// keyset-paginated by `cursor`. Returns the page and the next cursor
    /// (`None` when no further rows remain).
    async fn list_by_owner(
        &self,
        owner_name: &str,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Runner>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgRunnerRepository {
    pool: PgPool,
}

impl PgRunnerRepository {
    pub fn new(pool: PgPool) -> PgRunnerRepository {
        PgRunnerRepository { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl RunnerRepository for PgRunnerRepository {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_name: &str,
        owner_type: &RunnerOwnerType,
    ) -> Result<Runner, DatabaseError> {
        let runner = sqlx::query_as::<_, Runner>(
            r#"
            INSERT INTO ci.runners (name, owner_id, owner_name, owner_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, owner_id, owner_name, owner_type, last_active, created_at
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_name)
        .bind(owner_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(runner)
    }

    async fn get(
        &self,
        owner_name: &str,
        runner_name: &str,
    ) -> Result<Option<Runner>, DatabaseError> {
        let runner = sqlx::query_as::<_, Runner>(
            r#"
            SELECT r.id, r.name, r.owner_id, r.owner_name, r.owner_type, r.last_active, r.created_at
            FROM ci.runners r
            WHERE r.name = $2
              AND r.owner_name = $1
            "#,
        )
        .bind(owner_name)
        .bind(runner_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(runner)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        let result = sqlx::query("DELETE FROM ci.runners WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::RowNotFound);
        }

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Runner>, DatabaseError> {
        let runner = sqlx::query_as::<_, Runner>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, last_active, created_at
            FROM ci.runners WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(runner)
    }

    async fn touch(&self, id: Uuid) -> Result<(), DatabaseError> {
        let result = sqlx::query("UPDATE ci.runners SET last_active = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::RowNotFound);
        }

        Ok(())
    }

    async fn list_by_owner(
        &self,
        owner_name: &str,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Runner>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut runners = sqlx::query_as::<_, Runner>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, last_active, created_at
            FROM ci.runners
            WHERE owner_name = $1
              AND ($2::timestamptz IS NULL OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC
            LIMIT $4
            "#,
        )
        .bind(owner_name)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if runners.len() as i64 > limit {
            runners.pop();
            runners.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((runners, next_cursor))
    }
}
