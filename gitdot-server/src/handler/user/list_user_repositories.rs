use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_repositories as api;
use gitdot_core::dto::ListUserRepositoriesRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_user_repositories(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
    Query(query): Query<api::ListUserRepositoriesRequest>,
) -> Result<AppResponse<api::ListUserRepositoriesResponse>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListUserRepositoriesRequest::new(
        &user_name,
        query.cursor.as_deref(),
        query.limit,
        viewer_id,
    )?;
    state
        .user_service
        .list_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
