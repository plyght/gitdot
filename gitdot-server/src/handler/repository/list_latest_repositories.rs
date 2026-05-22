use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::list_latest_repositories as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn list_latest_repositories(
    State(state): State<AppState>,
) -> Result<AppResponse<api::ListLatestRepositoriesResponse>, AppError> {
    state
        .repo_service
        .list_latest_repositories()
        .await
        .map_err(AppError::from)
        .map(|repos| AppResponse::new(StatusCode::OK, repos.into_api()))
}
