use async_trait::async_trait;

use crate::{
    client::{SlackBotClient, SlackBotClientImpl},
    dto::{LinkSlackAccountRequest, LinkSlackAccountResponse},
    error::SlackError,
    repository::{PgSlackRepository, SlackRepository},
};

/// Links a Slack identity to a Gitdot user, completing the Slack app's
/// account-connection flow.
#[async_trait]
pub trait SlackService: Send + Sync + 'static {
    /// Links a Slack account to a Gitdot user.
    ///
    /// Verifies the signed `state` from the Slack flow to recover the Slack
    /// user, team, and channel, persists the link, and notifies the user in
    /// Slack that linking completed.
    ///
    /// # Errors
    /// - [`SlackError::Unauthorized`] — `state` failed verification
    async fn link_slack_account(
        &self,
        request: LinkSlackAccountRequest,
    ) -> Result<LinkSlackAccountResponse, SlackError>;
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

impl SlackServiceImpl<PgSlackRepository, SlackBotClientImpl> {
    pub fn new(slack_repo: PgSlackRepository, slack_bot_client: SlackBotClientImpl) -> Self {
        Self {
            slack_repo,
            slack_bot_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<SlR, SBC> SlackService for SlackServiceImpl<SlR, SBC>
where
    SlR: SlackRepository,
    SBC: SlackBotClient,
{
    async fn link_slack_account(
        &self,
        request: LinkSlackAccountRequest,
    ) -> Result<LinkSlackAccountResponse, SlackError> {
        let payload = self
            .slack_bot_client
            .verify_slack_state(&request.state)
            .map_err(|_| SlackError::Unauthorized)?;

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
