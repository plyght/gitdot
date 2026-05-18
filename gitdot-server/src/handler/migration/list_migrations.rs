use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::list_migrations as api;
use gitdot_core::dto::ListMigrationsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_migrations(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Query(query): Query<api::ListMigrationsRequest>,
) -> Result<AppResponse<api::ListMigrationsResponse>, AppError> {
    let request = ListMigrationsRequest::new(auth_user.id, query.cursor.as_deref(), query.limit)?;
    state
        .migration_service
        .list_migrations(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
