use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::UnsubscribeSlackWebhookRequest,
    extract::SlackBotSigned,
};

#[axum::debug_handler]
pub async fn unsubscribe_slack_webhook(
    State(_state): State<AppState>,
    Path((_owner, _repo, _webhook_id)): Path<(String, String, Uuid)>,
    SlackBotSigned(_body): SlackBotSigned<UnsubscribeSlackWebhookRequest>,
) -> Result<AppResponse<()>, AppError> {
    todo!()
}
