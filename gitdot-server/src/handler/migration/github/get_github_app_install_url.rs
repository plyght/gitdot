use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::github::get_github_app_install_url as api;
use gitdot_core::dto::GetGitHubAppInstallUrlRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_github_app_install_url(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Query(query): Query<api::GetGitHubAppInstallUrlRequest>,
) -> Result<AppResponse<api::GetGitHubAppInstallUrlResponse>, AppError> {
    let request = GetGitHubAppInstallUrlRequest::new(auth_user.id, &query.action)?;
    state
        .migration_service
        .get_github_app_install_url(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
