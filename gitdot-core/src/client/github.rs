use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use hmac::{Hmac, Mac};
use octocrab::models::{AppId, Installation, InstallationId, InstallationRepositories};
use secrecy::ExposeSecret;
use serde::Deserialize;
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    dto::{GitHubAppInstallAction, GitHubEmail, GitHubMembership, GitHubUser, InstallStatePayload},
    error::GitHubError,
};

const INSTALL_STATE_TTL_SECONDS: i64 = 600;

/// Talks to GitHub on behalf of gitdot across three concerns: user OAuth
/// (sign-in via `user:email`), GitHub App installation management, and webhook
/// signature verification.
///
/// OAuth calls take a user `access_token` per request, App calls authenticate
/// as the gitdot GitHub App, and install-state / webhook helpers are HMAC-signed
/// with the shared gitdot↔GitHub secret.
#[async_trait]
pub trait GitHubClient: Send + Sync + Clone + 'static {
    // OAuth operations

    /// Builds the GitHub OAuth authorize URL for the `user:email` scope,
    /// embedding `state` for CSRF protection. Pure string construction.
    fn get_authorization_url(&self, state: &str) -> String;

    /// Exchanges an OAuth `code` for a user access token.
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the token exchange request failed or
    ///   returned an unexpected body.
    async fn exchange_code(&self, code: &str) -> Result<String, GitHubError>;

    /// Fetches the authenticated user's profile (`GET /user`).
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the request failed or the token is
    ///   invalid.
    async fn get_user(&self, access_token: &str) -> Result<GitHubUser, GitHubError>;

    /// Fetches the authenticated user's email addresses (`GET /user/emails`),
    /// including verification and primary flags.
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the request failed or the token is
    ///   invalid.
    async fn get_user_emails(&self, access_token: &str) -> Result<Vec<GitHubEmail>, GitHubError>;

    /// Fetches `user_name`'s membership in `org_name`, used to confirm org
    /// membership during sign-in.
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the request failed, the token is
    ///   invalid, or the user is not a member (404).
    async fn get_user_membership(
        &self,
        org_name: &str,
        user_name: &str,
        access_token: &str,
    ) -> Result<GitHubMembership, GitHubError>;

    // GitHub App operations

    /// Builds the App installation URL, embedding an HMAC-signed,
    /// 10-minute-expiry state token that binds the install flow to `user_id`
    /// and `action` so the callback can be verified.
    ///
    /// # Errors
    /// - [`GitHubError::InvalidState`] — the state payload could not be
    ///   serialized or signed.
    fn get_github_app_install_url(
        &self,
        user_id: Uuid,
        action: GitHubAppInstallAction,
    ) -> Result<String, GitHubError>;

    /// Verifies the HMAC signature and expiry of an install-state `token`
    /// returned on the installation callback, recovering its payload.
    ///
    /// # Errors
    /// - [`GitHubError::InvalidState`] — the token is malformed, the signature
    ///   does not verify, or the payload has expired.
    fn verify_install_state(&self, token: &str) -> Result<InstallStatePayload, GitHubError>;

    /// Fetches installation metadata as the App.
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the request failed or the
    ///   installation does not exist.
    async fn get_installation(&self, installation_id: u64) -> Result<Installation, GitHubError>;

    /// Mints a short-lived (≈1 hour) installation access token for acting on
    /// the installation's repositories.
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the token could not be minted.
    async fn get_installation_access_token(
        &self,
        installation_id: u64,
    ) -> Result<String, GitHubError>;

    /// Lists the repositories an installation grants access to (currently the
    /// first page of up to 100).
    ///
    /// # Errors
    /// - [`GitHubError::OctocrabError`] — the request failed.
    async fn list_installation_repositories(
        &self,
        installation_id: u64,
    ) -> Result<InstallationRepositories, GitHubError>;

    // Webhook operations

    /// Verifies the `X-Hub-Signature-256` header (`sha256=<hex>`) of an inbound
    /// webhook against the raw `body` using the shared secret. Returns `Ok(())`
    /// when the signature matches.
    ///
    /// # Errors
    /// - [`GitHubError::InvalidSignature`] — the header is malformed or the
    ///   HMAC does not match.
    fn verify_webhook_signature(
        &self,
        body: &[u8],
        signature_header: &str,
    ) -> Result<(), GitHubError>;
}

#[derive(Debug, Clone)]
pub struct OctocrabClient {
    app_client: octocrab::Octocrab,
    oauth_client: octocrab::Octocrab,
    app_slug: String,
    client_id: String,
    client_secret: String,
    gitdot_github_secret: String,
}

impl OctocrabClient {
    pub fn new(
        app_id: u64,
        app_slug: String,
        private_key: String,
        client_id: String,
        client_secret: String,
        gitdot_github_secret: String,
    ) -> Self {
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())
            .expect("Invalid RSA private key PEM");

        let app_client = octocrab::Octocrab::builder()
            .app(AppId(app_id), key)
            .build()
            .expect("Failed to build GitHub App client");

        let oauth_client = octocrab::Octocrab::builder()
            .base_uri("https://github.com")
            .expect("Invalid base URI")
            .add_header(http::header::ACCEPT, "application/json".to_string())
            .build()
            .expect("Failed to build GitHub OAuth client");

        Self {
            app_client,
            oauth_client,
            app_slug,
            client_id,
            client_secret,
            gitdot_github_secret,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl GitHubClient for OctocrabClient {
    fn get_authorization_url(&self, state: &str) -> String {
        let mut url = url::Url::parse("https://github.com/login/oauth/authorize").unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("scope", "user:email")
            .append_pair("state", state);
        url.to_string()
    }

    async fn exchange_code(&self, code: &str) -> Result<String, GitHubError> {
        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
        }

        let response: TokenResponse = self
            .oauth_client
            .post(
                "/login/oauth/access_token",
                Some(&serde_json::json!({
                    "client_id": self.client_id,
                    "client_secret": self.client_secret,
                    "code": code,
                })),
            )
            .await?;

        Ok(response.access_token)
    }

    async fn get_user(&self, access_token: &str) -> Result<GitHubUser, GitHubError> {
        let crab = octocrab::Octocrab::builder()
            .personal_token(access_token.to_string())
            .build()?;
        let user: GitHubUser = crab.get("/user", None::<&()>).await?;
        Ok(user)
    }

    async fn get_user_emails(&self, access_token: &str) -> Result<Vec<GitHubEmail>, GitHubError> {
        let crab = octocrab::Octocrab::builder()
            .personal_token(access_token.to_string())
            .build()?;

        let emails: Vec<GitHubEmail> = crab.get("/user/emails", None::<&()>).await?;
        Ok(emails)
    }

    async fn get_user_membership(
        &self,
        org_name: &str,
        user_name: &str,
        access_token: &str,
    ) -> Result<GitHubMembership, GitHubError> {
        let crab = octocrab::Octocrab::builder()
            .personal_token(access_token.to_string())
            .build()?;
        let path = format!("/orgs/{org_name}/memberships/{user_name}");
        let membership: GitHubMembership = crab.get(path, None::<&()>).await?;
        Ok(membership)
    }

    fn get_github_app_install_url(
        &self,
        user_id: Uuid,
        action: GitHubAppInstallAction,
    ) -> Result<String, GitHubError> {
        let payload = InstallStatePayload {
            user_id,
            action,
            exp: Utc::now().timestamp() + INSTALL_STATE_TTL_SECONDS,
        };
        let payload_json = serde_json::to_vec(&payload).map_err(|_| GitHubError::InvalidState)?;
        let payload_b64 = URL_SAFE_NO_PAD.encode(&payload_json);

        let mut mac = Hmac::<Sha256>::new_from_slice(self.gitdot_github_secret.as_bytes())
            .map_err(|_| GitHubError::InvalidState)?;
        mac.update(payload_b64.as_bytes());
        let signature_b64 = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

        Ok(format!(
            "https://github.com/apps/{}/installations/new?state={payload_b64}.{signature_b64}",
            self.app_slug,
        ))
    }

    fn verify_install_state(&self, token: &str) -> Result<InstallStatePayload, GitHubError> {
        let (payload_b64, signature_b64) =
            token.split_once('.').ok_or(GitHubError::InvalidState)?;
        let signature = URL_SAFE_NO_PAD
            .decode(signature_b64)
            .map_err(|_| GitHubError::InvalidState)?;

        let mut mac = Hmac::<Sha256>::new_from_slice(self.gitdot_github_secret.as_bytes())
            .map_err(|_| GitHubError::InvalidState)?;
        mac.update(payload_b64.as_bytes());
        mac.verify_slice(&signature)
            .map_err(|_| GitHubError::InvalidState)?;

        let payload_json = URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|_| GitHubError::InvalidState)?;
        let payload: InstallStatePayload =
            serde_json::from_slice(&payload_json).map_err(|_| GitHubError::InvalidState)?;

        if payload.exp < Utc::now().timestamp() {
            return Err(GitHubError::InvalidState);
        }
        Ok(payload)
    }

    async fn get_installation(&self, installation_id: u64) -> Result<Installation, GitHubError> {
        let installation = self
            .app_client
            .apps()
            .installation(InstallationId(installation_id))
            .await?;

        Ok(installation)
    }

    async fn get_installation_access_token(
        &self,
        installation_id: u64,
    ) -> Result<String, GitHubError> {
        // TODO: cache the token (valid for an hour)
        let (_, token) = self
            .app_client
            .installation_and_token(InstallationId(installation_id))
            .await?;

        Ok(token.expose_secret().to_string())
    }

    async fn list_installation_repositories(
        &self,
        installation_id: u64,
    ) -> Result<InstallationRepositories, GitHubError> {
        let client = self
            .app_client
            .installation(InstallationId(installation_id))?;

        // TODO: implement fetch all pages
        let repositories = client
            .get("/installation/repositories", Some(&[("per_page", 100)]))
            .await?;

        Ok(repositories)
    }

    fn verify_webhook_signature(
        &self,
        body: &[u8],
        signature_header: &str,
    ) -> Result<(), GitHubError> {
        let hex_sig = signature_header
            .strip_prefix("sha256=")
            .ok_or(GitHubError::InvalidSignature)?;
        let sig_bytes = hex::decode(hex_sig).map_err(|_| GitHubError::InvalidSignature)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(self.gitdot_github_secret.as_bytes())
            .map_err(|_| GitHubError::InvalidSignature)?;
        mac.update(body);
        mac.verify_slice(&sig_bytes)
            .map_err(|_| GitHubError::InvalidSignature)?;
        Ok(())
    }
}
