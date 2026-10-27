---
id: goal
type: goal-and-requirements
title: vivy-test — one recipe, one assembled agent
status: draft
---

# vivy-test

## Goal

Test one idea cheaply: **a YAML recipe selects Rust components, and a build
assembles one runnable agent from them.** Change the recipe, rebuild, and the
compiled agent changes; runtime settings configure capabilities that are already
compiled in but never install new ones.

The idea is Vivy's, kept deliberately smaller. The original Vivy implementation
resolved typed Ports between modules and wrapped selection in species hashes,
grants, conformance records and a Studio lifecycle. This repository keeps only
*selection → static wiring → executable* and ordinary correctness checks.

## Problem

Agent harnesses select capabilities at runtime through configuration, plugin
manifests or an extension runtime, so what a running agent can do is a question
about a mutable environment. Here the selection is a build input: the plugin
types a recipe names are the ones compiled in, and a component left out of the
recipe is a crate absent from the built agent's dependency graph. That makes the
compiled product inspectable and cheap to rebuild in variants, without a plugin
SDK, a manifest format or a supervisor.

## Audience

The owner/operator, running it on Windows from this checkout. The public
repository records the experiment; it is not a published product, an installer,
or a compatibility promise. *(Audience and framing confirmed by the owner at
spec adoption.)*

## Capabilities

- **Recipe assembly.** `vivy generate|build <recipe.yaml>` checks a recipe
  (pinned Rig revision, ordered components naming crates and plugin types, Cargo
  features), writes the generated package, and builds the agent.
- **One assembled coding agent.** The baseline recipe produces an interactive
  terminal plus a print/JSON front: streaming model calls, coding tools (read,
  edit, write, attach, Git Bash shell, `search`/`list`), sessions with resume,
  fork, rewind and branch summary, compaction, usage telemetry.
- **Pi-shaped workflow surface.** `AGENTS.md`/`CLAUDE.md` context files and
  `SYSTEM.md`/`APPEND_SYSTEM.md`, prompt templates as commands, Markdown skills
  behind project trust, Pi's session and model command names, collaborative Plan
  mode with a user-reviewed exit.
- **Native delegation.** `task`, `message`, `wait`, bounded parallel batches and
  chains over Rig's subagent lifecycle, with role files, per-child views and
  usage.
- **Provider management.** OpenAI- and Anthropic-compatible instances with
  presets, per-model facts, hidden key entry, and a gateway path where a model ID
  reaches the endpoint unchanged.
- **Optional Python workbench.** A per-agent persistent CPython namespace (top
  level `await`, tasks outliving a cell), explicitly prepared with `uv`, trusted
  Python Skills, a generic MCP client registry for stdio and Streamable HTTP,
  complete artifacts, and an optional native MemPalace facade.
- **Housekeeping.** Credential environment-variable *names* in configuration,
  secrets withheld from every process the agent starts, redacted error causes,
  and one-click Windows build/start scripts.

## Non-goals

Excluded by decision; something neither listed here nor implemented above is
open, not forbidden.

- Species hash enforcement, provenance or attestation platform, conformance
  platform, Studio lifecycle.
- A standalone plugin SDK, plugin marketplace, or universal Port/Provider
  catalog; a recipe component stays a named group of plugin types from one crate.
- Pi TypeScript extension compatibility, Pi RPC/wire compatibility, or a Pi
  source API promise.
- Any dependency on the original Vivy repository, its source, sessions,
  credentials or production data.
- A second agent loop, second provider stack or credential path inside the Python
  workbench; Python calls back through the host for agents and models.
- Substituting a runtime enable flag for compiled selection.

## Principles

- **One composition root, one execution path.** Rust owns the agent loop, tools,
  sessions and terminal; Python owns libraries, MCP clients and bounded
  composition, and reaches agents and models through the host.
- **Reuse before writing.** Rig supplies the harness, terminal, providers,
  session store and subagent lifecycle; a Vivy plugin is a small typed plugin or a
  wrapper, added only where selection or product behavior needs it.
- **The omission guarantee is the plugin-crate boundary.** A plugin needed
  transitively stays a dependency; hiding a tool, linker dead-code removal and
  excluding a crate are different things, and only the last is a selection claim.
- **Claims follow recorded runs.** Source inspection is not platform acceptance:
  a behavior is reported as verified only for the path, platform and provider a
  run actually exercised, and a failure/cancellation check accompanies an
  execution change. This is why the limitation below is stated as it is.
- **Cross-platform is the goal; claims are per platform.** Windows carries the
  only recorded runs today, and Linux/macOS support is a goal — each platform's
  claim waits for its own run rather than following from another's evidence.
  *(Owner decision at spec adoption; supersedes the earlier Windows-only ruling
  in [`architecture`](docs/DESIGN.md).)*

## Known limitation

- **No real-provider acceptance.** Every model-integration result comes from a
  scripted loopback gateway. The owner deferred the real-endpoint run, so no
  provider-compatibility claim holds for any gateway or vendor — streaming and
  tool calls against a real endpoint are unverified.

The architecture and its decisions are in [`architecture`](docs/DESIGN.md).
