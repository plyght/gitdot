# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this crate.

## Commands

```bash
cargo check -p gitdot-cli            # Type check
cargo build -p gitdot-cli            # Build
cargo run -p gitdot-cli -- <cmd>     # Run (e.g., `-- login`)
cargo test -p gitdot-cli             # Run tests
cargo +nightly fmt -p gitdot-cli     # Format
cargo build -p gitdot-cli --release  # Release build (binary: dot)
```

## Architecture

The binary is named `dot`. Entry point: `src/bin/main.rs` → initializes rustls (`bootstrap::load_rustls()`) → parses CLI args → routes to the chosen command.

### Shipped commands

Only `login` and `status` are wired into `src/command.rs` today. Additional commands (`save`, `review`, `ci`, `runner`) live in `src/command/` but are commented out in the `Args` enum until their server-side counterparts ship. When re-enabling, uncomment the variant and re-export the corresponding `*Args` import.

### Command flow

Each command follows: `load config → build client → execute`.

- `login` / `status` → `UserConfig::load()` (`~/.config/gitdot/config.toml`) + `GitdotClient` with JWT
- `runner` (when re-enabled) → `RunnerConfig::load()` (`/etc/gitdot/runner.toml`) + `GitdotClient` with Basic auth

### Key modules

| Module | Role |
|--------|------|
| `client/gitdot.rs` + `client/methods/` | `GitdotClient`: reqwest wrapper with JWT or Basic auth; methods grouped by domain |
| `client/git.rs` | `GitWrapper`: async wrapper over the git CLI via `tokio::process::Command` |
| `client/credential.rs` | Credential storage via `git credential approve` |
| `config/user.rs` | `UserConfig`: async load/save, `~/.config/gitdot/config.toml` |
| `config/runner.rs` | `RunnerConfig`: sync load/save, `/etc/gitdot/runner.toml` |
| `executor/local.rs` | `LocalExecutor`: clones repo into `/tmp/gitdot/tasks/{id}`, runs `sh -c` command, streams output to S2 |
| `os/service.rs`, `os/install_service.rs` | `Service` trait with `launchd` (macOS) and `systemd` (Linux) impls via `#[cfg(target_os)]` |
| `util/ci.rs` | Finds `.gitdot-ci.toml` by walking up from cwd |

### Config defaults

```
Web:    https://gitdot.io
API:    https://api.gitdot.io
S2:     https://s2.gitdot.io
```

Overridden by `gitdot_server_url` / `gitdot_web_url` / `s2_server_url` in config files.
