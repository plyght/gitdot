use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_repositories_contributed as api;
use gitdot_core::dto::ListUserContributedRepositoriesRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_user_contributed_repositories(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
    Query(query): Query<api::ListUserContributedRepositoriesRequest>,
) -> Result<AppResponse<api::ListUserContributedRepositoriesResponse>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListUserContributedRepositoriesRequest::new(
        &user_name,
        viewer_id,
        query.from,
        query.cursor.as_deref(),
        query.limit,
    )?;
    state
        .user_service
        .list_contributed_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
