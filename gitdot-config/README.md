# gitdot-config

Shared configuration types for the [gitdot](https://gitdot.io) platform. Parses and validates `.gitdot.toml` CI config files (user-authored) and is consumed by [`gitdot-core`](../gitdot-core) when executing CI pipelines.

## Schema

```toml
[[builds]]
trigger = "pull_request"   # or "push_to_main"
tasks = ["lint", "test"]

[[tasks]]
name = "lint"
command = "cargo clippy"

[[tasks]]
name = "test"
command = "cargo test"
waits_for = ["lint"]
```

`CiConfig::new(toml)` parses and validates. All validation errors are collected before returning (no fail-fast). See [`src/validate.rs`](src/validate.rs) for the full rule set — duplicate detection, DAG check on `waits_for`, orphan tasks, empty commands, etc.

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
