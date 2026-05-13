use async_trait::async_trait;

use crate::{
    client::{SlackBotClient, SlackBotClientImpl},
    dto::{LinkSlackAccountRequest, LinkSlackAccountResponse},
    error::AuthenticationError,
    repository::{SlackRepository, SlackRepositoryImpl},
};

#[async_trait]
pub trait SlackService: Send + Sync + 'static {
    async fn link_slack_account(
        &self,
        request: LinkSlackAccountRequest,
    ) -> Result<LinkSlackAccountResponse, AuthenticationError>;
}

#[derive(Debug, Clone)]
pub struct SlackServiceImpl<SlR, SBC>
where
    SlR: SlackRepository,
    SBC: SlackBotClient,
{
    slack_repo: SlR,
    slack_bot_client: SBC,
}

impl SlackServiceImpl<SlackRepositoryImpl, SlackBotClientImpl> {
    pub fn new(slack_repo: SlackRepositoryImpl, slack_bot_client: SlackBotClientImpl) -> Self {
        Self {
            slack_repo,
            slack_bot_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<SlR, SBC> SlackService for SlackServiceImpl<SlR, SBC>
where
    SlR: SlackRepository,
    SBC: SlackBotClient,
{
    async fn link_slack_account(
        &self,
        request: LinkSlackAccountRequest,
    ) -> Result<LinkSlackAccountResponse, AuthenticationError> {
        let payload = self
            .slack_bot_client
            .verify_slack_state(&request.state)
            .map_err(|_| AuthenticationError::Unauthorized)?;

        let account = self
            .slack_repo
            .create_slack_account(
                request.gitdot_user_id,
                &payload.slack_user_id,
                &payload.slack_team_id,
            )
            .await?;

        self.slack_bot_client
            .notify_link_completed(request.gitdot_user_id, &payload.slack_channel_id)
            .await?;

        Ok(account.into())
    }
}
