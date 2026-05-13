use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{
        EmailClient, GitHubClient, ImageClient, ImageClientImpl, OctocrabClient, R2Client,
        R2ClientImpl, ResendClient, TokenClient, TokenClientImpl,
    },
    dto::{
        AuthTokensResponse, ExchangeGitHubCodeRequest, LogoutRequest, OAuthRedirectResponse,
        RefreshSessionRequest, SendAuthEmailRequest, VerifyAuthCodeRequest,
    },
    error::{AuthenticationError, OptionNotFoundExt},
    model::AuthProvider,
    repository::{SessionRepository, SessionRepositoryImpl, UserRepository, UserRepositoryImpl},
    util::{
        auth::{NOREPLY_EMAIL, get_auth_email},
        crypto::hash_string,
    },
};

#[async_trait]
pub trait SessionService: Send + Sync + 'static {
    async fn send_auth_email(
        &self,
        request: SendAuthEmailRequest,
    ) -> Result<(), AuthenticationError>;

    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;

    async fn refresh_session(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;

    async fn logout(&self, request: LogoutRequest) -> Result<(), AuthenticationError>;

    fn get_github_authorization_url(&self) -> OAuthRedirectResponse;

    async fn exchange_github_code(
        &self,
        request: ExchangeGitHubCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;
}

#[derive(Debug, Clone)]
pub struct SessionServiceImpl<SR, UR, EC, GH, TC, IC, RC>
where
    SR: SessionRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
{
    session_repo: SR,
    user_repo: UR,
    email_client: EC,
    github_client: GH,
    token_client: TC,
    image_client: IC,
    r2_client: RC,
}

impl
    SessionServiceImpl<
        SessionRepositoryImpl,
        UserRepositoryImpl,
        ResendClient,
        OctocrabClient,
        TokenClientImpl,
        ImageClientImpl,
        R2ClientImpl,
    >
{
    pub fn new(
        session_repo: SessionRepositoryImpl,
        user_repo: UserRepositoryImpl,
        email_client: ResendClient,
        github_client: OctocrabClient,
        token_client: TokenClientImpl,
        image_client: ImageClientImpl,
        r2_client: R2ClientImpl,
    ) -> Self {
        Self {
            session_repo,
            user_repo,
            email_client,
            github_client,
            token_client,
            image_client,
            r2_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<SR, UR, EC, GH, TC, IC, RC> SessionService for SessionServiceImpl<SR, UR, EC, GH, TC, IC, RC>
where
    SR: SessionRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
{
    async fn send_auth_email(
        &self,
        request: SendAuthEmailRequest,
    ) -> Result<(), AuthenticationError> {
        let email = request.email.as_ref().to_string();
        let user = match self.user_repo.get_by_email(&email).await? {
            Some(user) => user,
            None => {
                let user = self
                    .user_repo
                    .create(&email, false, AuthProvider::Email)
                    .await?;

                if let Ok(image_bytes) = self.image_client.generate_user_image(&email).await {
                    self.r2_client
                        .upload_object(&format!("users/{}.webp", user.id), image_bytes)
                        .await
                        .ok();
                }
                user
            }
        };

        let code = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_auth_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);
        self.session_repo
            .create_auth_code(user.id, &code, expires_at)
            .await?;

        let (subject, html) = get_auth_email(&code);
        self.email_client
            .send_email(NOREPLY_EMAIL, &email, &subject, &html)
            .await?;

        Ok(())
    }

    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        let auth_code = self
            .session_repo
            .get_auth_code(&request.code)
            .await?
            .or_not_found("auth_code", &request.code)?;

        if auth_code.used_at.is_some() {
            return Err(AuthenticationError::TokenRevoked("auth_code".into()));
        }
        if auth_code.expires_at < Utc::now() {
            return Err(AuthenticationError::TokenExpired("auth_code".into()));
        }

        self.session_repo.mark_auth_code_used(auth_code.id).await?;

        let user = self
            .user_repo
            .get_by_id(auth_code.user_id)
            .await?
            .or_not_found("user", auth_code.user_id)?;
        let is_new = !user.is_email_verified;

        self.user_repo.verify_email(auth_code.user_id).await?;
        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                auth_code.user_id,
                &refresh_token_hash,
                uuid::Uuid::new_v4(),
                request.user_agent.as_deref(),
                request.ip_address,
                refresh_expiry,
            )
            .await?;

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new,
        })
    }

    async fn refresh_session(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        let token_hash = hash_string(&request.refresh_token);
        let session = self
            .session_repo
            .get_session_by_refresh_hash(&token_hash)
            .await?
            .or_not_found("session", &token_hash)?;

        if session.revoked_at.is_some() {
            self.session_repo
                .revoke_sessions_by_family(session.refresh_token_family)
                .await?;
            return Err(AuthenticationError::TokenRevoked("session".into()));
        }
        if session.expires_at < Utc::now() {
            return Err(AuthenticationError::TokenExpired("session".into()));
        }

        self.session_repo.revoke_session(session.id).await?;

        let user = self
            .user_repo
            .get_by_id(session.user_id)
            .await?
            .or_not_found("user", session.user_id)?;
        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                session.user_id,
                &refresh_token_hash,
                session.refresh_token_family,
                request.user_agent.as_deref(),
                request.ip_address,
                refresh_expiry,
            )
            .await?;

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new: false,
        })
    }

    async fn logout(&self, request: LogoutRequest) -> Result<(), AuthenticationError> {
        let token_hash = hash_string(&request.refresh_token);
        let session = self
            .session_repo
            .get_session_by_refresh_hash(&token_hash)
            .await?
            .or_not_found("session", &token_hash)?;

        self.session_repo.revoke_session(session.id).await?;

        Ok(())
    }

    fn get_github_authorization_url(&self) -> OAuthRedirectResponse {
        let state = self.token_client.generate_oauth_state();
        let authorize_url = self.github_client.get_authorization_url(&state);
        OAuthRedirectResponse {
            authorize_url,
            state,
        }
    }

    async fn exchange_github_code(
        &self,
        request: ExchangeGitHubCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        self.token_client
            .verify_oauth_state(&request.state)
            .map_err(|_| AuthenticationError::Unauthorized)?;

        let github_token = self.github_client.exchange_code(&request.code).await?;
        let email = self.github_client.get_user_email(&github_token).await?;
        let (user, is_new) = match self.user_repo.get_by_email(&email).await? {
            Some(user) => (user, false),
            None => {
                let user = self
                    .user_repo
                    .create(&email, true, AuthProvider::GitHub)
                    .await?;
                (user, true)
            }
        };

        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                user.id,
                &refresh_token_hash,
                uuid::Uuid::new_v4(),
                request.user_agent.as_deref(),
                request.ip_address,
                refresh_expiry,
            )
            .await?;

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new,
        })
    }
}
