# Documentation

Read in this order when you arrive:

1. [`invariants.md`](invariants.md) — INV-1…INV-4, what the crate never does, and what checks each rule.
2. [`practices.md`](practices.md) — what goes where, how it is tested, how errors are shaped, how a version ships.
3. [`development.md`](development.md) — the tree, the commands, the CI and its traps.
4. [`clean-code.md`](clean-code.md) — the inside of a function, and the thresholds the linter enforces.
5. [`conventions.md`](conventions.md) — language, naming, comments, commits, Markdown.

Why the crate exists was decided with Skourier's user on 2026-10-17, while writing its address suggestions (framing 2026-008 of [`ssk-it/skourier-cadrage`](https://github.com/ssk-it/skourier-cadrage)): the bindings of Contacts are all `unsafe`, and Skourier's workspace forbids it — a dependency may hold it, reviewed on its own.
