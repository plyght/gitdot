use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_repository_commits as api;
use gitdot_core::dto::{
    ListRepositoryCommitsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_repository_commits(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::ListRepositoryCommitsRequest>,
) -> Result<AppResponse<api::ListRepositoryCommitsResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = ListRepositoryCommitsRequest::new(
        &owner,
        &repo,
        params.ref_name,
        params.from,
        params.to,
        params.cursor.as_deref(),
        params.limit,
    )?;
    state
        .repo_service
        .list_repository_commits(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
