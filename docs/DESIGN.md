# Decisions and assembly direction

Recorded on 2026-10-24. This is an experiment brief, not an implemented API or a
production compatibility promise. The user's decisions are fixed below; the
runtime integration choice remains open to a small working spike.

## Accepted decisions

| Decision | Practical consequence |
| --- | --- |
| Independent Rust test repository | No dependency on the original Vivy repository or its recovery work. |
| YAML recipe and compile-time composition are the central experiment | The recipe determines selected plugin dependencies and static constructor calls. |
| Rig is the implementation foundation | Reuse its components and, where economical, its existing harness and terminal plugins. |
| Pi coding-agent is the functional reference | Track the complete inspected coding-agent surface; deliver it incrementally. |
| Rust/Vivy extension equivalents | Do not implement compatibility with existing Pi TypeScript extension source. |
| Gateway-first model access | First integrate one gateway; avoid rebuilding individual provider login, routing, and catalog infrastructure. |
| Run first; keep mechanisms inexpensive | The first end-to-end coding task precedes a generalized plugin platform. |
| Skip species hashes and a standalone SDK | No content-addressed species identity, sealed manifests, attestation, or public SDK release work. |

The full Pi baseline is a coverage target, not the first delivery's acceptance
gate. Gateway-first access intentionally defers Pi's broad direct-provider and
OAuth surface. Embedding can initially use ordinary Rust library APIs; an
independently packaged SDK is unnecessary. Do not declare complete Pi parity
while those or other baseline rows remain incomplete.

## What Vivy's assembly idea contributes

Vivy selects modules before the application runs. A recipe names modules; module
descriptions identify provided/required typed Ports; a compiler resolves their
Provider/Consumer relationships and lifecycle order. It generates imports and
constructor calls, then builds the selected product. Runtime configuration can
configure capabilities already present, but cannot install new module code.

The inspected Go implementation has concrete seams:

- `sdk/internal/assembly/compiler.go`: `Compiler.Compile` resolves the recipe
  into an `AssemblyPlan` with modules, Port edges, and lifecycle order.
- `sdk/internal/assembly/generate.go`: `GenerateBinder` emits static imports,
  constructors, and lifecycle handling from that plan.
- `sdk/internal/assembly/runtime_generate.go`: `GenerateRuntimeAssembly` emits
  the application-facing provider inventory from the same plan.
- `sdk/internal/frontend_v1.go`: `Pack` prepares generated assembly and builds
  `cmd/vivy` using a Go overlay.

Vivy also surrounds that pipeline with trust classes, Grants, source hashes,
conformance records, embedded identity, SDK restrictions, and artifact inspection.
Those mechanisms greatly increase the implementation surface and are not needed
to test recipe-driven assembly. This experiment keeps **selection -> static
wiring -> executable** and ordinary correctness checks. It does not copy Vivy's
compiler, public schemas, or authority framework.

This account came from local source inspection at Vivy revision
`525d50a67152d6e9f5cc2eb469c889c6f26c21ab`. It describes the inspected construction
path, not the health of that repository or successful validation of its product.
The paths are explanatory landmarks only; successors must not clone Vivy to
follow them.

## What Rig already contributes

At the inspected revision, Rig offers two relevant integration levels:

| Route | Reuse | Cost or uncertainty |
| --- | --- | --- |
| `rig-harness` + `rig-ecs` + selected `rig-basics` / `rig-tui` plugins | Existing terminal agent, tools, sessions, compaction, commands, plugin assembly | Bevy/ECS concepts and dependencies; inspected launcher documents Linux/macOS, so Windows must be investigated. |
| `rig-core` + `rig-agent` + selected tool/memory crates | Model contracts, an existing agent runtime, tools and memory primitives | More product work for terminal interaction, session UX, extensions and RPC; a small loop does not imply a small complete coding agent. |

Start by checking whether the existing harness can satisfy the initial run with
less owned code. Choose the lower-level route if platform or integration evidence
makes it cheaper overall. Do not implement both runtimes or write a new agent loop
just to give it Vivy names. Earlier advice favoring core/agent addressed a narrower
experiment; the expanded Pi coverage makes harness reuse worth checking first.

Rig's launcher already demonstrates static assembly:

```text
plugins.toml
  -> generated Cargo.toml (selected plugin crates)
  -> generated src/main.rs (ordered load::<PluginType> calls)
  -> cargo build
  -> executable using rig-harness
```

`src/launcher/project.rs` contains `generate`, `manifest`, and `main_rs` for this
path. A generated program installs the harness and calls
`rig_harness::load::<PluginType>` for each selection. `src/launcher/build.rs`
handles building. Reloading involves rebuilding and restarting; it is not native
plugin hot-loading. See the pinned [project generator][rig-project] and
[harness description][rig-harness].

## Assembly path

```text
recipe.yaml
  -> small parser and explicit component resolution
  -> generated Cargo.toml + static Rust entry point
  -> one ordinary Cargo build of the selected agent
  -> run with gateway and session settings
```

One recipe format and one assembly path are enough. The builder may be a small
repository-local script or command. It does not need an SDK, service, daemon,
schema registry, or self-update system. If Rig's generator is reusable, YAML can
adapt to that path; do not maintain independent hand-edited YAML and TOML sources
of the same selections. Generated files must be marked and regenerated.

The implemented form is [`recipes/coding-agent.yaml`](../recipes/coding-agent.yaml):
a pinned Rig `git` + `rev`, then an ordered list of components. A component
names one crate (a Rig crate, or this repository's by `path`), the plugin types
(`Plugin + Default`) it adds, and the Cargo `features` it needs of any selected
crate. There is no hidden component catalog: the recipe says which types are
compiled in. `crates/vivy` (`vivy build <recipe>`) checks the recipe, writes
`.generated/<name>/Cargo.toml` and `src/main.rs` (marked generated, rewritten
only when changed), keeps the resolved lock next to the recipe as
`<name>.Cargo.lock`, and runs `cargo build`. The generated `main.rs` mirrors
Rig's launcher: the harness plugins, then `rig_harness::load::<T>` per plugin in
recipe order, then `rig_harness::run`.

The builder rejects unknown fields (with the line), duplicate components or
plugin types, empty components, malformed names and type paths, a plugin path
outside its component's crate, two sources for one crate, and features for an
unselected crate. Whether a type exists and is a plugin is left to rustc, which
names the missing type at the generated line. Plugin crates are built with
`default-features = false`; this matters because `rig-basics` enables
`rig-tui` through its default `tui` feature.

Runtime settings stay out of the recipe and use Rig's own
`RIG_HOME/settings.toml`. The gateway takes base URL and model ID there; its key
is the agent secret `gateway` (`RIG_SECRET_GATEWAY`), not a configurable plain
environment variable, because the harness withholds `RIG_SECRET_*` variables
from every process it starts (shell and tools) and would pass any other
variable on to them.

A Vivy plugin can be a Rust crate with a small typed installation function, or a
thin wrapper around an existing Rig plugin. Wrap only where selection or product
behavior needs it. Do not invent a universal interface covering every future
feature, or hide useful Rig types behind duplicate type hierarchies.

**The omission guarantee is at the selected plugin-crate boundary.** A plugin
needed transitively remains a dependency. Removing a wrapper that shares
`rig-tools` with another wrapper does not exclude individual tool modules from
that upstream crate's compilation. Likewise, selecting one provider does not
prove all other provider source modules were excluded. `rig-agent` also depends
on `rig-memory`. Verify the actual Cargo graph; do not confuse hiding a tool,
linker dead-code removal, and excluding a crate from compilation.

"One compilation" means one assembled product build per recipe selection, with
Cargo's normal dependency builds and incremental cache. It does not mean one
`rustc` process, no build-time helper, or never rebuilding after a recipe change.
Compiled MCP support may discover remote tools; that is runtime data behind an
existing capability, not installation of new native Rust plugins.

## First proof and subsequent direction

The first working slice should establish:

1. A recipe builds an agent that accepts a task, reaches the configured gateway,
   streams output, and completes a real file-edit/command task in a disposable
   workspace. It can continue the conversation.
2. One meaningful optional plugin is removed by changing the recipe and
   rebuilding. Its registration disappears, and its crate disappears from the
   generated package's resolved graph when no selected component depends on it.
3. Invalid selection fails with an actionable error; a tool/model failure is
   visible; cancellation stops owned work. No credentials appear in tracked
   configuration or diagnostic output.
4. Record actual cold build, warm rebuild, and one plugin-edit rebuild times,
   including toolchain, machine and target. Separate dependency downloads from
   compilation. Optimize only where measurements justify it.

Use one baseline recipe and a temporary selection variation for this proof; a
recipe catalog is unnecessary. Status of each criterion is in the next section.

## First slice: route, evidence and gaps

Recorded 2026-10-24 on Linux x86_64 (cloud container, 4 vCPU, 15 GB RAM),
Rust 1.97.1 (`rust-toolchain.toml`, Rig's pin), Rig `55cd481a`, debug profile.

**Route: Rig's harness.** `rig-harness` + `rig-basics` + `rig-tui` already give
the loop, tools, sessions, compaction, the print/JSON front and the terminal
view, and Rig's launcher showed the generated shape to copy. Neither a
`rig-core` + `rig-agent` host nor an own loop was written. Rig's `rig` launcher
and `plugins.toml` are **not** used: the recipe is the one selection source.
Rig's login, `/reload` and `/plugins` are left out of the recipe because they
are a second, runtime selection path or a stored-key path.

**One Vivy plugin: `plugins/vivy-gateway`.** Rig's `--model` accepts only
catalog models, so a gateway's arbitrary model IDs need a plugin. It follows
Rig's own pattern (`Connector::provide` on a declared vendor, as in Rig's
`corpus/base/src/local_models.rs`): `gateway/<model>` is added to the catalog
and served by Rig's OpenAI wire with the baseline compatible-gateway quirks
(`Dialect::gateway`), on Chat Completions, at the configured base URL, sending
the model ID unchanged. Only the user's settings are read, so a cloned
project's `.rig/settings.toml` cannot redirect the key. The plugin must depend
on the recipe's Rig revision; a different one would add a second copy of the
Rig crates.

| Criterion | Status | Evidence |
| --- | --- | --- |
| Task, gateway, streaming, file edit + command, continuation | **Offline only** | Against a local scripted Chat Completions server (`scripts/mock_gateway.py`): one `-p` run streamed, called `shell`, wrote `hello.txt`, returned the tool result and exited 0; the request went to `/v1/chat/completions` with the bearer key and the model ID unchanged (`org/model-1`), offering `read`, `edit`, `write`, `shell`. A second process with `RIG_SESSION=<id>` continued the session (both user turns sent, model restored). **No real gateway has been called yet.** |
| Optional plugin removed by recipe | Passed | Without the `terminal` component, `cargo tree` of the generated package loses `rig-tui` and 62 other crates (ratatui, crossterm, syntect, …): 307 → 245 packages. On a terminal, the variant prints "this build has no front for an interactive terminal"; the baseline opens Rig's terminal view. Removing a `rig-basics` plugin type alone removes no crate. |
| Errors, cancellation, credentials | Passed offline | Recipe errors above; an unknown plugin type fails in rustc naming it. A gateway 401 is shown with the gateway's message (exit 1); a missing key names `RIG_SECRET_GATEWAY`; an unknown model suggests the configured one. SIGINT during a tool's `sleep 300` exits 130 and the `sleep` is killed. In the shell tool, `env \| grep -c RIG_SECRET_` printed 0; the test key appears in no session or workspace file. |
| Build times | Measured | Dependency download (`cargo fetch`, 513 packages) 34 s, separate from compiling. Cold compile of the baseline 161 s; warm `vivy build` with nothing changed under 1 s; after a one-line edit in `vivy-gateway` 4 s. Changing the selection (baseline → no `terminal`, sharing the target directory) 82 s, mostly re-compiling crates whose features changed. Target directory 2.9 GB; the debug binary is 613 MB. No optimization was attempted. |

Gaps and differences from Rig's launcher, by evidence so far:

- **Real gateway smoke is pending** the endpoint, model ID and key. Streaming
  and tool calls on that gateway are unverified until then.
- **Launcher-only behavior is absent:** `--resume` is the launcher's flag (use
  `RIG_SESSION=<id>` with the binary, or `/resume` in the terminal view), and
  build-failure rollback and `/reload` do not exist; change the recipe and run
  `vivy build` again.
- **The gateway model has no catalog facts** (context window, prices,
  reasoning), so compaction thresholds and cost reporting may not apply to it.
  Add settings for them when a real gateway shows the need.
- **Windows** has not been built or run; the inspected Rig launcher documents
  Linux and macOS only.

After that, use [PI-BASELINE.md](PI-BASELINE.md) to add capabilities in usable
slices. Focus research on session semantics, hook behavior, terminal reuse, MCP,
and JavaScript Code Mode where existing libraries may save substantial effort.
Rig's Steel/Scheme mode is not Pi's JavaScript Code Mode equivalent.

[rig-project]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/src/launcher/project.rs
[rig-harness]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/crates/rig-harness/README.md
