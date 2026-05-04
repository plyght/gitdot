use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_api::resource::webhook as api;
use gitdot_core::dto::WebhookResponse;

use super::IntoApi;

impl IntoApi for WebhookResponse {
    type ApiType = api::WebhookResource;
    fn into_api(self) -> Self::ApiType {
        api::WebhookResource {
            id: self.id,
            repository_id: self.repository_id,
            url: self.url,
            secret: self.secret,
            events: self.events.into_iter().map(Into::into).collect(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeSlackWebhookRequest {
    pub gitdot_user_id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeSlackWebhookRequest {
    pub gitdot_user_id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
}
