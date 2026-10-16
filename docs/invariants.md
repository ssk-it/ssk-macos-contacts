# Invariants

The constraints this crate does not negotiate. Each one is numbered so that it can be quoted — in a review, a commit message, a code comment — and the numbers are stable: a lifted invariant keeps its number, marked as lifted, with the date and the reason.

## Changing an invariant

An invariant is not an obstacle to work around. If one gets in the way, either change approach, or lift it explicitly: a pull request that edits this file, says why, and changes the checks accordingly. Weakening a check to get a change through — allowing `unsafe` in another module, removing a banned crate from `deny.toml` — is changing the invariant without saying so.

## INV-1 — The `unsafe` code lives in one module, each block with its reason

**The rule.** Only `src/macos.rs` may hold `unsafe`; every block in it says, in a `SAFETY:` comment, why it is sound.

**What it protects.** The reason the crate exists: Skourier forbids `unsafe` in its own workspace and takes it here, where it can be read in one place. Spread or unexplained, it could not be reviewed.

**Without it.** A convenient `unsafe` slips into another file; a block's soundness rests on something nobody wrote down, and a framework change breaks it silently.

**Checked by.** `unsafe_code = "deny"` in `Cargo.toml`, which `src/macos.rs` alone allows; clippy's `undocumented_unsafe_blocks`, a CI failure with `-D warnings`. Both were seen failing on a stray block and an unexplained one.

## INV-2 — It reads, and nothing leaves

**The rule.** The crate reads contacts and the access the user gave; it never writes a contact, never keeps one, and reaches no network.

**What it protects.** The user's address book, which says with whom they live and work. What an application does with what it reads is its own promise (Skourier's `RG-adresses-proposees-sans-tiers`); this crate adds nothing to it.

**Without it.** A cache of contacts in a file, a request somewhere: nothing in the application would show it.

**Checked by.** `deny.toml` bans the network and storage crates. **Not tooled**: a call to a writing method of the framework — reviewed in `src/macos.rs`, the only place it could be.

## INV-3 — Nothing is asked unless the caller asks

**The rule.** Only `request_access` shows the system's prompt; `access` and `contacts` never do.

**What it protects.** The moment the user is asked, which the application chooses — when its user wants suggestions from their contacts, not when it starts.

**Without it.** A prompt at a moment the user cannot relate to anything, which they refuse for good.

**Checked by.** The test `reads_nothing_without_access`: without access, `contacts` answers `NotAllowed` and reads nothing. **Not tooled**: that `access` never prompts — it calls only `authorizationStatusForEntityType`, which the framework documents as reading the status.

## INV-4 — Nothing stronger than the MPL enters

**The rule.** Every dependency is under a licence in `deny.toml`'s allow-list.

**What it protects.** Skourier, and any application that builds this crate in.

**Checked by.** `cargo deny check` in CI, an allow-list that refuses any licence it does not name.
