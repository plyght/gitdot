use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_organizations as api;
use gitdot_core::dto::ListOrganizationsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn list_organizations(
    State(state): State<AppState>,
    Query(query): Query<api::ListOrganizationsRequest>,
) -> Result<AppResponse<api::ListOrganizationsResponse>, AppError> {
    let request = ListOrganizationsRequest::new(query.cursor.as_deref(), query.limit)?;
    state
        .org_service
        .list_organizations(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
