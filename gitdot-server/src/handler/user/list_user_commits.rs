use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_commits as api;
use gitdot_core::dto::ListUserCommitsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_user_commits(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
    Query(params): Query<api::ListUserCommitsRequest>,
) -> Result<AppResponse<api::ListUserCommitsResponse>, AppError> {
    let request = ListUserCommitsRequest::new(
        &user_name,
        auth_user.map(|u| u.id),
        params.from,
        params.to,
        params.cursor.as_deref(),
        params.limit,
    )?;
    state
        .user_service
        .list_commits(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
