use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::build::list_builds as api;
use gitdot_core::dto::ListBuildsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_builds(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<api::ListBuildsRequest>,
) -> Result<AppResponse<api::ListBuildsResponse>, AppError> {
    let request = ListBuildsRequest::new(&owner, &repo, query.cursor.as_deref(), query.limit)?;
    state
        .build_service
        .list_builds(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
