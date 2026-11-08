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

The `Advisories` workflow (`.github/workflows/advisories.yml`) runs `cargo deny --locked check advisories` every Monday on `main`, on the same Ubuntu runner, and by hand from the Actions tab: the advisory database changes every day, `Cargo.lock` does not. GitHub disables the schedule of a public repository after sixty days without a commit; the Actions tab then offers to enable it again.

## When the dependency check is red

`cargo deny --locked check` reads `deny.toml` and the graph of `Cargo.lock`. It can turn red on a pull request that touched no dependency: the advisory database it downloads changes every day, and a new advisory against a crate already locked is enough. That is what it is for. A red Monday run is handled the same way, in a pull request of its own; GitHub tells of a failed scheduled run by email to the last person who changed its `cron` line. The message of a red run names the check that failed:

- **`vulnerability`, `unsound`, `yanked`, `unmaintained`** — move the crate to a fixed version: `cargo update -p <crate>`, or `--precise <version>` for the one the advisory's "Solution" gives; the `objc2` family moves together (practices.md, "Dependencies"). With no fixed version yet, read whether the advisory concerns how this crate uses it; only if it does not, add its ID to `ignore` under `[advisories]` in `deny.toml`, with the reason and the date — it is a decision, reviewed like code, and removed once a fix ships.
- **`rejected`** (a licence) — a new crate needs a licence the allow-list lacks. Allowing it is a decision about what Skourier ships (INV-4): add it to `deny.toml` with the crate that needs it, never a copyleft stronger than the MPL's.
- **`license-not-encountered`** — the last crate needing a licence left: remove the licence from the list.
- **`banned`** — a crate that `[bans]` refuses: it reaches the network or keeps something (INV-2). The dependency that brought it goes, not the ban.
- **`source-not-allowed`**, **`wildcard`** — a dependency from a Git repository or another registry, or without a version: name the version, or list the source in the same pull request, saying why.


## Tooling traps

- **A test binary is not an application bundle.** macOS never shows it the prompt, and stops a process that asks for contacts without `NSContactsUsageDescription`: the tests never call `request_access`. Try it from an application's bundle.
- **The keys of a fetch request are the framework's constants**, read as `unsafe` statics: a key the request did not fetch makes the framework throw when its property is read. `read_contact` reads exactly the four keys `contacts` fetches.
