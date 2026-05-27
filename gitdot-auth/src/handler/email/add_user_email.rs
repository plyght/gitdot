use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::email::add_user_email as api;
use gitdot_axum::extract::Principal;
use gitdot_core::dto::AddUserEmailRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

pub async fn add_user_email(
    principal: Principal,
    State(state): State<AppState>,
    Json(body): Json<api::AddUserEmailRequest>,
) -> Result<AppResponse<api::AddUserEmailResponse>, AppError> {
    let request = AddUserEmailRequest::new(principal.id, &body.email)?;
    state
        .email_verification_service
        .add_email(request)
        .await
        .map_err(AppError::from)
        .map(|email| AppResponse::new(StatusCode::CREATED, email.into_api()))
}
