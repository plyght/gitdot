mod create_webhook;
mod delete_webhook;
mod get_webhook;
mod list_webhooks;
mod slack;
mod update_webhook;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_webhook::create_webhook;
use delete_webhook::delete_webhook;
use get_webhook::get_webhook;
use list_webhooks::list_webhooks;
use slack::{subscribe::subscribe_slack_webhook, unsubscribe::unsubscribe_slack_webhook};
use update_webhook::update_webhook;

pub fn create_webhook_router() -> Router<AppState> {
    Router::new()
        .route("/repository/{owner}/{repo}/webhook", post(create_webhook))
        .route("/repository/{owner}/{repo}/webhooks", get(list_webhooks))
        .route(
            "/repository/{owner}/{repo}/webhook/{webhook_id}",
            get(get_webhook)
                .patch(update_webhook)
                .delete(delete_webhook),
        )
        // Internal routes intended for communication with Slack bot server
        .route(
            "/repository/{owner}/{repo}/webhook/slack",
            post(subscribe_slack_webhook),
        )
        .route(
            "/repository/{owner}/{repo}/webhook/slack/{webhook_id}",
            post(unsubscribe_slack_webhook),
        )
}
