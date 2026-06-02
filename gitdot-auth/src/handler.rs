mod account;
mod device;
mod email;
mod github;
mod logout;
mod refresh_session;
mod slack;

use std::time::Duration;

use axum::{
    Router, middleware,
    routing::{delete, get, post},
};

use gitdot_axum::middleware::{create_rate_limiter, verify_vercel_oidc};

use crate::app::AppState;

use account::{add_user_email, delete_account, verify_user_email};
use device::{authorize_device, create_device_code, poll_token};
use email::{send_auth_email, verify_auth_code};
use github::{exchange_github_code, redirect_to_github_auth};
use logout::logout;
use refresh_session::refresh_session;
use slack::link_slack_account;

const DEVICE_POLL_PERIOD: Duration = Duration::from_millis(500); // ~2 req/s, covers the 1 req/s poll loop
const DEVICE_POLL_BURST: u32 = 10;

const SENSITIVE_PERIOD: Duration = Duration::from_millis(333); // ~3 req/s
const SENSITIVE_BURST: u32 = 15;

const DEFAULT_PERIOD: Duration = Duration::from_millis(66); // ~15 req/s
const DEFAULT_BURST: u32 = 60;

pub fn create_auth_router(state: AppState) -> Router<AppState> {
    let cli_routes = Router::new()
        .merge(
            Router::new()
                .route("/auth/device/token", post(poll_token))
                .layer(create_rate_limiter(DEVICE_POLL_PERIOD, DEVICE_POLL_BURST)),
        )
        .merge(
            Router::new()
                .route("/auth/device/code", post(create_device_code))
                .layer(create_rate_limiter(DEFAULT_PERIOD, DEFAULT_BURST)),
        );

    // routes that either send an email or authenticate using 6 character OTP codes
    // use strict rate limiting to prevent brute-force attacks
    let web_sensitive = Router::new()
        .route("/auth/device/authorize", post(authorize_device))
        .route("/auth/account", delete(delete_account))
        .route("/auth/account/add-email", post(add_user_email))
        .route("/auth/account/verify-email", post(verify_user_email))
        .route("/auth/email/send", post(send_auth_email))
        .route("/auth/email/verify", post(verify_auth_code))
        .layer(create_rate_limiter(SENSITIVE_PERIOD, SENSITIVE_BURST));

    let web_session = Router::new()
        .route("/auth/github/redirect", get(redirect_to_github_auth))
        .route("/auth/github/exchange", post(exchange_github_code))
        .route("/auth/slack/link", post(link_slack_account))
        .route("/auth/refresh", post(refresh_session))
        .route("/auth/logout", post(logout))
        .layer(create_rate_limiter(DEFAULT_PERIOD, DEFAULT_BURST));

    let web_routes = Router::new()
        .merge(web_sensitive)
        .merge(web_session)
        .route_layer(middleware::from_fn_with_state(
            state.vercel_oidc_config,
            verify_vercel_oidc,
        ));

    Router::new().merge(cli_routes).merge(web_routes)
}
