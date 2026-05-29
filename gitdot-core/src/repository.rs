//! Data-access layer: one trait plus a sqlx-backed `Impl` per domain.
//!
//! Repositories own the SQL for a domain (raw `sqlx` queries, transactions via
//! `pool.begin()`), returning [`model`](crate::model) structs wrapped in
//! `Result<_, DatabaseError>`. Each trait is `Clone` so services can hold and
//! share it. Grouped into auth, ci, core, migration, and webhook.

mod auth;
mod ci;
mod core;
mod migration;
mod webhook;

#[cfg(all(test, feature = "db-tests"))]
mod test_common;

pub use auth::*;
pub use ci::*;
pub use core::*;
pub use migration::*;
pub use webhook::*;
