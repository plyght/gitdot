use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT},
    error::OrganizationError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListOrganizationsRequest {
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListOrganizationsRequest {
    pub fn new(cursor: Option<&str>, limit: Option<u32>) -> Result<Self, OrganizationError> {
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
