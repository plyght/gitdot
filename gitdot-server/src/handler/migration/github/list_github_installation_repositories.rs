use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::github::list_github_installation_repositories as api;
use gitdot_core::dto::ListGitHubInstallationRepositoriesRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_github_installation_repositories(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(installation_id): Path<i64>,
) -> Result<AppResponse<api::ListGitHubInstallationRepositoriesResponse>, AppError> {
    let request = ListGitHubInstallationRepositoriesRequest {
        owner_id: auth_user.id,
        installation_id,
    };
    state
        .migration_service
        .list_github_installation_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|repos| AppResponse::new(StatusCode::OK, repos.into_api()))
}
