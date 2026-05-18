use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::{InputError, RunnerError},
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListRunnersRequest {
    pub owner_name: OwnerName,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListRunnersRequest {
    pub fn new(
        owner_name: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, RunnerError> {
        let owner_name =
            OwnerName::try_new(owner_name).map_err(|e| InputError::new("owner name", e))?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            owner_name,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
