use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::create_repository_commit_filter as api;
use gitdot_core::dto::{
    CreateRepositoryCommitFilterRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_repository_commit_filter(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<api::CreateRepositoryCommitFilterRequest>,
) -> Result<AppResponse<api::CreateRepositoryCommitFilterResponse>, AppError> {
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

    let core_request = CreateRepositoryCommitFilterRequest::new(
        auth_user.id,
        &owner,
        &repo,
        &request.name,
        request.authors,
        request.tags,
        request.paths,
    )?;
    state
        .repo_service
        .create_repository_commit_filter(core_request)
        .await
        .map_err(AppError::from)
        .map(|filter| AppResponse::new(StatusCode::CREATED, filter.into_api()))
}
