//! Request and response data-transfer objects, plus validated input types.
//!
//! DTOs are the values that cross layer boundaries: requests carry validated
//! inputs into services, and responses are built from [`model`](crate::model)
//! rows via `From`. Split into shared `common` types (pagination cursors,
//! validated `OwnerName`/`RepositoryName`, …), per-`domain` request/response
//! types, and `client` payloads for external integrations.

mod client;
mod common;
mod domain;

pub use client::*;
pub use common::*;
pub use domain::*;
