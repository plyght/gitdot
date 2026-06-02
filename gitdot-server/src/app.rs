mod bootstrap;
mod error;
mod response;
mod settings;
mod state;

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{Router, middleware::from_fn, routing::get};
use http::StatusCode;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tokio::net;
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use gitdot_axum::middleware::{create_rate_limiter, log_request};

use crate::handler::{
    create_git_http_router, create_internal_router, create_migration_router,
    create_organization_router, create_repository_router, create_user_router,
};

pub use error::AppError;
pub use response::AppResponse;
pub use settings::Settings;
pub use state::AppState;

const API_RATE_LIMIT_PERIOD: Duration = Duration::from_millis(10);
const API_RATE_LIMIT_BURST: u32 = 200;

pub struct GitdotServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl GitdotServer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Arc::new(Settings::new()?);
        let pool = PgPool::connect(settings.database_url.expose_secret()).await?;
        let state = AppState::new(settings.clone(), pool).await?;
        let router = create_router(state);
        let listener = tokio::net::TcpListener::bind(&settings.get_server_address())
            .await
            .unwrap();

        Ok(Self { router, listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!("Starting server on {}", self.listener.local_addr().unwrap());
        axum::serve(
            self.listener,
            self.router
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .context("Failed to start server")?;
        Ok(())
    }
}

fn create_router(app_state: AppState) -> Router {
    let web_origin = app_state
        .settings
        .gitdot_web_url
        .parse()
        .expect("GITDOT_WEB_URL must be a valid origin");

    let api_middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(log_request))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list([web_origin]))
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::PATCH,
                    http::Method::DELETE,
                    http::Method::HEAD,
                ])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                .allow_credentials(true),
        )
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(create_rate_limiter(
            API_RATE_LIMIT_PERIOD,
            API_RATE_LIMIT_BURST,
        ))
        .layer(PropagateRequestIdLayer::x_request_id());

    let api_router = Router::new()
        .merge(create_user_router())
        .merge(create_organization_router())
        .merge(create_repository_router())
        .merge(create_migration_router())
        .layer(api_middleware);

    let git_router = Router::new().merge(create_git_http_router());
    let internal_router = Router::new().merge(create_internal_router());

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(api_router)
        .merge(git_router)
        .merge(internal_router)
        .with_state(app_state)
}
