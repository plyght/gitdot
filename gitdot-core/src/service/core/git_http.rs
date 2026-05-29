use async_trait::async_trait;

use crate::{
    client::{GitHttpClient, GitHttpClientImpl},
    dto::{GitHttpResponse, InfoRefsRequest, ReceivePackRequest, UploadPackRequest},
    error::GitHttpError,
};

/// Smart HTTP git protocol surface. Delegates to a [`GitHttpClient`] that shells
/// out to `git http-backend` (CGI) against the bare repo for the given owner/repo,
/// returning the backend's raw response (status, headers, streamed body).
#[async_trait]
pub trait GitHttpService: Send + Sync + 'static {
    /// Handles `GET /info/refs?service=...`, the ref-advertisement step of a
    /// clone/fetch/push handshake. Forwards `service` (e.g. `git-upload-pack` or
    /// `git-receive-pack`) to `git http-backend`.
    async fn info_refs(&self, request: InfoRefsRequest) -> Result<GitHttpResponse, GitHttpError>;

    /// Handles the `git-upload-pack` RPC (clone/fetch): streams `request.body`
    /// into `git http-backend` and returns the packfile response. No extra
    /// environment is injected.
    async fn upload_pack(
        &self,
        request: UploadPackRequest,
    ) -> Result<GitHttpResponse, GitHttpError>;

    /// Handles the `git-receive-pack` RPC (push): streams `request.body` into
    /// `git http-backend`. When `request.pusher_id` is set it is exported as the
    /// `GITDOT_PUSHER_ID` environment variable so the receive hooks can attribute
    /// the push.
    async fn receive_pack(
        &self,
        request: ReceivePackRequest,
    ) -> Result<GitHttpResponse, GitHttpError>;
}

#[derive(Debug, Clone)]
pub struct GitHttpServiceImpl<G>
where
    G: GitHttpClient,
{
    git_http_client: G,
}

impl GitHttpServiceImpl<GitHttpClientImpl> {
    pub fn new(git_http_client: GitHttpClientImpl) -> Self {
        Self { git_http_client }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<G> GitHttpService for GitHttpServiceImpl<G>
where
    G: GitHttpClient,
{
    async fn info_refs(&self, request: InfoRefsRequest) -> Result<GitHttpResponse, GitHttpError> {
        self.git_http_client
            .info_refs(&request.owner, &request.repo, &request.service)
            .await
    }

    async fn upload_pack(
        &self,
        request: UploadPackRequest,
    ) -> Result<GitHttpResponse, GitHttpError> {
        self.git_http_client
            .service_rpc(
                &request.owner,
                &request.repo,
                "upload-pack",
                &request.content_type,
                request.body,
                vec![],
            )
            .await
    }

    async fn receive_pack(
        &self,
        request: ReceivePackRequest,
    ) -> Result<GitHttpResponse, GitHttpError> {
        let mut env_vars = vec![];
        if let Some(pusher_id) = request.pusher_id {
            env_vars.push(("GITDOT_PUSHER_ID".to_string(), pusher_id.to_string()));
        }

        self.git_http_client
            .service_rpc(
                &request.owner,
                &request.repo,
                "receive-pack",
                &request.content_type,
                request.body,
                env_vars,
            )
            .await
    }
}
