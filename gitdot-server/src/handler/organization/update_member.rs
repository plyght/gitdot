use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::update_organization_member as api;
use gitdot_core::dto::{
    OrganizationMemberAuthorizationRequest, UpdateOrganizationMemberRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_member(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((org_name, member_id)): Path<(String, Uuid)>,
    Json(request): Json<api::UpdateOrganizationMemberRequest>,
) -> Result<AppResponse<api::UpdateOrganizationMemberResponse>, AppError> {
    let auth_request =
        OrganizationMemberAuthorizationRequest::new(auth_user.id, &org_name, member_id)?;
    state
        .authorization_service
        .verify_authorized_for_organization_member(auth_request)
        .await?;

    let request =
        UpdateOrganizationMemberRequest::new(&org_name, member_id, request.role_description)?;
    state
        .org_service
        .update_member(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
