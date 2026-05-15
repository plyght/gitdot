# gitdot-cli

Command-line client for [gitdot](https://gitdot.io). Sign in to gitdot from your terminal so `git push` to gitdot repositories just works.

The binary is named `dot`.

> **Status: early.** Only authentication commands ship today. Repository management, code review, and CI features will land in subsequent releases.

## Install

```sh
cargo install gitdot-cli
```

This installs the `dot` binary into your Cargo bin directory (usually `~/.cargo/bin`).

> If you have Graphviz installed, its `dot` binary may also be on your `PATH`. Disambiguate with `which -a dot`, or rename the shim if you need both.

## Commands

### `dot login`

Authenticates with gitdot via the OAuth device-code flow. Opens a verification URL and waits for you to enter a one-time code in your browser.

```sh
$ dot login
Open the following URL in your browser:
https://gitdot.io/oauth/device
Enter the code: ABCD-1234
Successfully logged in as mikkel!
```

On success:

- Your username and email are written to `~/.config/gitdot/config.toml`.
- An access token is stored via `git credential approve`, so `git push` to gitdot repositories authenticates automatically against whatever credential helper your system uses (macOS Keychain, libsecret, Windows Credential Manager, etc.).

### `dot status`

Prints the currently logged-in user, or "Not logged in" if no session is active.

```sh
$ dot status
Logged in as mikkel
```

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
