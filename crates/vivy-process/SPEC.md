---
id: process
type: module-design
status: draft
title: Process ownership and credential policy (`crates/vivy-process`)
parent: architecture
---

## Responsibility

The two things every component that starts a process needs, in one place.

**Ownership.** `OwnedChild::spawn` gives a child that owns its whole process tree:
on Windows a Job Object with kill-on-close, the child created suspended and
attached before its primary thread is resumed, so no descendant can escape an
assignment race; on Unix a process group killed as a group. Dropping the child, or
dropping an `AbortGuard` held by the calling future, stops the tree — a tool future
dropped by cancellation must stop its process even while a blocking I/O task still
waits on its output. A normal exit is preserved unchanged, so callers can report
the child's own code.

**Credential policy.** Names, never values. `variables_from_file` reads the
configured credential variable names out of `settings.toml`; `ProvidersPlugin`
registers names as its instances load and bumps a generation counter;
`filter_credentials` and `command()` remove those names, every `RIG_SECRET_*`, and
the agent-only `RIG_*` state from a command. A credential-policy change is what
resets a Python worker, so a filtered environment is never stale.

## Boundary

The lowest shared layer: `toml` plus `windows-sys`/`libc`, with no dependency on
Rig or on any plugin crate. That is deliberate — the shell tool, the Python
workbench, provider management and the builder all need these two behaviors, and
sharing them here keeps those components from depending on each other.

Owns process-tree lifetime and the filtering policy. It does not own what a
component decides to spawn, and it makes nothing safe: a filtered environment
still runs trusted code with the user's permissions.

The platform seam is one API with two implementations. Windows is the only one
with recorded runs; Unix keeps compiling and carries no platform claim (see
`goal`).
