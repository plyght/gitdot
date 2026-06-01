use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EmailVerificationCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub code_hash: String,
    pub attempt_count: i16,

    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}
