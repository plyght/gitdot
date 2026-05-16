use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

use gitdot_core::dto::{
    DeleteRepositoryCommitFilterRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn delete_repository_commit_filter(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, filter_id)): Path<(String, String, Uuid)>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Write,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let core_request = DeleteRepositoryCommitFilterRequest::new(filter_id);
    state
        .repo_service
        .delete_repository_commit_filter(core_request)
        .await?;
    Ok(AppResponse::new(StatusCode::OK, ()))
}
