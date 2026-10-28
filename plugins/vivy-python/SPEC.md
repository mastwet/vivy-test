---
id: python
type: module-design
status: draft
title: Python workbench dispatcher (`plugins/vivy-python`)
parent: architecture
depends-on: [basics, process]
---

## Responsibility

The `python_repl(code, timeout_secs?)` tool and everything needed to keep it
honest. Each live agent owns one persistent CPython worker: a framed
newline-delimited JSON exchange, one `__main__` namespace, one asyncio loop, cells
with top-level `await`, and background tasks that outlive their cell. Background
output keeps its call identity; output with no owner is retained in the event
journal instead of being attributed to the wrong call.

`worker.rs` is the supervision seam: it drains the worker continuously, and only
the active request may complete its waiter, so a late or queued reply can never
satisfy the wrong call. Cancellation or timeout interrupts the running cell,
allows a bounded unwind, then kills the worker's owned process tree; ordinary
exceptions preserve variables. A change to the credential filtering policy resets
workers so a filtered environment is never stale.

`management` holds the lifecycle commands — `/python prepare`, `status`, `reset`,
`close` — and registers this component's entry with the shared setup coordinator.
What the owner approved for automatic initialization, the consent it is keyed by
and the outcomes it publishes belong to [`architecture`](../../docs/DESIGN.md)
(*Automatic workbench preparation*) and are not restated here. Preparation itself
stays a cancellable owned child, never a side effect of a model's call: it creates a `uv` environment, installs hash-locked dependencies and
trusted Python Skills, probes the result, and only then publishes
`.vivy/workbench/current.json`. A failed or cancelled preparation leaves the
previous environment in place; concurrent preparation reports busy. Reset refreshes
the trusted Skill inventory and starts fresh state on next use; close releases the
worker. Neither removes artifacts or committed memories. `management/status`
publishes the cached environment state, live worker observation and memory binding
as the `basics` `workbench_status` snapshot, without importing anything or waiting
on a running call's lock.

## Boundary

Depends on `basics` for the status contract and the trusted Skill inventory Rig's
Skill prompt also uses — one inventory, not a second catalog — and on `process` for
owned process trees and credential filtering. The embedded Python code is
[`workbench-python`](python/vivy_workbench/SPEC.md).

What it must not become: a second agent loop, a second provider or credential path,
or MCP in Rust. Model calls and delegation requested from Python go through the
host's own gateway, usage and cancellation path.

Invariants: trusted Python and Skills are **not** sandboxed; removing the `python`
recipe component removes this crate and the embedded worker from the build; a
crash or an interrupted write leaves state unknown and explicitly reported, never
silently replayed; compaction keeps the live worker while a new agent, fork,
session switch, resume or exit gets a fresh namespace.
