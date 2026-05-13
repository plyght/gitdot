use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{
    RepositoryAuthorizationRequest, RepositoryPermission, UnstarRepositoryRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn unstar_repository(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = UnstarRepositoryRequest::new(auth_user.id, &owner, &repo)?;
    state.repo_service.unstar_repository(request).await?;

    Ok(AppResponse::new(StatusCode::OK, ()))
}
