use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{EmailVerificationCode, UserEmail},
};

/// sqlx data-access layer for the `auth.email_verification_codes` table, with
/// writes against `core.user_emails` when consuming a code.
#[async_trait]
pub trait EmailVerificationRepository: Send + Sync + Clone + 'static {
    /// Inserts a verification code (`user_id`, `email`, hashed code, expiry) and
    /// returns the created row.
    async fn create_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, DatabaseError>;

    /// Returns the verification code matching `code_hash`, or `Ok(None)` if none
    /// exists. Does not check expiry or `used_at`.
    async fn get_code_by_hash(
        &self,
        code_hash: &str,
    ) -> Result<Option<EmailVerificationCode>, DatabaseError>;

    /// Invalidates all outstanding codes for `(user_id, email)` by setting
    /// `used_at = NOW()` on rows where they match and `used_at IS NULL`.
    async fn invalidate_codes_for_email(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<(), DatabaseError>;

    /// In a single transaction: marks the code `used_at = NOW()`, then inserts
    /// the verified `core.user_emails` row for `(user_id, email)` and returns it.
    /// If the user already has a row for this email it is flipped to verified
    /// instead. A `23505` surfaces when a *different* user has already verified
    /// the address.
    async fn mark_code_used_and_add_email(
        &self,
        code_id: Uuid,
        user_id: Uuid,
        email: &str,
    ) -> Result<UserEmail, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgEmailVerificationRepository {
    pool: PgPool,
}

impl PgEmailVerificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl EmailVerificationRepository for PgEmailVerificationRepository {
    async fn create_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, DatabaseError> {
        let code = sqlx::query_as::<_, EmailVerificationCode>(
            r#"
            INSERT INTO auth.email_verification_codes (user_id, email, code_hash, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(email)
        .bind(code_hash)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(code)
    }

    async fn get_code_by_hash(
        &self,
        code_hash: &str,
    ) -> Result<Option<EmailVerificationCode>, DatabaseError> {
        let code = sqlx::query_as::<_, EmailVerificationCode>(
            r#"
            SELECT * FROM auth.email_verification_codes WHERE code_hash = $1
            "#,
        )
        .bind(code_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(code)
    }

    async fn invalidate_codes_for_email(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE auth.email_verification_codes
            SET used_at = NOW()
            WHERE user_id = $1 AND email = $2 AND used_at IS NULL
            "#,
        )
        .bind(user_id)
        .bind(email)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn mark_code_used_and_add_email(
        &self,
        code_id: Uuid,
        user_id: Uuid,
        email: &str,
    ) -> Result<UserEmail, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
            UPDATE auth.email_verification_codes SET used_at = NOW() WHERE id = $1
            "#,
        )
        .bind(code_id)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query_as::<_, UserEmail>(
            r#"
            INSERT INTO core.user_emails (user_id, email, is_primary, is_verified, verified_at)
            VALUES ($1, $2, FALSE, TRUE, NOW())
            ON CONFLICT (user_id, email)
            DO UPDATE SET is_verified = TRUE,
                          verified_at = COALESCE(core.user_emails.verified_at, NOW())
            RETURNING id, user_id, email, is_primary, is_verified, created_at
            "#,
        )
        .bind(user_id)
        .bind(email)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row)
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{EmailVerificationRepository, PgEmailVerificationRepository};
    use crate::repository::test_common::insert_user;

    #[sqlx::test]
    async fn create_and_get_code(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        let code = repo
            .create_code(user, "alice@x.com", "hash", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        assert_eq!(code.user_id, user);
        assert_eq!(code.email, "alice@x.com");
        assert_eq!(code.code_hash, "hash");
        assert!(code.used_at.is_none());

        let found = repo.get_code_by_hash("hash").await.unwrap().expect("found");
        assert_eq!(found.id, code.id);
        assert!(repo.get_code_by_hash("missing").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn invalidate_codes_marks_outstanding_used(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        repo.create_code(user, "alice@x.com", "h1", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        repo.create_code(user, "alice@x.com", "h2", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        repo.invalidate_codes_for_email(user, "alice@x.com")
            .await
            .unwrap();

        assert!(
            repo.get_code_by_hash("h1")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
        assert!(
            repo.get_code_by_hash("h2")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
    }

    #[sqlx::test]
    async fn mark_code_used_consumes_code_and_creates_verified_email(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;

        let code = repo
            .create_code(
                alice,
                "alice2@x.com",
                "hash",
                Utc::now() + Duration::hours(1),
            )
            .await
            .unwrap();

        let row = repo
            .mark_code_used_and_add_email(code.id, alice, "alice2@x.com")
            .await
            .unwrap();

        // The code is consumed and alice's verified row is created.
        assert!(
            repo.get_code_by_hash("hash")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
        assert_eq!(row.user_id, alice);
        assert_eq!(row.email, "alice2@x.com");
        assert!(row.is_verified);
    }
}
