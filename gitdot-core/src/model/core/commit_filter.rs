use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CommitFilter {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
