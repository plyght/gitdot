use std::time::Instant;

use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};

/// Emits one structured log event per request under target
/// `gitdot_server::request`.
///
/// Fields: `method`, `route` (matched pattern, not rendered URL),
/// `status`, `status_class`, `duration_ms`.
///
/// Designed to feed Cloud Logging log-based metrics for the
/// per-endpoint observability dashboard. Human-readable per-request
/// logging is covered by `tower_http::TraceLayer` at `tower_http=debug`.
pub async fn log_request(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_owned())
        .unwrap_or_else(|| "<unmatched>".to_string());
    let start = Instant::now();

    let response = next.run(request).await;

    let status = response.status().as_u16();
    let status_class = match status {
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "other",
    };
    let duration_ms = start.elapsed().as_millis() as u64;

    tracing::info!(
        target: "gitdot_server::request",
        method = %method,
        route = %route,
        status = status,
        status_class = status_class,
        duration_ms = duration_ms,
        "{method} {route} → {status} ({duration_ms}ms)"
    );

    response
}
