# gitdot-api

Shared API contract for [gitdot](https://gitdot.io) — resource types and endpoint definitions used by the gitdot backend, CLI, and frontend.

> Published primarily so that [`gitdot-cli`](https://crates.io/crates/gitdot-cli) can be installed via `cargo install`. Most users will not depend on this crate directly.

## What's in here

- **`resource/`** — Serializable response types (`UserResource`, `RepositoryResource`, `ReviewResource`, etc.).
- **`endpoint/`** — Zero-sized types implementing the `Endpoint` trait, pairing an HTTP method and path with request/response types.
- **Core traits** — `ApiResource`, `ApiRequest`, and `Endpoint`, plus `#[derive(ApiResource)]` / `#[derive(ApiRequest)]` from the companion [`gitdot-api-derive`](https://crates.io/crates/gitdot-api-derive) crate.

## Example

```rust
use gitdot_api::endpoint::user::GetUser;
use gitdot_api::Endpoint;

assert_eq!(GetUser::PATH, "/user");
assert_eq!(GetUser::METHOD, http::Method::GET);
```

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
