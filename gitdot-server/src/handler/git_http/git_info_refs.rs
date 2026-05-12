use axum::extract::{Path, Query, State};

use gitdot_core::{
    dto::{
        GitHttpBody, GitHttpResponse, InfoRefsRequest, RepositoryAuthorizationRequest,
        RepositoryPermission,
    },
    error::AuthorizationError,
};

use crate::{
    app::{AppError, AppState},
    dto::{GitHttpServerResponse, InfoRefsQuery},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn git_info_refs(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<GitHttpServerResponse, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let permission = match params.service.as_str() {
        "git-receive-pack" => RepositoryPermission::Write,
        _ => RepositoryPermission::Read,
    };
    let auth_request = RepositoryAuthorizationRequest::new(user_id, &owner, &repo, permission)?;
    match state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await
    {
        Ok(()) => {}
        // surface the readonly state via the smart protocol so `git push` shows
        // a useful `remote: error: …` message instead of an opaque 403.
        Err(AuthorizationError::ReadonlyRepository) => {
            return Ok(smart_protocol_error_response(
                &params.service,
                "repository is readonly; push not allowed",
            ));
        }
        Err(e) => return Err(e.into()),
    }

    let request = InfoRefsRequest::new(&owner, &repo, &params.service)?;
    let response = state.git_http_service.info_refs(request).await?;
    Ok(response.into())
}

fn smart_protocol_error_response(service: &str, message: &str) -> GitHttpServerResponse {
    let mut body = Vec::new();
    body.extend_from_slice(pkt_line(&format!("# service={service}\n")).as_bytes());
    body.extend_from_slice(b"0000");
    body.extend_from_slice(pkt_line(&format!("ERR {message}\n")).as_bytes());
    body.extend_from_slice(b"0000");

    GitHttpResponse {
        status_code: 200,
        headers: vec![
            (
                "Content-Type".to_string(),
                format!("application/x-{service}-advertisement"),
            ),
            ("Cache-Control".to_string(), "no-cache".to_string()),
        ],
        body: GitHttpBody::Buffered(body),
    }
    .into()
}

fn pkt_line(s: &str) -> String {
    format!("{:04x}{}", s.len() + 4, s)
}
