use std::time::Instant;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_blob as api;
use gitdot_core::dto::{
    GetRepositoryBlobRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_blob(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryBlobRequest>,
) -> Result<AppResponse<api::GetRepositoryBlobResponse>, AppError> {
    let start = Instant::now();

    let auth_request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    let auth_start = Instant::now();
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;
    let auth_ms = auth_start.elapsed().as_millis() as u64;

    let request = GetRepositoryBlobRequest::new(&repo, &owner, params.ref_name, params.path)?;
    let ref_name = request.ref_name.clone();
    let path = request.path.clone();

    let blob_start = Instant::now();
    let result = state.repo_service.get_repository_blob(request).await;
    let blob_ms = blob_start.elapsed().as_millis() as u64;

    tracing::error!(
        elapsed_ms = start.elapsed().as_millis() as u64,
        auth_ms,
        blob_ms,
        %owner,
        %repo,
        %ref_name,
        %path,
        ok = result.is_ok(),
        "get_repository_blob timing"
    );

    result
        .map_err(AppError::from)
        .map(|blob| AppResponse::new(StatusCode::OK, blob.into_api()))
}
