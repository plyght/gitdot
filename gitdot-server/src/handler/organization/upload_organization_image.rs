use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
};

use gitdot_core::{
    dto::{OrganizationAuthorizationRequest, UpdateOrganizationImageRequest},
    error::{ImageError, OrganizationError},
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn upload_organization_image(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<AppResponse<()>, AppError> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !matches!(content_type, "image/jpeg" | "image/png" | "image/webp") {
        return Err(
            OrganizationError::InvalidImage(ImageError::DecodeError(format!(
                "unsupported image type: {content_type}"
            )))
            .into(),
        );
    }

    let auth_request = OrganizationAuthorizationRequest::new(auth_user.id, &org_name)?;
    state
        .authorization_service
        .verify_authorized_for_organization(auth_request)
        .await?;

    state
        .org_service
        .update_organization_image(UpdateOrganizationImageRequest::new(&org_name, body)?)
        .await?;

    Ok(AppResponse::new(StatusCode::NO_CONTENT, ()))
}
