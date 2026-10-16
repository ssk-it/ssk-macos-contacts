# ssk-macos-contacts — instructions for an agent

ssk-macos-contacts is a small Rust crate that holds the `unsafe` calls to macOS Contacts behind a safe API, for Skourier. The project documentation is in `docs/` and `README.md`; this file only says where to read and what concerns an agent.

## Where to read

**Before any change**, read `docs/invariants.md` and `docs/practices.md`.

| Document | What it holds |
| --- | --- |
| `docs/invariants.md` | INV-1…INV-4 — the non-negotiable constraints, and what checks each |
| `docs/practices.md` | What goes where, tests, errors, dependencies, releases |
| `docs/development.md` | Tree, commands, CI and its traps |
| `docs/clean-code.md` | The inside of a function, and the lint thresholds |
| `docs/conventions.md` | Language, naming, comments, commits, Markdown |

Do not copy here what those files say: read them there.

## How to work

**Check rather than assume.** Before stating that a method of the framework is sound, or exists, read its documentation and the binding's signature. Before concluding that a check works, watch it fail once on a defect introduced on purpose.

**Red test first.** Always: a test that fails for the right reason, before the line that makes it pass. What can be tested without the user's contacts — the shape of a contact, the refusal without access — is tested; what needs them is said, and checked in the application.

**Every `unsafe` block says why it is sound** (INV-1). A block whose reason cannot be written is not written.

**Never commit on `main`.** One worktree per topic, branched from `origin/main`, one pull request per topic.

**Ask before acting outside the repository**: pushing, tagging a release, changing the repository's settings or visibility.

**Keep the documentation true.** A change that makes a document wrong fixes it in the same pull request; a newly met tooling trap goes to `docs/development.md`, not here.

**Language.** Everything in the repository is in English. Talk to the user in the language they write in.
