# Development

## The tree

```
Cargo.toml              The crate, its lints (INV-1) and its macOS-only dependencies
src/lib.rs              The public API
src/contact.rs          A contact, made from what the framework gives
src/macos.rs            The calls to Contacts — the only unsafe code
deny.toml               The dependency policy (INV-2, INV-4)
clippy.toml             The lint thresholds of docs/clean-code.md
rust-toolchain.toml     The Rust every machine builds with
.github/                The checks of every pull request, and Dependabot
```

## Commands

| What | Command |
| --- | --- |
| Format | `cargo fmt --all` |
| Lint, as the CI does | `cargo clippy --all-targets --locked -- -D warnings` |
| Tests | `cargo test --locked` |
| Dependencies | `cargo deny --locked check` |

## CI

The `Checks` workflow runs on GitHub-hosted runners: a macOS one for format, clippy and the tests, an Ubuntu one for `cargo deny`. Not on the organisation's self-hosted runners, which execute whatever a pull request contains — acceptable for a private repository, not for this public one.

## Tooling traps

- **A test binary is not an application bundle.** macOS never shows it the prompt, and stops a process that asks for contacts without `NSContactsUsageDescription`: the tests never call `request_access`. Try it from an application's bundle.
- **The keys of a fetch request are the framework's constants**, read as `unsafe` statics: a key the request did not fetch makes the framework throw when its property is read. `read_contact` reads exactly the four keys `contacts` fetches.
