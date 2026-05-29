//! Database model structs mapped from SQL rows via `#[derive(FromRow)]`.
//!
//! One module per schema/domain — auth, ci, core, metrics, migration, and
//! webhook — mirroring the PostgreSQL layout. Repositories read rows into these
//! structs; services convert them into [`dto`](crate::dto) responses.

mod auth;
mod ci;
mod core;
mod metrics;
mod migration;
mod webhook;

pub use auth::*;
pub use ci::*;
pub use core::*;
pub use metrics::*;
pub use migration::*;
pub use webhook::*;
