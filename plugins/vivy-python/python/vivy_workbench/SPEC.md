---
id: workbench-python
type: submodule-design
status: draft
title: Embedded Python runtime (`plugins/vivy-python/python/vivy_workbench`)
parent: python
---

## Responsibility

The trusted Python package the Rust dispatcher runs (`python -m
vivy_workbench.repl`), adapted from Prime's bounded REPL and MCP mechanisms under
the retained MIT notice.

- `repl` — the framed stdio protocol and the runtime: one namespace, one event
  loop, cells with top-level `await`, in-cell host callbacks serviced while a cell
  result is awaited (otherwise a model or tool call from Python would deadlock the
  host).
- `mcp` — a kernel-owned generic MCP client registry with two surfaces: dispatch
  (`list_tools`, `call_tool`) and discovery (`list_connections`, `search_tools`,
  `describe_tool`, `reload`, `close`). Connections open lazily, are reused, support
  stdio and Streamable HTTP, and receive a filtered environment. Adding a service
  is configuration data, not a new module.
- `memory` — a thin facade over the native MemPalace engine: `remember`, `search`,
  `get`, `close`, with explicit source/wing/room filters, complete chunk-validated
  reads and one serialized worker for the synchronous engine. Its narrowing is the
  point: the facade is not MemPalace's ingestion or hybrid ranking, and its chunk
  `limit` precedes logical-record deduplication.
- `capabilities` — binds only host-discovered trusted Skills, lazily; instructions
  stay files. A broken import, missing host handler, name collision or reserved
  name is reported as unavailable without hiding the other Skills.
- `artifacts` — complete artifacts with descriptors; a failed write never yields a
  complete one. `prepare` — explicit `uv` preparation that publishes an atomic
  pointer only after a fresh environment probes. `errors` — the typed MCP error
  that keeps the complete failing envelope.

## Boundary

Runtime code only: it does not own an agent loop, a model SDK path, a provider
credential, or the environment pointer's publisher (that is the Rust `management`
side). It must not import MemPalace's stdio MCP server — that server redirects
stdout and loads a broad surface unsuitable inside the workbench — nor ship a
second memory engine.

Invariants: an MCP call is never retried automatically, and a missing completion
means unknown, not safely retryable; error envelopes and full text are preserved
rather than truncated into a false success; nothing here is a sandbox, so the code
runs with the user's permissions.
