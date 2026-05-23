# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot-metrics` is the standalone Axum HTTP server for product telemetry ingestion. Today it accepts batched web-vital events from the frontend and writes them to ClickHouse via `MetricsService` (defined in `gitdot-core`). Like the other binary crates, it's a thin transport layer — no business logic lives here.

## Structure

- `app.rs` — Router construction, middleware stack, `GitdotMetricsServer` entrypoint
- `app/settings.rs` — `Settings` loaded via figment from env vars (ClickHouse credentials, OIDC URL, gitdot public key)
- `app/state.rs` — `AppState` with `MetricsService` and `Arc<Settings>`; exposes `AuthConfig` / `VercelOidcConfig` via `FromRef`
- `app/bootstrap.rs` — thin startup orchestrator; delegates to `gitdot_axum::bootstrap` (env, crypto provider, tracing)
- `app/error.rs` — `AppError` enum mapping core errors to HTTP status codes
- `handler/metrics/web_vital.rs` — `POST /metrics/web-vital` handler
- `bin/main.rs` — Entry point: `GitdotMetricsServer::new().await?.start().await`

## Configuration

All settings — including ClickHouse credentials — are loaded from environment variables at startup via figment (`Figment::new().merge(Env::raw()).extract()`). In production these are injected via Cloud Run's Secret Manager bindings; locally they come from `.env`.

- Required: `GITDOT_PUBLIC_KEY`, `VERCEL_OIDC_URL`, `CLICKHOUSE_USER`, `CLICKHOUSE_PASSWORD`, `CLICKHOUSE_DATABASE`
- Optional with defaults: `PORT=8083`, `GITDOT_WEB_URL=http://localhost:3000`, `CLICKHOUSE_URL=http://localhost:8123`

## Auth Model

- The web-vital endpoint accepts an **optional** `Principal` — anonymous events from logged-out visitors are still recorded (user id will be `NULL` in ClickHouse).
- `ClientIp` and `UserAgent` extractors capture network metadata; country/region/city headers (forwarded by Vercel) are read from the request body.
- Vercel OIDC verification middleware is wired through `gitdot-axum` for routes that need it.

## Adding a New Metric Endpoint

1. Add the resource/endpoint types to `gitdot-api/src/resource/metrics.rs` and `gitdot-api/src/endpoint/metrics/`
2. Add the DTO + service method to `gitdot-core` (`MetricsService`)
3. Create `handler/metrics/{name}.rs` with an async function: extract params, build the core DTO, call `state.metrics_service.{method}`, return a status code (most ingestion endpoints return `204 No Content`)
4. Register the route in `handler/metrics.rs` router function
5. Merge router in `handler.rs` `create_metrics_router()`

## Rust Import Ordering

Follows the workspace convention (see the root `CLAUDE.md`): mod declarations, std, then 3rd-party crates, then workspace crates, then `crate`/`super`, each group blank-line separated, with `imports_granularity = "Crate"`. All imports and re-exports come before any declarations or logic.

## Conventions

- Handlers are one function per file, no business logic — delegate to `MetricsService`
- Ingestion endpoints return `StatusCode::NO_CONTENT` on success; failures bubble up via `AppError`
- Tower governor rate-limits all routes
