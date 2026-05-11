# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot-consumer` is a long-running Kafka consumer that subscribes to `gitdot.repo.pushed`, looks up the Slack webhooks subscribed to each repo's `Push` events, and notifies the gitdot Slack bot server. Like the other binary crates, it's a thin shell — message handling logic lives in `WebhookService` (defined in `gitdot-core`).

## Structure

- `app.rs` — `GitdotConsumer` entrypoint: bootstrap, settings, pool, state, subscribe
- `app/settings.rs` — `Settings` loaded via figment from env vars
- `app/state.rs` — `ConsumerState` (the consumer's `AppState` equivalent), plus `ConsumerHandle` enum (Plain vs Gcp) and `build_consumer` for SASL OAUTHBEARER on GCP Managed Kafka
- `app/runner.rs` — Run loop: stream messages, dispatch to `handle_message`, commit offset on success. SIGTERM/SIGINT handlers for graceful shutdown
- `app/bootstrap.rs` — dotenvy + tracing-subscriber + rustls crypto provider
- `bin/main.rs` — Entry point: `GitdotConsumer::new().await?.run().await`

## Configuration

All settings — including the Slack HMAC secret — are loaded from environment variables at startup via figment (`Figment::new().merge(Env::raw()).extract()`). In production these are injected via Cloud Run's Secret Manager bindings; locally they come from `.env`.

- Required fields (`DATABASE_URL`, `GITDOT_SLACK_SECRET`) have no default — figment fails fast with "missing field X" if unset
- Optional fields use `#[serde(default = "fn_name")]`
- `kafka_auth: KafkaAuthMode` deserializes from `local` / `gcp_oauthbearer` via a small adapter (`deserialize_kafka_auth`)

See `.env.example` for the full var list.

## Consumer Semantics

- **At-least-once delivery**: offset is committed only after `handle_message` returns `Ok`. Handler failure logs and skips the commit so the partition redelivers.
- **Sequential per-partition**: one message at a time. Fan-out to multiple Slack channels happens inside `handle_message` and is currently sequential (TODO: parallelize).
- **Graceful shutdown**: SIGTERM/SIGINT break the loop; in-flight messages finish, then the consumer exits.

## Adding a New Event Type

Today the consumer handles `gitdot.repo.pushed` only. To handle a new topic:

1. Add a new topic constant alongside `REPO_PUSHED_TOPIC` in `app.rs`
2. Subscribe to it in `GitdotConsumer::new()` alongside the existing `subscribe(&[REPO_PUSHED_TOPIC])` call
3. In `runner.rs::handle_message`, branch on `msg.topic()` and dispatch to a per-topic handler fn
4. New handler reads `msg.payload()`, deserializes, calls into a `WebhookService` (or new service) method, returns `Result<()>`

If a new event needs different repository or client access, extend `ConsumerState::new` to build them — no `secret_client` parameter; pull from `settings` directly.

## Rust Import Ordering

```rust
// 1. mod declarations
mod runner;

// 2. std imports
use std::sync::Arc;

// 3. 3rd-party crate imports
use anyhow::Context;
use rdkafka::consumer::Consumer;

// 4. Workspace crate imports
use gitdot_core::{dto::RepoPushEvent, service::WebhookService};

// 5. crate/super/self imports
use crate::app::ConsumerState;

// 6. pub use re-exports
pub use settings::Settings;
```

Separate each group with a blank line. Merge imports from the same crate (`imports_granularity = "Crate"`). All imports and re-exports must come before any declarations or logic.

## Conventions

- Handler functions return `anyhow::Result<()>`; errors bubble up, log at the call site, and skip the offset commit
- No business logic in `handle_message` — deserialize → build request DTO → call service → done
- Kafka producer/consumer authentication uses `KafkaAuthMode` shared with `gitdot-server`. GCP OAUTHBEARER wraps a Google access token in a `GOOG_OAUTH2_TOKEN`-flavored JWT envelope (see `gitdot-core/src/client/kafka.rs`)
