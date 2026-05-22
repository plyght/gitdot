# gitdot-axum

Shared Axum server utilities for the gitdot backend.

This crate exists so process bootstrap, request extractors, and middleware are
written once and reused across the binary crates (`gitdot-server`,
`gitdot-auth`, `gitdot-consumer`) rather than copy-pasted into each. It holds no
business logic and no HTTP handlers.

## Modules

- **`bootstrap`** — startup helpers: `load_env`, `install_crypto_provider`, and
  `init_tracing` / `init_tracing_with`.
- **`config`** — typed config structs (`AuthConfig`, `VercelOidcConfig`) that a
  host service stores in its `AppState` and exposes via `FromRef`.
- **`extract`** — axum `FromRequestParts` extractors: `Principal` (gitdot JWT
  auth), `ClientIp`, `UserAgent`.
- **`middleware`** — `log_request` (structured request logging) and
  `verify_vercel_oidc` (Vercel OIDC token verification).

## Usage

A host service depends on the crate and calls the `bootstrap` helpers at
startup:

```rust
gitdot_axum::bootstrap::load_env();
gitdot_axum::bootstrap::install_crypto_provider();
gitdot_axum::bootstrap::init_tracing("info,tower_http=debug,axum::rejection=trace");
```

To use the auth-aware extractor or middleware, implement `FromRef` for the
relevant config type on the service's `AppState` — e.g. `AuthConfig` for the
`Principal` extractor, `VercelOidcConfig` for the `verify_vercel_oidc`
middleware.
