use chrono::Utc;
use uuid::Uuid;

use crate::model::{
    AuthProvider, Commit, CommitRepository, Repository, RepositoryOwnerType, RepositoryVisibility,
    User,
};

pub fn create_user(name: &str) -> User {
    User {
        id: Uuid::new_v4(),
        name: name.to_string(),
        provider: AuthProvider::Email,
        display_name: None,
        location: None,
        readme: None,
        links: vec![],
        created_at: Utc::now(),
        image_updated_at: Utc::now(),
        emails: vec![],
    }
}

pub fn create_repository(
    owner_id: Uuid,
    owner_type: RepositoryOwnerType,
    visibility: RepositoryVisibility,
) -> Repository {
    Repository {
        id: Uuid::new_v4(),
        name: "myrepo".to_string(),
        owner_id,
        owner_name: "owner".to_string(),
        owner_type,
        visibility,
        description: None,
        stars: 0,
        user_star: false,
        readonly: false,
        created_at: Utc::now(),
    }
}

pub fn create_commit(sha: &str) -> Commit {
    Commit {
        id: Uuid::new_v4(),
        repo_id: Uuid::new_v4(),
        author_id: Some(Uuid::new_v4()),
        author_name: Some("alice".to_string()),
        author_image_updated_at: None,
        git_author_name: "Alice".to_string(),
        git_author_email: "alice@example.com".to_string(),
        ref_name: "refs/heads/main".to_string(),
        sha: sha.to_string(),
        parent_sha: String::new(),
        message: "msg".to_string(),
        created_at: Utc::now(),
        review_number: None,
        diff_position: None,
        diffs: vec![],
        repository: CommitRepository {
            id: Uuid::new_v4(),
            owner_name: "alice".to_string(),
            name: "repo".to_string(),
            visibility: "public".to_string(),
        },
    }
}
