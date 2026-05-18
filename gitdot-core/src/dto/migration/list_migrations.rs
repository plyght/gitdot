use uuid::Uuid;

use crate::{
    dto::common::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT},
    error::MigrationError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListMigrationsRequest {
    pub user_id: Uuid,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListMigrationsRequest {
    pub fn new(
        user_id: Uuid,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, MigrationError> {
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            user_id,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
