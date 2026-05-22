# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Purpose

`gitdot-axum` houses Axum server utilities shared across the backend binary
crates — `gitdot-server`, `gitdot-auth`, and `gitdot-consumer`. It holds no
business logic and no HTTP handlers; it is a library of reusable startup
helpers, request extractors, middleware, and the config types they read.

## Structure

- `bootstrap.rs` — process startup helpers: `load_env` (dotenvy),
  `install_crypto_provider` (rustls aws-lc-rs), and `init_tracing` /
  `init_tracing_with` (terminal-aware fmt layer; the `_with` variant appends
  caller-supplied layers such as an OpenTelemetry bridge)
- `config.rs` — `AuthConfig` (gitdot JWT public key) and `VercelOidcConfig`
  (Vercel OIDC JWKS + issuer). Plain `Clone` structs; host services expose them
  via `FromRef<AppState>`
- `extract/` — axum `FromRequestParts` extractors: `Principal` (verifies a
  gitdot Bearer JWT against `AuthConfig`), `ClientIp`, `UserAgent`
- `middleware/` — `log_request` (structured per-request tracing event) and
  `verify_vercel_oidc` (validates the `x-vercel-oidc-token` header against
  `VercelOidcConfig`)

`lib.rs` declares the four `pub mod`s. Each `extract/` and `middleware/`
submodule is one file per item, re-exported through `extract.rs` /
`middleware.rs`.

## Conventions

- No business logic and no handlers — this crate is purely shared
  infrastructure. Domain-specific code belongs in `gitdot-core` or the
  consuming crate
- Extractors that need config are generic over the state `S` with a
  `Config: FromRef<S>` bound, so any service can use them by implementing
  `FromRef`
- Infallible extractors use `Rejection = Infallible`; auth extractors reject
  with `StatusCode::UNAUTHORIZED`
- Keep one item per file under `extract/` and `middleware/`, re-exported from
  the parent module file

## Rust Import Ordering

Follows the workspace convention (see the root `CLAUDE.md`): std, then
3rd-party crates, then workspace crates, then `crate`/`super`, each group
blank-line separated, with `imports_granularity = "Crate"`.
