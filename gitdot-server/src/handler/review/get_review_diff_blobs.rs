use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_review_diff_blobs as api;
use gitdot_core::dto::{
    GetReviewDiffBlobsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_review_diff_blobs(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
    Query(query): Query<api::GetReviewDiffBlobsRequest>,
) -> Result<AppResponse<api::GetReviewDiffBlobsResponse>, AppError> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = GetReviewDiffBlobsRequest::new(
        &owner,
        &repo,
        number,
        position,
        query.revision,
        query.compare_to,
    )?;
    state
        .review_service
        .get_review_diff_blobs(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
