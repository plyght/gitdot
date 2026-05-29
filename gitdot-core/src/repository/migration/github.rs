use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{GitHubInstallation, GitHubInstallationType},
};

#[async_trait]
pub trait GitHubRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        installation_id: i64,
        owner_id: Uuid,
        installation_type: GitHubInstallationType,
        github_login: &str,
    ) -> Result<GitHubInstallation, DatabaseError>;

    async fn get(
        &self,
        owner_id: Uuid,
        installation_id: i64,
    ) -> Result<Option<GitHubInstallation>, DatabaseError>;

    async fn delete_by_installation_id(&self, installation_id: i64) -> Result<(), DatabaseError>;

    async fn list_by_owner(
        &self,
        owner_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<GitHubInstallation>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct GitHubRepositoryImpl {
    pool: PgPool,
}

impl GitHubRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl GitHubRepository for GitHubRepositoryImpl {
    async fn create(
        &self,
        installation_id: i64,
        owner_id: Uuid,
        installation_type: GitHubInstallationType,
        github_login: &str,
    ) -> Result<GitHubInstallation, DatabaseError> {
        let installation = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            INSERT INTO migration.github_installations (installation_id, owner_id, type, github_login)
            VALUES ($1, $2, $3, $4)
            RETURNING id, installation_id, owner_id, type, github_login, created_at
            "#,
        )
        .bind(installation_id)
        .bind(owner_id)
        .bind(installation_type)
        .bind(github_login)
        .fetch_one(&self.pool)
        .await?;

        Ok(installation)
    }

    async fn get(
        &self,
        owner_id: Uuid,
        installation_id: i64,
    ) -> Result<Option<GitHubInstallation>, DatabaseError> {
        let installation = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            SELECT id, installation_id, owner_id, type, github_login, created_at
            FROM migration.github_installations
            WHERE owner_id = $1 AND installation_id = $2
            "#,
        )
        .bind(owner_id)
        .bind(installation_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(installation)
    }

    async fn delete_by_installation_id(&self, installation_id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM migration.github_installations WHERE installation_id = $1")
            .bind(installation_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_owner(
        &self,
        owner_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<GitHubInstallation>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut installations = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            SELECT id, installation_id, owner_id, type, github_login, created_at
            FROM migration.github_installations
            WHERE owner_id = $1
              AND ($2::timestamptz IS NULL OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC
            LIMIT $4
            "#,
        )
        .bind(owner_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if installations.len() as i64 > limit {
            installations.pop();
            installations.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((installations, next_cursor))
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{GitHubInstallationType, GitHubRepository, GitHubRepositoryImpl};
    use crate::repository::test_common::{insert_installation_at, insert_user};

    #[sqlx::test]
    async fn create_and_get_installation(pool: PgPool) {
        let repo = GitHubRepositoryImpl::new(pool.clone());
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        insert_user(&pool, owner, "alice").await;
        insert_user(&pool, other, "bob").await;

        let created = repo
            .create(1001, owner, GitHubInstallationType::User, "octocat")
            .await
            .unwrap();
        assert_eq!(created.installation_id, 1001);
        assert_eq!(created.owner_id, owner);
        assert_eq!(created.r#type, GitHubInstallationType::User);
        assert_eq!(created.github_login, "octocat");

        let found = repo.get(owner, 1001).await.unwrap().expect("found");
        assert_eq!(found.id, created.id);

        // Lookup is scoped to both owner and installation id.
        assert!(repo.get(owner, 9999).await.unwrap().is_none());
        assert!(repo.get(other, 1001).await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn delete_by_installation_id_removes(pool: PgPool) {
        let repo = GitHubRepositoryImpl::new(pool.clone());
        let owner = Uuid::new_v4();
        insert_user(&pool, owner, "alice").await;
        repo.create(1001, owner, GitHubInstallationType::Organization, "acme")
            .await
            .unwrap();

        repo.delete_by_installation_id(1001).await.unwrap();
        assert!(repo.get(owner, 1001).await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn list_by_owner_paginates_newest_first(pool: PgPool) {
        let repo = GitHubRepositoryImpl::new(pool.clone());
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        insert_user(&pool, owner, "alice").await;
        insert_user(&pool, other, "bob").await;
        let now = Utc::now();
        insert_installation_at(&pool, 1, owner, "first", now - Duration::days(3)).await;
        insert_installation_at(&pool, 2, owner, "second", now - Duration::days(2)).await;
        insert_installation_at(&pool, 3, owner, "third", now - Duration::days(1)).await;
        // Another owner's installation must not appear.
        insert_installation_at(&pool, 4, other, "other", now).await;

        let (page, cursor) = repo.list_by_owner(owner, None, 2).await.unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].installation_id, 3);
        assert_eq!(page[1].installation_id, 2);
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo.list_by_owner(owner, Some(cursor), 2).await.unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].installation_id, 1);
        assert!(cursor.is_none());
    }
}
