# gitdot-auth

Standalone Axum HTTP server for authentication flows on [gitdot](https://gitdot.io). Handles device-code login, GitHub OAuth, session refresh, email verification, and Slack account linking.

This crate is a thin transport layer — handlers extract request parameters and delegate to `AuthenticationService` in [`gitdot-core`](../gitdot-core). It runs as its own binary so the auth surface area (rate limiting, CORS, secrets) can be scaled and audited independently of the main API.

## Run

```sh
cargo run -p gitdot-auth
```

Reads configuration from environment variables (see `.env.example`). All settings — including secrets — are loaded at startup via [figment](https://crates.io/crates/figment); the server fails fast if a required variable is missing. In production these are injected via Cloud Run's Secret Manager bindings.

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
