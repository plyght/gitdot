use async_trait::async_trait;
use chrono::Utc;

use crate::{
    client::{Git2Client, GitClient, KafkaClient, KafkaClientImpl},
    dto::{PublishRepoPushRequest, RepoPushCommit, RepoPushEvent},
    error::{OptionNotFoundExt, WebhookError},
    repository::{UserRepository, UserRepositoryImpl},
};

/// Publishes domain events for repository activity onto Kafka so downstream
/// consumers (webhook delivery, notifications) can react asynchronously.
#[async_trait]
pub trait EventService: Send + Sync + 'static {
    /// Builds a [`RepoPushEvent`] for a push and publishes it to Kafka.
    ///
    /// Looks up the pusher to attach their display name, then runs `rev_list`
    /// between `old_sha` and `new_sha` to collect the commits introduced by the
    /// push (sha + message). The event is stamped with the current time and
    /// emitted via the Kafka client; no webhooks are delivered directly here.
    ///
    /// # Errors
    /// - [`WebhookError::NotFound`] if no user matches `pusher_id`.
    /// - [`WebhookError::GitError`] if listing the pushed commits fails.
    /// - [`WebhookError::KafkaError`] if publishing the event fails.
    async fn publish_repo_push(&self, request: PublishRepoPushRequest) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone)]
pub struct EventServiceImpl<U, G, K>
where
    U: UserRepository,
    G: GitClient,
    K: KafkaClient,
{
    user_repo: U,
    git_client: G,
    kafka_client: K,
}

impl EventServiceImpl<UserRepositoryImpl, Git2Client, KafkaClientImpl> {
    pub fn new(
        user_repo: UserRepositoryImpl,
        git_client: Git2Client,
        kafka_client: KafkaClientImpl,
    ) -> Self {
        Self {
            user_repo,
            git_client,
            kafka_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<U, G, K> EventService for EventServiceImpl<U, G, K>
where
    U: UserRepository,
    G: GitClient,
    K: KafkaClient,
{
    async fn publish_repo_push(&self, request: PublishRepoPushRequest) -> Result<(), WebhookError> {
        let pusher = self
            .user_repo
            .get_by_id(request.pusher_id)
            .await?
            .or_not_found("user", request.pusher_id)?;

        let git_commits = self
            .git_client
            .rev_list(
                &request.owner,
                &request.repo,
                &request.old_sha,
                &request.new_sha,
            )
            .await?;

        let commits = git_commits
            .into_iter()
            .map(|c| RepoPushCommit {
                sha: c.sha,
                message: c.message,
            })
            .collect();

        let event = RepoPushEvent {
            owner: request.owner.into_inner(),
            repo: request.repo.into_inner(),
            ref_name: request.ref_name,
            old_sha: request.old_sha,
            new_sha: request.new_sha,
            pusher_id: request.pusher_id,
            pusher_name: pusher.name,
            commits,
            pushed_at: Utc::now(),
        };

        self.kafka_client.publish_repo_push(event).await?;

        Ok(())
    }
}
