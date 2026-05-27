use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub provider: AuthProvider,

    // user metadata provided by user
    pub display_name: Option<String>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,

    pub created_at: DateTime<Utc>,

    #[sqlx(json)]
    pub emails: Vec<UserEmail>,
}

impl User {
    pub fn primary_email(&self) -> Option<&UserEmail> {
        self.emails.iter().find(|e| e.is_primary)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "core.auth_provider", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    Email,
    GitHub,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserEmail {
    pub id: Uuid,
    pub user_id: Uuid,

    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,

    pub created_at: DateTime<Utc>,
}
