use async_trait::async_trait;
use chrono::Utc;

use crate::{
    client::{
        GitHubClient, OctocrabClient, SlackBotClient, SlackBotClientImpl, TokenClient,
        TokenClientImpl,
    },
    dto::{
        IssueTaskJwtRequest, IssueTaskJwtResponse, JwtClaims, ValidateTokenRequest,
        ValidateTokenResponse, VerifyGithubSignatureRequest, VerifySlackBotSignatureRequest,
    },
    error::AuthenticationError,
    repository::{TokenRepository, TokenRepositoryImpl},
    util::{
        auth::{GITDOT_SERVER_ID, S2_SERVER_ID},
        crypto::hash_string,
    },
};

#[async_trait]
pub trait TokenService: Send + Sync + 'static {
    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthenticationError>;

    async fn issue_task_token(
        &self,
        request: IssueTaskJwtRequest,
    ) -> Result<IssueTaskJwtResponse, AuthenticationError>;

    fn verify_github_signature(
        &self,
        request: VerifyGithubSignatureRequest,
    ) -> Result<(), AuthenticationError>;

    fn verify_slack_bot_signature(
        &self,
        request: VerifySlackBotSignatureRequest,
    ) -> Result<(), AuthenticationError>;
}

#[derive(Debug, Clone)]
pub struct TokenServiceImpl<TR, GH, SBC, TC>
where
    TR: TokenRepository,
    GH: GitHubClient,
    SBC: SlackBotClient,
    TC: TokenClient,
{
    token_repo: TR,
    github_client: GH,
    slack_bot_client: SBC,
    token_client: TC,
}

impl TokenServiceImpl<TokenRepositoryImpl, OctocrabClient, SlackBotClientImpl, TokenClientImpl> {
    pub fn new(
        token_repo: TokenRepositoryImpl,
        github_client: OctocrabClient,
        slack_bot_client: SlackBotClientImpl,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            token_repo,
            github_client,
            slack_bot_client,
            token_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<TR, GH, SBC, TC> TokenService for TokenServiceImpl<TR, GH, SBC, TC>
where
    TR: TokenRepository,
    GH: GitHubClient,
    SBC: SlackBotClient,
    TC: TokenClient,
{
    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthenticationError> {
        if !self.token_client.validate_token_format(&request.token) {
            return Err(AuthenticationError::Unauthorized);
        }
        if !&request.token.starts_with(request.token_type.prefix()) {
            return Err(AuthenticationError::Unauthorized);
        }

        let token_hash = hash_string(&request.token);
        let access_token = self
            .token_repo
            .get_token_by_hash(&token_hash)
            .await?
            .ok_or(AuthenticationError::Unauthorized)?;

        self.token_repo.touch_token(access_token.id).await?;

        Ok(ValidateTokenResponse {
            principal_id: access_token.principal_id,
        })
    }

    async fn issue_task_token(
        &self,
        request: IssueTaskJwtRequest,
    ) -> Result<IssueTaskJwtResponse, AuthenticationError> {
        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            iss: GITDOT_SERVER_ID.to_string(),
            aud: vec![GITDOT_SERVER_ID.to_string(), S2_SERVER_ID.to_string()],
            sub: request.task_id.to_string(),
            iat: now,
            exp: now + request.duration.as_secs() as usize,
        };
        let token = self.token_client.generate_jwt(&claims)?;

        Ok(IssueTaskJwtResponse { token })
    }

    fn verify_github_signature(
        &self,
        request: VerifyGithubSignatureRequest,
    ) -> Result<(), AuthenticationError> {
        self.github_client
            .verify_webhook_signature(&request.body, &request.signature)
            .map_err(|_| AuthenticationError::Unauthorized)
    }

    fn verify_slack_bot_signature(
        &self,
        request: VerifySlackBotSignatureRequest,
    ) -> Result<(), AuthenticationError> {
        self.slack_bot_client
            .verify_request_signature(&request.timestamp, &request.body, &request.signature)
            .map_err(|_| AuthenticationError::Unauthorized)
    }
}
