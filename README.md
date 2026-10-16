# ssk-macos-contacts

A safe Rust API over macOS Contacts: whether the user lets the application read them, a way to ask, and the contacts that have an email address — for an application whose own code may hold no `unsafe`.

It exists for [Skourier](https://github.com/ssk-it/skourier), whose workspace forbids `unsafe` and which suggests the addresses of the user's contacts while a message is written. The framework's Rust bindings ([objc2-contacts](https://crates.io/crates/objc2-contacts)) are all `unsafe`; this crate holds those calls, each with the reason it is sound, behind three functions:

```rust
use std::time::Duration;

let access = ssk_macos_contacts::access(); // asks nothing
let access = ssk_macos_contacts::request_access(Duration::from_secs(60)); // the system's prompt, the first time
let contacts = ssk_macos_contacts::contacts()?; // only once access is granted
```

It reads, and never writes a contact. Off macOS it answers that there are no Contacts. An application that asks must say why in its `Info.plist` (`NSContactsUsageDescription`), or macOS stops it.

The technical documentation is in [`docs/`](docs/README.md). Licensed under the [Mozilla Public License 2.0](LICENSE).
