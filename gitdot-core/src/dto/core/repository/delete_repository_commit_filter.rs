use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DeleteRepositoryCommitFilterRequest {
    pub filter_id: Uuid,
}

impl DeleteRepositoryCommitFilterRequest {
    pub fn new(filter_id: Uuid) -> Self {
        Self { filter_id }
    }
}
