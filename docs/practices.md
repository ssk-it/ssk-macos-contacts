# Practices

## What goes where

- `src/lib.rs` — the public API: `Access`, `ContactsError`, `access`, `request_access`, `contacts`. It holds no `unsafe` and works on any system: off macOS, there are no Contacts.
- `src/contact.rs` — `Contact`, and how one is made from what the framework gives: its name, its addresses each once. Pure, and tested without Contacts.
- `src/macos.rs` — the calls to the framework, the only `unsafe` (INV-1). As little logic as possible: what can move to `contact.rs`, does.

## Tests

A test needs no contact of the user's, and no permission: what depends on them — the prompt, a real listing — is checked in the application that uses the crate, from its bundle. What can be tested here is: the shape of a contact (`contact.rs`), and that without access nothing is read. Red first: a test that fails for the right reason before the line that makes it pass.

## Errors

`ContactsError` says what the caller can act on: not allowed — with the access as it stands, for the application to say so —, or failed, with what the system said. The crate never panics on what the framework gives.

## Dependencies

The `objc2` family only, released together and grouped by Dependabot. A new dependency is a review point: it must not reach the network or keep anything (INV-2), and its licence must be in `deny.toml` (INV-4).

## Releases

A version is a Git tag `vMAJOR.MINOR.PATCH` (semver), which Skourier takes in its `Cargo.toml`. Tagging is acting outside the repository: it is asked first. The crate is not published on crates.io (`publish = false`).
