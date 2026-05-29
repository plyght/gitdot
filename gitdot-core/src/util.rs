//! Stateless helpers shared across the crate's services and repositories.
//!
//! - `auth` — reserved-name checks, server identifiers, and auth email bodies
//! - `crypto` — string hashing for codes and tokens
//! - `cursor` — encode/decode keyset-pagination cursors
//! - `git` — git constants (default branch, zero SHA) and receive-hook scripts
//! - `github` — GitHub clone-URL construction
//! - `image` — deterministic identicon/avatar generation
//! - `review` — `refs/for` magic-ref naming for the review protocol
//! - `template` — bundled gitignore/license templates
//! - `user` — default user profile content

pub mod auth;
pub mod crypto;
pub mod cursor;
pub mod git;
pub mod github;
pub mod image;
pub mod review;
pub mod template;
pub mod user;
