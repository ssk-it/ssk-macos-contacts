# Clean code

The inside of a unit: how a function reads. What goes in which file is in [`practices.md`](practices.md).

## Readability

- A name says what a thing is for the caller: `contacts`, `request_access`, `contact_of` — not how it is computed.
- A function does one thing at one level: `contacts` gathers, `read_contact` reads one, `contact_of` decides what a contact is.
- An `unsafe` block is as small as the call it wraps, and its `SAFETY:` comment says why that call is sound here (INV-1).

## Design

- What can be pure is pure, and out of `macos.rs`: it can then be tested without Contacts.
- The public API hides the framework's types: an application never sees an `NSString` nor a `CNContact`.

## Refactoring

A change of form never comes with a change of behaviour in the same commit.

## What the tooling enforces

Clippy runs with `-D warnings` in CI, with these thresholds (`clippy.toml`): cognitive complexity 10, nesting 4, arguments 4. `undocumented_unsafe_blocks` fails on an `unsafe` block without its `SAFETY:` comment; `unsafe_code = "deny"` on one outside `src/macos.rs`.
