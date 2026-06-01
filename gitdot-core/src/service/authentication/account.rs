use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{EmailClient, RedisClient, RedisClientImpl, SmtpClient, TokenClient, TokenClientImpl},
    dto::{AddUserEmailRequest, UserEmailResponse, VerifyUserEmailRequest},
    error::{AccountError, ConflictError},
    repository::{
        EmailCodeVerification, EmailVerificationRepository, PgEmailVerificationRepository,
        PgUserRepository, UserRepository,
    },
    util::{
        auth::{NOREPLY_EMAIL, get_code_email},
        crypto::hash_string,
    },
};

/// wrong-guess budget for an email verification code before it is burned
const MAX_EMAIL_CODE_ATTEMPTS: i16 = 5;

/// minimum delay between verification-code sends for the same `(user, email)`
const EMAIL_VERIFY_SEND_COOLDOWN: std::time::Duration = std::time::Duration::from_secs(30);

/// Manages a user's account emails: adding secondary emails, issuing
/// emailed verification codes, and verifying them. Each code is single-use,
/// time-limited, and scoped to the requesting user.
#[async_trait]
pub trait AccountService: Send + Sync + 'static {
    /// Emails a verification code for the address without persisting anything to
    /// `core.user_emails` — the row is only created once the code is verified.
    /// Any prior active codes for the address are invalidated, so calling this
    /// again simply re-issues a fresh code. Fails with `Conflict` if the email
    /// is already verified by any user.
    async fn add_email(&self, request: AddUserEmailRequest) -> Result<(), AccountError>;

    /// Verifies a code previously emailed for `request.email`. On success the
    /// verified `user_emails` row is created and the code is marked used. Scoped
    /// to the caller — a code only verifies the address for `request.user_id`.
    async fn verify_email(
        &self,
        request: VerifyUserEmailRequest,
    ) -> Result<UserEmailResponse, AccountError>;
}

#[derive(Debug, Clone)]
pub struct AccountServiceImpl<UR, ER, EC, TC, RD>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
    RD: RedisClient,
{
    user_repo: UR,
    email_verification_repo: ER,
    email_client: EC,
    token_client: TC,
    redis_client: RD,
}

impl
    AccountServiceImpl<
        PgUserRepository,
        PgEmailVerificationRepository,
        SmtpClient,
        TokenClientImpl,
        RedisClientImpl,
    >
{
    pub fn new(
        user_repo: PgUserRepository,
        email_verification_repo: PgEmailVerificationRepository,
        email_client: SmtpClient,
        token_client: TokenClientImpl,
        redis_client: RedisClientImpl,
    ) -> Self {
        Self {
            user_repo,
            email_verification_repo,
            email_client,
            token_client,
            redis_client,
        }
    }
}

impl<UR, ER, EC, TC, RD> AccountServiceImpl<UR, ER, EC, TC, RD>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
    RD: RedisClient,
{
    fn get_rate_limit_key(&self, user_id: uuid::Uuid, email: &str) -> String {
        format!(
            "email_verify_send:{}",
            hash_string(&format!("{user_id}:{email}"))
        )
    }

    async fn issue_and_send_code(
        &self,
        user_id: uuid::Uuid,
        email: &str,
    ) -> Result<(), AccountError> {
        // rate-limit code sends per (user, email)
        let rate_limit_key = self.get_rate_limit_key(user_id, email);
        match self
            .redis_client
            .set_nx_with_ttl(&rate_limit_key, &true, EMAIL_VERIFY_SEND_COOLDOWN)
            .await
        {
            Ok(true) => {}
            Ok(false) => return Err(AccountError::TooManyAttempts),
            Err(e) => {
                tracing::warn!(error = %e, "email code send rate-limit check failed; allowing send")
            }
        }

        self.email_verification_repo
            .invalidate_codes_for_email(user_id, email)
            .await?;

        let (code, code_hash) = self.token_client.generate_readable_code();
        let expires_at = Utc::now()
            + Duration::seconds(self.token_client.get_auth_code_expiry_in_seconds() as i64);
        self.email_verification_repo
            .create_code(user_id, email, &code_hash, expires_at)
            .await?;

        let (subject, html) = get_code_email(&code);
        self.email_client
            .send_email(NOREPLY_EMAIL, email, &subject, &html)
            .await?;

        Ok(())
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<UR, ER, EC, TC, RD> AccountService for AccountServiceImpl<UR, ER, EC, TC, RD>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
    RD: RedisClient,
{
    async fn add_email(&self, request: AddUserEmailRequest) -> Result<(), AccountError> {
        let email = request.email.as_ref();

        if self.user_repo.is_email_taken(email).await? {
            return Err(ConflictError::new("email", email).into());
        }

        self.issue_and_send_code(request.user_id, email).await?;

        Ok(())
    }

    async fn verify_email(
        &self,
        request: VerifyUserEmailRequest,
    ) -> Result<UserEmailResponse, AccountError> {
        let email = request.email.as_ref();
        let code_hash = hash_string(request.code.as_ref());

        let outcome = match self
            .email_verification_repo
            .verify_and_consume_email_code(
                request.user_id,
                email,
                &code_hash,
                MAX_EMAIL_CODE_ATTEMPTS,
            )
            .await
        {
            Ok(outcome) => outcome,
            Err(e) if e.is_unique_violation() => {
                return Err(ConflictError::new("email", email).into());
            }
            Err(e) => return Err(e.into()),
        };

        match outcome {
            EmailCodeVerification::Success(row) => Ok(row.into()),
            EmailCodeVerification::AttemptsExhausted => Err(AccountError::TooManyAttempts),
            EmailCodeVerification::Invalid | EmailCodeVerification::NoActiveCode => {
                Err(AccountError::InvalidCode)
            }
        }
    }
}
