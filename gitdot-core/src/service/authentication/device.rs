use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{TokenClient, TokenClientImpl},
    dto::{
        AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest,
        TokenResponse,
    },
    error::{DeviceError, InputError, OptionNotFoundExt},
    model::{DeviceAuthorizationStatus, TokenType},
    repository::{
        DeviceRepository, DeviceRepositoryImpl, TokenRepository, TokenRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
    util::crypto::hash_string,
};

#[async_trait]
pub trait DeviceService: Send + Sync + 'static {
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, DeviceError>;

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, DeviceError>;

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), DeviceError>;
}

#[derive(Debug, Clone)]
pub struct DeviceServiceImpl<DR, TR, UR, TC>
where
    DR: DeviceRepository,
    TR: TokenRepository,
    UR: UserRepository,
    TC: TokenClient,
{
    device_repo: DR,
    token_repo: TR,
    user_repo: UR,
    token_client: TC,
}

impl
    DeviceServiceImpl<
        DeviceRepositoryImpl,
        TokenRepositoryImpl,
        UserRepositoryImpl,
        TokenClientImpl,
    >
{
    pub fn new(
        device_repo: DeviceRepositoryImpl,
        token_repo: TokenRepositoryImpl,
        user_repo: UserRepositoryImpl,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            device_repo,
            token_repo,
            user_repo,
            token_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<DR, TR, UR, TC> DeviceService for DeviceServiceImpl<DR, TR, UR, TC>
where
    DR: DeviceRepository,
    TR: TokenRepository,
    UR: UserRepository,
    TC: TokenClient,
{
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, DeviceError> {
        let (device_code, device_code_hash) = self.token_client.generate_high_entropic_code();
        let user_code = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_device_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);

        self.device_repo
            .create_device_authorization(
                &device_code_hash,
                &user_code,
                &request.client_id,
                expires_at,
            )
            .await?;

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_url: request.verification_url,
            expires_in: expiry_secs,
            interval: self.token_client.get_polling_interval_in_seconds(),
        })
    }

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, DeviceError> {
        let device_code_hash = hash_string(&request.device_code);
        let device_auth = self
            .device_repo
            .get_device_authorization_by_device_code_hash(&device_code_hash)
            .await?
            .or_not_found("device_code", &device_code_hash)?;

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.device_repo
                .expire_device_authorization(device_auth.id)
                .await?;
            return Err(DeviceError::TokenExpired("device_code".into()));
        }

        match device_auth.status {
            DeviceAuthorizationStatus::Pending => {
                Err(DeviceError::TokenPending("device_code".into()))
            }
            DeviceAuthorizationStatus::Expired => {
                Err(DeviceError::TokenExpired("device_code".into()))
            }
            DeviceAuthorizationStatus::Authorized => {
                let user_id = device_auth
                    .user_id
                    .ok_or(InputError::new("user_id", "missing"))?;

                let user = self
                    .user_repo
                    .get_by_id(user_id)
                    .await?
                    .or_not_found("user", user_id)?;

                let (access_token, token_hash) = self
                    .token_client
                    .generate_access_token(&TokenType::Personal);

                self.token_repo
                    .create_token(
                        user_id,
                        &device_auth.client_id,
                        &token_hash,
                        TokenType::Personal,
                    )
                    .await?;

                self.device_repo
                    .expire_device_authorization(device_auth.id)
                    .await?;

                Ok(TokenResponse {
                    access_token,
                    user_name: user.name,
                    user_email: user.email,
                })
            }
        }
    }

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), DeviceError> {
        self.device_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?
            .or_not_found("user_code", request.user_code.as_ref())?;

        Ok(())
    }
}
