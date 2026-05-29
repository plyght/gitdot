//! Core business logic, data access, and external integrations for the gitdot
//! backend.
//!
//! Both the main HTTP server (`gitdot-server`) and the auth server
//! (`gitdot-auth`) depend on this crate and delegate to its services. A request
//! flows down a trait-based, layered architecture:
//!
//! ```text
//! Service (business logic) → Repository (sqlx data access) → PostgreSQL
//! ```
//!
//! Every layer is a trait so it can be mocked in tests. External systems (git,
//! object storage, email, Kafka, …) are reached through the [`client`] module.
//! Data crosses layers as [`dto`] types, is persisted as [`model`] structs, and
//! failures surface as the domain enums in [`error`]. Stateless helpers live in
//! [`util`].

pub mod client;
pub mod dto;
pub mod error;
pub mod model;
pub mod repository;
pub mod service;
pub mod util;

pub(crate) use gitdot_core_derive::instrument_all;
