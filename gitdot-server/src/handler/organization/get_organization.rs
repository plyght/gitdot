use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_organization as api;
use gitdot_core::dto::GetOrganizationRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_organization(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<api::GetOrganizationResponse>, AppError> {
    let request = GetOrganizationRequest::new(&org_name)?;
    state
        .org_service
        .get_organization(request)
        .await
        .map_err(AppError::from)
        .map(|org| AppResponse::new(StatusCode::OK, org.into_api()))
}
