use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::list_repository_commit_filters as api;
use gitdot_core::dto::{
    ListRepositoryCommitFiltersRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_repository_commit_filters(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<api::ListRepositoryCommitFiltersRequest>,
) -> Result<AppResponse<api::ListRepositoryCommitFiltersResponse>, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = ListRepositoryCommitFiltersRequest::new(
        user_id,
        &owner,
        &repo,
        query.cursor.as_deref(),
        query.limit,
    )?;
    state
        .repo_service
        .list_repository_commit_filters(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
