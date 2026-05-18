use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Build, BuildTrigger, BuildWithStats},
};

#[async_trait]
pub trait BuildRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        repository_id: Uuid,
        trigger: BuildTrigger,
        commit_sha: &str,
        ref_name: &str,
    ) -> Result<Build, DatabaseError>;

    async fn get(&self, repository_id: Uuid, number: i32) -> Result<Option<Build>, DatabaseError>;

    async fn list_by_repo(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<BuildWithStats>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct BuildRepositoryImpl {
    pool: PgPool,
}

impl BuildRepositoryImpl {
    pub fn new(pool: PgPool) -> BuildRepositoryImpl {
        BuildRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl BuildRepository for BuildRepositoryImpl {
    async fn create(
        &self,
        repository_id: Uuid,
        trigger: BuildTrigger,
        commit_sha: &str,
        ref_name: &str,
    ) -> Result<Build, DatabaseError> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            INSERT INTO ci.builds (repository_id, trigger, commit_sha, ref_name, number)
            VALUES ($1, $2, $3, $4, COALESCE((SELECT MAX(number) FROM ci.builds WHERE repository_id = $1), 0) + 1)
            RETURNING id, number, repository_id, ref_name, trigger, commit_sha, status, created_at
            "#,
        )
        .bind(repository_id)
        .bind(trigger)
        .bind(commit_sha)
        .bind(ref_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }

    async fn get(&self, repository_id: Uuid, number: i32) -> Result<Option<Build>, DatabaseError> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, number, repository_id, ref_name, trigger, commit_sha, status, created_at
            FROM ci.builds WHERE repository_id = $1 AND number = $2
            "#,
        )
        .bind(repository_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(build)
    }

    async fn list_by_repo(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<BuildWithStats>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut builds = sqlx::query_as::<_, BuildWithStats>(
            r#"
            SELECT
                b.id, b.number, b.repository_id, b.ref_name, b.trigger, b.commit_sha,
                CASE
                    WHEN COUNT(t.id) = 0 THEN 'running'::ci.build_status
                    WHEN COUNT(t.id) FILTER (WHERE t.status = 'failure') > 0 THEN 'failure'::ci.build_status
                    WHEN COUNT(t.id) = COUNT(t.id) FILTER (WHERE t.status = 'success') THEN 'success'::ci.build_status
                    ELSE 'running'::ci.build_status
                END AS status,
                CAST(COUNT(t.id) AS INT) AS total_tasks,
                CAST(COUNT(t.id) FILTER (WHERE t.status = 'success') AS INT) AS completed_tasks,
                b.created_at,
                COALESCE(MAX(t.updated_at), b.created_at) AS updated_at
            FROM ci.builds b
            LEFT JOIN ci.tasks t ON t.build_id = b.id
            WHERE b.repository_id = $1
              AND ($2::timestamptz IS NULL OR (b.created_at, b.id) < ($2, $3))
            GROUP BY b.id, b.number, b.repository_id, b.ref_name, b.trigger, b.commit_sha, b.created_at
            ORDER BY b.created_at DESC, b.id DESC
            LIMIT $4
            "#,
        )
        .bind(repository_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if builds.len() as i64 > limit {
            builds.pop();
            builds.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((builds, next_cursor))
    }
}
