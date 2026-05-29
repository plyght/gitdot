use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{
        Migration, MigrationOriginService, MigrationRepository as MigrationRepositoryModel,
        MigrationRepositoryStatus, MigrationStatus, RepositoryOwnerType, RepositoryVisibility,
    },
};

#[async_trait]
pub trait MigrationRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        author_id: Uuid,
        origin_service: MigrationOriginService,
        origin: &str,
        origin_type: &RepositoryOwnerType,
        destination: &str,
        destination_type: &RepositoryOwnerType,
    ) -> Result<Migration, DatabaseError>;

    async fn get(&self, author_id: Uuid, number: i32) -> Result<Option<Migration>, DatabaseError>;

    async fn list(
        &self,
        author_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Migration>, Option<Cursor>), DatabaseError>;

    async fn update_status(
        &self,
        id: Uuid,
        status: MigrationStatus,
    ) -> Result<Migration, DatabaseError>;

    async fn create_migration_repository(
        &self,
        migration_id: Uuid,
        origin_full_name: &str,
        origin_repository_id: i64,
        origin_created_at: Option<DateTime<Utc>>,
        destination_full_name: &str,
        visibility: &RepositoryVisibility,
    ) -> Result<MigrationRepositoryModel, DatabaseError>;

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
        error: Option<&str>,
    ) -> Result<MigrationRepositoryModel, DatabaseError>;

    async fn set_destination_repository_id(
        &self,
        migration_repository_id: Uuid,
        destination_repository_id: Uuid,
    ) -> Result<(), DatabaseError>;

    async fn list_by_origin_repository_id(
        &self,
        origin_repository_id: i64,
    ) -> Result<Vec<MigrationRepositoryModel>, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct MigrationRepositoryImpl {
    pool: PgPool,
}

impl MigrationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl MigrationRepository for MigrationRepositoryImpl {
    async fn create(
        &self,
        author_id: Uuid,
        origin_service: MigrationOriginService,
        origin: &str,
        origin_type: &RepositoryOwnerType,
        destination: &str,
        destination_type: &RepositoryOwnerType,
    ) -> Result<Migration, DatabaseError> {
        let migration = sqlx::query_as::<_, Migration>(
            r#"
            INSERT INTO migration.migrations (number, author_id, origin_service, origin, origin_type, destination, destination_type)
            VALUES (
                COALESCE((SELECT MAX(number) FROM migration.migrations WHERE author_id = $1), 0) + 1,
                $1, $2, $3, $4, $5, $6
            )
            RETURNING id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at, NULL AS repositories
            "#,
        )
        .bind(author_id)
        .bind(origin_service)
        .bind(origin)
        .bind(origin_type)
        .bind(destination)
        .bind(destination_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(migration)
    }

    async fn get(&self, author_id: Uuid, number: i32) -> Result<Option<Migration>, DatabaseError> {
        let migration = sqlx::query_as::<_, Migration>(
            r#"
            SELECT m.id, m.number, m.author_id, m.origin_service, m.origin, m.origin_type,
                   m.destination, m.destination_type, m.status, m.created_at, m.updated_at,
                   COALESCE(
                       (SELECT json_agg(json_build_object(
                           'id', mr.id,
                           'migration_id', mr.migration_id,
                           'origin_full_name', mr.origin_full_name,
                           'origin_repository_id', mr.origin_repository_id,
                           'destination_full_name', mr.destination_full_name,
                           'destination_repository_id', mr.destination_repository_id,
                           'visibility', mr.visibility,
                           'origin_created_at', mr.origin_created_at,
                           'status', mr.status,
                           'error', mr.error,
                           'created_at', mr.created_at,
                           'updated_at', mr.updated_at
                       ) ORDER BY mr.created_at ASC)
                       FROM migration.migration_repositories mr WHERE mr.migration_id = m.id),
                       '[]'::json
                   ) AS repositories
            FROM migration.migrations m
            WHERE m.author_id = $1 AND m.number = $2
            "#,
        )
        .bind(author_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(migration)
    }

    async fn list(
        &self,
        author_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Migration>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.map(|c| c.created_at);
        let cursor_id = cursor.map(|c| c.id);

        let mut migrations = sqlx::query_as::<_, Migration>(
            r#"
            SELECT m.id, m.number, m.author_id, m.origin_service, m.origin, m.origin_type,
                   m.destination, m.destination_type, m.status, m.created_at, m.updated_at,
                   COALESCE(
                       (SELECT json_agg(json_build_object(
                           'id', mr.id,
                           'migration_id', mr.migration_id,
                           'origin_full_name', mr.origin_full_name,
                           'origin_repository_id', mr.origin_repository_id,
                           'destination_full_name', mr.destination_full_name,
                           'destination_repository_id', mr.destination_repository_id,
                           'visibility', mr.visibility,
                           'origin_created_at', mr.origin_created_at,
                           'status', mr.status,
                           'error', mr.error,
                           'created_at', mr.created_at,
                           'updated_at', mr.updated_at
                       ) ORDER BY mr.created_at ASC)
                       FROM migration.migration_repositories mr WHERE mr.migration_id = m.id),
                       '[]'::json
                   ) AS repositories
            FROM migration.migrations m
            WHERE m.author_id = $1
              AND ($2::timestamptz IS NULL OR (m.created_at, m.id) < ($2, $3))
            ORDER BY m.created_at DESC, m.id DESC
            LIMIT $4
            "#,
        )
        .bind(author_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if migrations.len() as i64 > limit {
            migrations.pop();
            migrations.last().map(|m| Cursor {
                created_at: m.created_at,
                id: m.id,
            })
        } else {
            None
        };

        Ok((migrations, next_cursor))
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: MigrationStatus,
    ) -> Result<Migration, DatabaseError> {
        let migration = sqlx::query_as::<_, Migration>(
            r#"
            UPDATE migration.migrations SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at, NULL AS repositories
            "#,
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(migration)
    }

    async fn create_migration_repository(
        &self,
        migration_id: Uuid,
        origin_full_name: &str,
        origin_repository_id: i64,
        origin_created_at: Option<DateTime<Utc>>,
        destination_full_name: &str,
        visibility: &RepositoryVisibility,
    ) -> Result<MigrationRepositoryModel, DatabaseError> {
        let repo = sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            INSERT INTO migration.migration_repositories
                (migration_id, origin_full_name, origin_repository_id, destination_full_name, visibility, origin_created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, migration_id, origin_full_name, origin_repository_id, destination_full_name, destination_repository_id, visibility, origin_created_at, status, error, created_at, updated_at
            "#,
        )
        .bind(migration_id)
        .bind(origin_full_name)
        .bind(origin_repository_id)
        .bind(destination_full_name)
        .bind(visibility)
        .bind(origin_created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(repo)
    }

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
        error: Option<&str>,
    ) -> Result<MigrationRepositoryModel, DatabaseError> {
        let repo = sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            UPDATE migration.migration_repositories
            SET status = $2, error = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, migration_id, origin_full_name, origin_repository_id, destination_full_name, destination_repository_id, visibility, origin_created_at, status, error, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(error)
        .fetch_one(&self.pool)
        .await?;

        Ok(repo)
    }

    async fn set_destination_repository_id(
        &self,
        migration_repository_id: Uuid,
        destination_repository_id: Uuid,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE migration.migration_repositories
            SET destination_repository_id = $2, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(migration_repository_id)
        .bind(destination_repository_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_by_origin_repository_id(
        &self,
        origin_repository_id: i64,
    ) -> Result<Vec<MigrationRepositoryModel>, DatabaseError> {
        let rows = sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            SELECT id, migration_id, origin_full_name, origin_repository_id,
                   destination_full_name, destination_repository_id, visibility,
                   origin_created_at, status, error, created_at, updated_at
            FROM migration.migration_repositories
            WHERE origin_repository_id = $1 AND destination_repository_id IS NOT NULL
            "#,
        )
        .bind(origin_repository_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{
        MigrationOriginService, MigrationRepository, MigrationRepositoryImpl,
        MigrationRepositoryStatus, MigrationStatus, RepositoryOwnerType, RepositoryVisibility,
    };
    use crate::repository::test_common::{insert_migration_at, insert_user, insert_user_repo};

    #[sqlx::test]
    async fn create_assigns_sequential_numbers_per_author(pool: PgPool) {
        let repo = MigrationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;

        let first = repo
            .create(
                alice,
                MigrationOriginService::GitHub,
                "octo/repo",
                &RepositoryOwnerType::User,
                "alice/repo",
                &RepositoryOwnerType::User,
            )
            .await
            .unwrap();
        assert_eq!(first.number, 1);
        assert_eq!(first.author_id, alice);
        assert_eq!(first.origin_service, MigrationOriginService::GitHub);
        assert_eq!(first.origin, "octo/repo");
        assert_eq!(first.origin_type, RepositoryOwnerType::User);
        assert_eq!(first.destination, "alice/repo");
        assert_eq!(first.status, MigrationStatus::Pending);
        assert!(first.repositories.is_none());

        // Numbers increment per author.
        let second = repo
            .create(
                alice,
                MigrationOriginService::GitHub,
                "octo/two",
                &RepositoryOwnerType::User,
                "alice/two",
                &RepositoryOwnerType::User,
            )
            .await
            .unwrap();
        assert_eq!(second.number, 2);

        // A different author starts numbering again at 1.
        let bob_first = repo
            .create(
                bob,
                MigrationOriginService::GitHub,
                "octo/b",
                &RepositoryOwnerType::Organization,
                "bob/b",
                &RepositoryOwnerType::Organization,
            )
            .await
            .unwrap();
        assert_eq!(bob_first.number, 1);
        assert_eq!(bob_first.origin_type, RepositoryOwnerType::Organization);
    }

    #[sqlx::test]
    async fn get_returns_migration_with_repositories(pool: PgPool) {
        let repo = MigrationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let migration = repo
            .create(
                alice,
                MigrationOriginService::GitHub,
                "octo/repo",
                &RepositoryOwnerType::User,
                "alice/repo",
                &RepositoryOwnerType::User,
            )
            .await
            .unwrap();
        repo.create_migration_repository(
            migration.id,
            "octo/repo",
            42,
            None,
            "alice/repo",
            &RepositoryVisibility::Public,
        )
        .await
        .unwrap();

        let fetched = repo
            .get(alice, migration.number)
            .await
            .unwrap()
            .expect("found");
        let repos = fetched.repositories.expect("repositories projected");
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].origin_full_name, "octo/repo");
        assert_eq!(repos[0].origin_repository_id, 42);
        assert_eq!(repos[0].visibility, RepositoryVisibility::Public);
        assert_eq!(repos[0].status, MigrationRepositoryStatus::Pending);

        assert!(repo.get(alice, 999).await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn list_paginates_newest_first(pool: PgPool) {
        let repo = MigrationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let now = Utc::now();
        insert_migration_at(&pool, alice, 1, now - Duration::days(3)).await;
        insert_migration_at(&pool, alice, 2, now - Duration::days(2)).await;
        insert_migration_at(&pool, alice, 3, now - Duration::days(1)).await;

        let (page, cursor) = repo.list(alice, None, 2).await.unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].number, 3);
        assert_eq!(page[1].number, 2);
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo.list(alice, Some(cursor), 2).await.unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].number, 1);
        assert!(cursor.is_none());
    }

    #[sqlx::test]
    async fn update_status_changes_status(pool: PgPool) {
        let repo = MigrationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let migration = repo
            .create(
                alice,
                MigrationOriginService::GitHub,
                "o",
                &RepositoryOwnerType::User,
                "d",
                &RepositoryOwnerType::User,
            )
            .await
            .unwrap();
        assert_eq!(migration.status, MigrationStatus::Pending);

        let updated = repo
            .update_status(migration.id, MigrationStatus::Completed)
            .await
            .unwrap();
        assert_eq!(updated.status, MigrationStatus::Completed);
    }

    #[sqlx::test]
    async fn migration_repository_create_and_update_status(pool: PgPool) {
        let repo = MigrationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let migration = repo
            .create(
                alice,
                MigrationOriginService::GitHub,
                "o",
                &RepositoryOwnerType::User,
                "d",
                &RepositoryOwnerType::User,
            )
            .await
            .unwrap();

        let created = repo
            .create_migration_repository(
                migration.id,
                "octo/repo",
                99,
                None,
                "alice/repo",
                &RepositoryVisibility::Private,
            )
            .await
            .unwrap();
        assert_eq!(created.migration_id, migration.id);
        assert_eq!(created.origin_repository_id, 99);
        assert_eq!(created.visibility, RepositoryVisibility::Private);
        assert_eq!(created.status, MigrationRepositoryStatus::Pending);
        assert!(created.error.is_none());
        assert!(created.destination_repository_id.is_none());

        let failed = repo
            .update_migration_repository_status(
                created.id,
                MigrationRepositoryStatus::Failed,
                Some("boom"),
            )
            .await
            .unwrap();
        assert_eq!(failed.status, MigrationRepositoryStatus::Failed);
        assert_eq!(failed.error.as_deref(), Some("boom"));
    }

    #[sqlx::test]
    async fn set_destination_and_list_by_origin_repository_id(pool: PgPool) {
        let repo = MigrationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let migration = repo
            .create(
                alice,
                MigrationOriginService::GitHub,
                "o",
                &RepositoryOwnerType::User,
                "d",
                &RepositoryOwnerType::User,
            )
            .await
            .unwrap();

        // Two migration repos share an origin id; only one gets a destination.
        let linked = repo
            .create_migration_repository(
                migration.id,
                "octo/repo",
                555,
                None,
                "alice/repo",
                &RepositoryVisibility::Public,
            )
            .await
            .unwrap();
        repo.create_migration_repository(
            migration.id,
            "octo/repo",
            555,
            None,
            "alice/repo2",
            &RepositoryVisibility::Public,
        )
        .await
        .unwrap();

        let dest_repo = Uuid::new_v4();
        insert_user_repo(&pool, dest_repo, "repo", alice, "public").await;
        repo.set_destination_repository_id(linked.id, dest_repo)
            .await
            .unwrap();

        // Only the row whose destination is set is returned.
        let rows = repo.list_by_origin_repository_id(555).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, linked.id);
        assert_eq!(rows[0].destination_repository_id, Some(dest_repo));

        // An unknown origin id matches nothing.
        assert!(
            repo.list_by_origin_repository_id(123)
                .await
                .unwrap()
                .is_empty()
        );
    }
}
