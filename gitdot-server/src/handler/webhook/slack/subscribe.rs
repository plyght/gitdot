use axum::extract::{Path, State};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::SubscribeSlackWebhookRequest,
    extract::SlackBotSigned,
};

#[axum::debug_handler]
pub async fn subscribe_slack_webhook(
    State(_state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    SlackBotSigned(_body): SlackBotSigned<SubscribeSlackWebhookRequest>,
) -> Result<AppResponse<()>, AppError> {
    todo!()
}
