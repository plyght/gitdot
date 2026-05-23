# gitdot-server

Axum HTTP server that forms the main API layer for [gitdot](https://gitdot.io).

The crate is intentionally thin: handlers extract request parameters, delegate to services from [`gitdot-core`](../gitdot-core), and map results back to JSON via the `IntoApi` trait. The server composes per-domain routers into a single `Router`, applies shared middleware (tracing, CORS, request IDs, timeouts), and exposes three route groups — main API, smart-HTTP git protocol, and internal hooks.

## Auth

A sealed `Authenticator` trait with four schemes: `UserJwt` (Supabase ES256 JWT), `UserToken` (personal access token), `RunnerToken` (CI runner token), and `TaskJwt` (EdDSA task JWT). Handlers declare their auth requirement in the function signature via `Principal<S>`.

## Run

```sh
cargo run -p gitdot-server
```

Reads configuration from environment variables. Key vars: `PORT`, `GIT_PROJECT_ROOT`, `DATABASE_URL`, `GITDOT_PUBLIC_KEY`, `SUPABASE_JWT_PUBLIC_KEY`, `OAUTH_DEVICE_VERIFICATION_URI`, `S2_SERVER_URL`, `VERCEL_OIDC_URL`. See `.env.example` for the full list and [`CLAUDE.md`](CLAUDE.md) for conventions when adding handlers.

A second binary, `gitdot-keygen`, generates the EdDSA keypair used to sign task JWTs.

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
