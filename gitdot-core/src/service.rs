//! Business-logic layer: one trait plus an `Impl` per domain.
//!
//! Services orchestrate [`repository`](crate::repository) data access and
//! [`client`](crate::client) integrations, enforce authorization and domain
//! rules, and return [`dto`](crate::dto) responses. Each service is generic over
//! its repository/client traits so it can be unit-tested with mocks. Grouped
//! into authentication, authorization, ci, core, metrics, migration, and
//! webhook.

mod authentication;
mod authorization;
mod ci;
mod core;
mod metrics;
mod migration;
mod webhook;

#[cfg(test)]
mod test_client;
#[cfg(test)]
mod test_common;
#[cfg(test)]
mod test_repository;

pub use authentication::*;
pub use authorization::{AuthorizationService, AuthorizationServiceImpl};
pub use ci::*;
pub use core::*;
pub use metrics::{MetricsService, MetricsServiceImpl};
pub use migration::{MigrationService, MigrationServiceImpl};
pub use webhook::{
    EventService, EventServiceImpl, GithubWebhookService, GithubWebhookServiceImpl,
    SlackWebhookService, SlackWebhookServiceImpl, WebhookService, WebhookServiceImpl,
};
