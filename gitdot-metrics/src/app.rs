mod bootstrap;
mod error;
mod settings;
mod state;

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{Router, middleware::from_fn, routing::get};
use http::StatusCode;
use tokio::net;
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use gitdot_axum::middleware::{create_rate_limiter, log_request};

use crate::handler::create_metrics_router;

pub use error::AppError;
pub use settings::Settings;
pub use state::AppState;

const METRICS_RATE_LIMIT_PERIOD: Duration = Duration::from_millis(50); // ≈20 req/s
const METRICS_RATE_LIMIT_BURST: u32 = 100;

pub struct GitdotMetricsServer {
    router: Router,
    listener: net::TcpListener,
}

impl GitdotMetricsServer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Arc::new(Settings::new()?);
        let state = AppState::new(settings.clone()).await?;
        let router = create_router(state);
        let listener = net::TcpListener::bind(&settings.get_server_address()).await?;

        Ok(Self { router, listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!(
            "Starting metrics server on {}",
            self.listener.local_addr().unwrap()
        );
        axum::serve(
            self.listener,
            self.router
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .context("Failed to start metrics server")?;
        Ok(())
    }
}

fn create_router(state: AppState) -> Router {
    let web_origin = state
        .settings
        .gitdot_web_url
        .parse()
        .expect("GITDOT_WEB_URL must be a valid origin");

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(log_request))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list([web_origin]))
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(create_rate_limiter(
            METRICS_RATE_LIMIT_PERIOD,
            METRICS_RATE_LIMIT_BURST,
        ))
        .layer(PropagateRequestIdLayer::x_request_id());

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(create_metrics_router(state.clone()))
        .layer(middleware)
        .with_state(state)
}
