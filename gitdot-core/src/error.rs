//! Domain error enums for the crate, built with `thiserror`.
//!
//! Each domain defines its own enum (e.g. `RepositoryError`, `SessionError`)
//! that carries a `DatabaseError` variant plus domain-specific cases; handlers
//! map these to HTTP responses. Organized into shared `common` errors
//! (database, not-found, conflict, input), per-`domain` errors, and `client`
//! errors for external integrations.

mod client;
mod common;
mod domain;

pub use client::*;
pub use common::*;
pub use domain::*;
