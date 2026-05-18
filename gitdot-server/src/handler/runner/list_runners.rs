use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::runner::list_runners as api;
use gitdot_core::dto::ListRunnersRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, UserJwt},
};

#[axum::debug_handler]
pub async fn list_runners(
    State(state): State<AppState>,
    _auth_user: Principal<UserJwt>,
    Path(owner): Path<String>,
    Query(query): Query<api::ListRunnersRequest>,
) -> Result<AppResponse<api::ListRunnersResponse>, AppError> {
    let request = ListRunnersRequest::new(&owner, query.cursor.as_deref(), query.limit)?;
    state
        .runner_service
        .list_runners(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
