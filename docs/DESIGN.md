# Decisions and assembly direction

Recorded on 2026-10-24. This is an experiment brief, not an implemented API or a
production compatibility promise. The user's decisions are fixed below; the
runtime integration choice was resolved in favor of Rig's harness (see the
implementation sections below).

Updated 2026-10-25: **Hybrid architecture and first-stage boundary** below is the
current design. Dated implementation sections preserve earlier evidence; they
do not establish the newly designed capabilities. The owner has since reported
a successful manual test; that report does not supply a new automated test record.

The later **Collaborative Plan mode** section records the owner-approved Plan
addition and its verification. This does not expand the parked Goal/evolution work.

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
| Hybrid direct tools and Python workbench | Frequent coding tools remain direct; MCP and programmable composition live in Python. |
| Native delegation; optional Python orchestration later | Recipe-selected Rig wrapper provides single tasks, bounded parallel batches and chains. Python/apixis remains a later composition direction. See the dated subagent delivery record. |
| Prime-anchored automatic evolution | Use Prime's actual harness/proposal/history formats; add a narrow Vivy persona extension. |

The full Pi baseline is a coverage target, not the first delivery's acceptance
gate. Gateway-first access intentionally defers Pi's broad direct-provider and
OAuth surface. Embedding can initially use ordinary Rust library APIs; an
independently packaged SDK is unnecessary. Do not declare complete Pi parity
while those or other baseline rows remain incomplete.

## Hybrid architecture and first-stage boundary

This section is the implementation authority for the accepted miniature diagram.
The [research checkpoint](research/2026-10-25-hybrid-agent-workbench-decisions.md)
preserves evidence and discussion, including a mirrored copy in agent-vivy; it
is not another editable specification. The [Pi baseline](PI-BASELINE.md) remains
the single capability inventory. The original [owner sketch](research/assets/2026-10-25-owner-architecture.png)
is preserved as input, not a second configuration or literal storage schema.

**Choice:** keep the existing Rig harness and terminal, directly select useful
upstream plugins, and extend the existing Python worker for deferred external
capabilities. Rust owns execution and durable state; Python owns its execution
namespace and MCP clients. A lower-level agent rewrite would duplicate working
session/terminal behavior; a full Prime port would add unneeded daemon and factory
machinery. Reuse Rig's subagent lifecycle for the accepted recursive experiment.
The selected route minimizes owned code while retaining
recipe-selected compilation and Prime-shaped evolution.

**Core boundary (owner, 2026-10-25):** Rust/Rig owns Pi's coding core: the agent
loop, coding tools (read, edit, write, shell, search), sessions, compaction and
the TUI, including execution of recursively delegated agents. Python owns external
integration and workflow logic: MCP, optional memory backends and experimental
composition. Reuse the same Rust execution path when those workflows need agents.

```text
One YAML recipe -> generated Rust assembly -> one Rig host

Rig host: gateway/model calls, context assembly, TUI/CLI, sessions, cancellation
  Context <- fixed persona + evolving state + conversation + selected skills
  Model -> direct read/edit/write/shell/search/list
        -> python_repl -> MCP clients / Python functions / large-data processing
                           -> optional external memory or PEN, when specified
                           -> direct delegation / optional apixis flow
                                -> host bridge -> child Rig agent (same unit)
  Host <- correlated execution events and artifact references
  Task evidence -> automatic refinement -> checked Prime-shaped edits
                -> authoritative harness state -> future context
```

### Ownership and reuse

| Concern | Owner and first-stage implementation choice |
| --- | --- |
| Main agent loop, gateway, streaming, usage, cancellation | Existing Rust/Rig execution path; do not create a Python agent loop or direct credential path. |
| Terminal, print/JSON, sessions, compaction, commands and native hooks | Select existing Rig plugins. Add only demonstrated gaps; exact Pi semantics require checks. |
| Frequent coding tools | Current Rig/Vivy tools remain directly model-visible. Python access, when required, dispatches to the same implementation. |
| AGENTS.md, prompt templates, Markdown skill discovery/loading | Existing Rig/basics mechanisms; no second discovery catalog. |
| Python execution, MCP discovery/calls, data composition | Existing vivy-python worker. MCP only for servers that are not Python libraries, through the official Python MCP SDK; no MCP in Rust. |
| Agent delegation requested from Python | A small bridge to Rig's existing subagent lifecycle; host-owned model/gateway access, usage, limits and cancellation. No Python-owned agent loop or separate provider path. |
| Model calls requested from Python | A bounded request through the host's gateway, usage and cancellation path for a concrete task, without a second provider stack. |
| Evolution scheduling, validation, state/history commits | One host owner, using the normal model-call path. Python may prepare evidence or execute checks; it does not own another refinement scheduler/store. |
| Memory backends | MemPalace or memU, imported directly in the worker (delivery step 4), with one memory owner across forks. PEN is deferred. |

Reuse a plugin directly unless Vivy adds actual selection behavior or a required
product difference. A concept in this table does not require a separate crate,
service, registry or abstract interface. Keep current crate boundaries unless
an implementation demonstrates a concrete need to change them.

### Recursive delegation and optional graph orchestration

Implemented in the independent subagent branch on 2026-10-25 following the owner's
expanded Pi-example scope. `vivy_basics::subagents::VivySubagentsPlugin` wraps
`rig_basics::subagents::SubagentsPlugin`; native `task`, `message`, `wait`,
`SpawnSubagent`, reports, usage and session storage remain the execution path.
`task_parallel` aggregates at most eight tasks with four running children per
session, including follow-up turns. `task_chain` substitutes `{previous}` and stops
on the first unsuccessful step. Workflow reports are claimed before parent inbox
acceptance; the parent receives one aggregate. Full transcripts stay in Rig;
inline child answers are limited to 50 KiB with references to their child views.

Built-in scout/planner/reviewer/worker roles are overridden by home `agents/*.md`,
then trusted project `.rig/agents/*.md`. Invalid overrides disable the role.
Roles refresh on delegation; active and restored children keep their saved role
prompt. Tool selections cannot exceed parent access or the selected role.
Optional model/effort reuse native gateway resolution. Default depth is one;
explicit native `[subagents].max_depth` settings still work. Admission at capacity
queues root requests; nested spawns at capacity are refused to avoid deadlocks.

Children have independent conversations in the host process. Their optional
`cwd` is canonicalized inside the current project and applied through WorkDir,
without changing process cwd; project instructions, skills and trust stay shared.
No worker process, OS sandbox, cross-project context reload or automatic worktree
is implied. Scout/planner select read/search/list. Reviewer also has shell and
therefore can write; worker inherits parent tools. Existing trust is preserved;
no new approval or permission platform is introduced.

Native cancellation stops running children; pending launches and follow-ups are
cancelled too. Interrupted workflows are not replayed on resume. Completed child
conversations, roles and directories support native follow-up after resume. The
TUI exposes roles, progress, readable answers, child views and native usage;
unknown pricing remains unknown. See [delivery evidence](logs/2026-10-25-subagents.md).

The later direction remains Python/apixis orchestration over the same Rig
execution path. Each agent owns its context/workbench; Rust owns identities,
records and lifecycle. Graph state references host tasks. Before that later slice,
resolve the bidirectional bridge, Python 3.12+ requirement, graph abort propagation
and worker event-task cleanup described in the research record. Recipe assembly
continues to select compiled capabilities. Existing delivery gates still apply.

The [mirrored decision record, section 17](research/2026-10-25-hybrid-agent-workbench-decisions.md#17-recursive-rig-agents-with-pythonapixis-orchestration)
contains pinned Prime/Rig/apixis evidence. Section 17.7 records the latest scope;
the earlier full-bridge/graph effort range does not estimate this native
slice. Pi's subagent example is a behavior reference, not a claim of process,
permission or TypeScript extension compatibility.

### State homes and model-visible context

- Rig's session store owns conversation, branches and admitted tool exchanges.
  Supplemental execution records reference those identities instead of copying
  a second transcript. The concrete append hook is selected against the pinned
  Rig implementation before event code is added.
- Prime-shaped harness state owns learned prompt notes, memories, skill references
  and the persona extension. Use its `local`/`global` scope vocabulary; the host
  selects the owning session/profile directory, never model-supplied identity.
  The first implementation has one operator profile, not multi-tenancy. Here
  `local` means session/branch working state; `global` means that operator's
  cross-session learned state, not a tenant or project registry.
- ACTMEM is a contextual view of local working-state entries in that harness,
  not a new ACTMEM database/file or a second editable task board. Conversation
  compaction summarizes history; it does not independently own current task state.
- Explicit owner-authored persona/mission constraints have one selected source;
  evolving observations live in the harness. They cannot override fixed inputs.
- Markdown skill bodies retain their existing file owner. Harness skill entries
  reference those files or executable Python artifacts; do not copy a second
  authoritative skill body into a new registry. Validate references at use time.
- Large outputs and produced files are artifacts; records contain bounded previews
  and references with retention/truncation status. Python variables are ephemeral
  execution state, not automatically serialized durable data.
- External long-term memory is not a prerequisite for evolution. If later selected,
  define its data ownership and correction/deletion path first; do not mirror every
  harness entry into several competing memory authorities.

The host assembles context with a bounded budget and an identifiable state revision.
Keep stable constraints, relevant learned state, conversation and selected skill
content distinct. Tool descriptions/bodies load as needed. Record what actually
entered the model request; data stored only inside Python was not seen by the model.
Recompute the learned-state projection at the next task boundary. Ordinary reads
and context assembly do not secretly trigger refinement or network memory jobs.

### Workbench boundary and failure behavior

Retain stdlib CPython with top-level await unless a required behavior needs IPython.
The Prime-style interface does not by itself require Jupyter or a notebook server.
Extend the existing framed bridge with only the operations the first MCP task and
event collector need; do not start with three generic RPC services.

The host must service Python-to-host requests while awaiting a cell result, or
programmatic tool/model calls can deadlock. Carry parent call identity and the
same cancellation/deadline through the bridge. Keep protocol frames separate from
stdout/stderr. Attribute nested MCP calls to their cell even when the model sees
only a final summary. Direct arbitrary Python I/O remains outside detailed managed
call tracing; this worker is not a sandbox or complete side-effect audit.

MCP connection tasks need worker-owned lifecycle management. The current worker
cancels leftover asyncio tasks at cell end, so simply importing the SDK cannot
establish reusable sessions. Distinguish owned client tasks from unsupported
detached user work; close clients on reset/exit. The first MCP slice covers stdio
and Streamable HTTP against selected test servers, explicit configuration and
credentials, and truthful errors. Broad OAuth, service catalogs, resource UX and
all Pi MCP exposure modes remain baseline gaps, not hidden completion claims.

Reset namespace and MCP clients when switching sessions or moving to an incompatible
conversation branch; never leak future-branch variables into an earlier context.
Use a new kernel-instance ID and a visible reset notice. No automatic cell replay,
arbitrary object snapshots, daemon survival or detached tasks in this stage.

Local harness entries must also match the active branch. On fork/rewind, inherit
only a revision known to precede the fork; otherwise start with empty local state
and show a reset notice. Do not invent a historical snapshot system for the first
slice. Profile/global learning deliberately persists across conversation rewind;
undoing it uses explicit refinement rollback. Pending refinement captures its
session, branch and prior revision; discard results after an incompatible switch.

For newly managed Python/MCP bridge effects, acknowledge recorded intent before
dispatch and record the observed result. Missing completion means unknown, not
safely retryable. A recording failure prevents new managed bridge calls and is
surfaced; an effect already performed remains potentially unknown. This does not
claim a global admission guarantee for existing native tools or arbitrary Python I/O.
At the pinned Rig revision, `rig-ecs/src/session_log.rs` keeps `flush` crate-private
and reports write failure without stopping execution. Registering extra saved
records alone cannot meet this contract: verify the smallest acknowledged append
adapter, including failure before dispatch. Supplemental records may own nested
call evidence, never another copy of the conversation or a second execution engine.
Preserve artifacts before publishing durable references, and mark unavailable data
explicitly. Redact credentials in the collection path, including error payloads.
Do not retain unlimited streams or claim complete replay of unrecorded external state.

### Prime anchor and persona growth

Use Prime `5c898d1b27b109bd2d383c68d60a30536e39cc22`'s `HarnessState`,
`HarnessEntry`, proposal actions and before/after history as the schema baseline
(see research section 15 for source links). There is one supported subset, not a
parallel Vivy evolution format. Subagent/factory kinds do not require implementing
their runtimes: unsupported executable entries must remain inactive and report that
status. Unknown kinds must not be silently remapped or erased.

Start persona representation with existing entry kinds and explicit Vivy metadata;
only introduce a versioned new kind if a demonstrated requirement cannot fit.
Learned user/persona observations are scoped, sourced and correctable. Fixed owner
constraints stay outside automatic edits. This is an implementation default, not
a claim of existing Prime persona support or tested upstream interoperability.

After a task boundary, bounded automatic refinement may emit no change or small
edits. Exclude refinement outputs from fresh task evidence to avoid self-reinforcing
loops. Record the considered task/evidence identity in the same refinement
bookkeeping, including no-change outcomes, so restart or duplicate notifications
do not consume the same evidence again. Reconcile interrupted attempts with the
state/history commit; uncertain attempts are not silently retried. Explicit user
reconsideration may create a new linked attempt. No separate scheduler store is
needed. Evidence content is data, not authority to change the host's fixed rules.
Apply checks proportional to the content: sourced notes need scope/conflict checks;
executable helpers need isolated functional checks; measured-gain claims need actual
evaluation. Do not require manual approval for every ordinary refinement.

One host writer applies against the expected prior revision, preserving durable
history and rollback linkage. Use normal version checks, not species hash enforcement.
Define and verify crash reconciliation between state and history before shipping;
two successful independent file writes are not an atomic commit. Corrupt state is
reported and automatic writes stop; do not silently replace it with empty state.
Do not rewrite live context mid-task. Rollback restores learned state/artifact
selection, not external side effects or a Python heap. Refinement cannot rewrite
its evaluator, authority checks or Rust recipe; such changes use the normal build.

### Delivery slices and stopping points

Owner decision, 2026-10-25: finish Pi's coding core before adding anything new.
Do these in order; each is done only when its check passes.

1. **Real model facts for the gateway model.** Optional `[gateway]` settings for
   context window, max output, image input, reasoning levels and prices, plus
   the Rig dialect whose option mapping the gateway takes. Done when automatic
   compaction triggers before the limit, an image reaches the model, and
   `/effort` changes the request.
2. **Proof on real paths.** One real-gateway smoke; then Pi and vivy pointed at
   the same mock gateway, comparing what each sends: prompt, tool schemas, tool
   results and compaction requests. Done when every PI-BASELINE core row has a
   recorded result instead of a description.
3. **Session gaps closed with Rig plugins.** `session_picker` in the recipe for
   `/resume`, and a branch summary on rewind. Done when resume, fork, rewind and
   summary all work in the TUI.
4. **Python workbench for memory.** Import MemPalace or memU directly in the
   worker; MCP only for servers that are not Python. Clients live in a
   worker-level registry that survives the end of a cell; one JSONL debug log
   per worker; a pinned venv; one memory owner across forks. Done when memory is
   saved, a worker reset recovers it, and the log shows every call.

**Not now:** Prime evolution, persona, PEN, MCP in Rust, or more design
documents. Pick them up only after step 4 works.

**Owner update (2026-10-25): only Pi's spec.** Step 4 (MemPalace/memU memory)
is parked: Pi has no memory backend. After step 3 the work closes the gaps
between vivy and Pi's built-in coding-agent core, in slices: (A) context and
system-prompt files, (B) session flags and commands (`--no-session`,
`--fork`, `/clone`, `/import`, `/tree`), (C) CLI and model (`/thinking`,
`--export`, several gateway models), (D) project trust. MCP, Code Mode, RPC
and settings/themes are decided after these. PI-BASELINE's Result column
records each slice.

Status: slices A-D are implemented and checked offline (`compare_pi.py`,
`sessions_tui.py`); E is this record. The real-gateway smoke is still blocked
by the network policy.

**Step 1 evidence (2026-10-25, Linux, scripted gateway; `scripts/smoke.py`):**
the settings map onto Rig's `ModelSpec` (`context_window`, `max_output_tokens`,
`input.image`, `reasoning` as `Listed` levels, `pricing`). An `@pic.png` message
carried an `image_url` part; without `image_input` it carried text only. With
`dialect = "openrouter"`, `/effort high` made the next request carry
`reasoning: {"effort": "high"}`; Rig refuses every request option for an unnamed
dialect, so `reasoning` without a `dialect` is ignored with a warning and
`/effort` refuses up front instead of failing each call. In a 64k window with
~10k-token replies, the seventh turn sent Rig's in-place summary request first
("Compacted 9 messages … kept the newest 4"), then a shorter turn request; near
the limit `max_tokens` is clamped to the room left. The real gateway is still
unverified: this environment's network policy denies `openrouter.ai`.

**Step 2 status (2026-10-25):** the real-gateway smoke is still blocked by the
same network policy. The Pi comparison is done offline: `python3
scripts/compare_pi.py target/debug/vivy` runs ten scenarios on Pi (pinned
revision, from source) and vivy against one scripted gateway; PI-BASELINE's
Result column records the outcome for every core row.

**Step 3 evidence (2026-10-25, Linux, scripted gateway; `scripts/sessions_tui.py`):**
the recipe names `rig_basics::session_picker::SessionPickerPlugin` (Rig's
sessions plugins would add it anyway; the recipe now says so) and a new
`branch-summary` component, `vivy_basics::branch::BranchSummaryPlugin`. Rig's
rewind forks an agent just before a message and keeps the old one; nothing
summarized what it left behind. The plugin takes the messages the fork did not
copy, asks Rig's summarizer for a checkpoint of them as a request of its own
(as Pi does), in a turn of the fork's own, and adds the answer to the fork as a
user message in Pi's words ("The following is a summary of a branch that this
conversation came back from"). Driven on a pseudo-terminal: two turns, Alt+Up
twice and Enter forked before the second message, the summary request carried
exactly the two messages left behind, the summary and a notice showed in the
fork, and the fork's next turn sent the summary before the recalled message;
`/fork` and `/agents` listed three agents; `/new`, then the `/resume` picker,
listed and resumed the earlier session. With the summary request refused, the
failure was reported, nothing was inserted and the fork went on. Differences
from Pi: it always summarizes (Pi asks), and Rig's branches are agents of one
session rather than one tree file.

Before claiming each slice, demonstrate its real path and a representative failure:
cancel/reset without orphan clients; missing tool/auth failure without a fake success;
branch reset without state bleed; a nested call linked to the visible summary;
and a refinement applied on the next task, conflict rejected, and previous state
restored after restart. Also check recording failure before MCP dispatch and that
completed/no-change evidence is not automatically consumed twice after resume.
A bounded recorded input/mock server can cover destructive
or nondeterministic failures; it does not replace the chosen real gateway/MCP smoke.

No full Prime daemon, RLM swarm, marketplace, automatic dependency installer,
multi-backend memory framework or new custom TUI enters these slices. If Python
third-party dependencies are needed, pin an explicit environment and report missing
packages; removing the workbench component must remove its startup/dependency
requirement. This adds no claim that Python packages compile into the Rust binary.

### Architecture review record (2026-10-25)

The review targets this design and its direct implementation seams, not the health
of original Vivy's whole repository. Prevented design risks: duplicate transcript
and ACTMEM ownership; a second Python model loop; MCP tasks cancelled after each
cell; branch state bleed; deadlocked host callbacks; non-atomic state/history
updates; uncontrolled context growth; and importing dormant Prime/DIVA features.
These boundaries are design decisions, not fixes already implemented in runtime.

A fresh read-only reviewer identified two additional gaps now resolved here:
local harness state must follow branch ownership, and refinement must remember
consumed evidence even when it makes no edit. Source inspection also confirmed
that Rig's existing log notices do not provide bridge-effect admission; that
guarantee remains a focused implementation requirement, not an upstream claim.

Each new owned layer must name the delivered user behavior and explain why direct
Rig/SDK use is insufficient. Use before/after owned-code and dependency changes as
review evidence, not an LOC quota. Delete a superseded owner in the same delivery;
do not keep two mechanisms synchronized by convention. The acceptance task above
is the stop condition: unrequested infrastructure is not extra progress.

The exact Rig persistence hook, smallest framed bridge messages and state/history
commit procedure remain bounded implementation choices to verify against actual
APIs. PEN and an external memory provider remain unspecified and excluded. This
review cannot prove the absence of all architecture defects without implementation
and real-path tests.

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
- **Gateway model facts** are optional `[gateway]` settings (README); without
  them the model has no context window, so compaction never triggers, images
  are not sent, and `/effort` has no levels. Delivery step 1 records the checks.
- **Windows** has not been built or run; the inspected Rig launcher documents
  Linux and macOS only.

## Second slice: daily-use core

Recorded 2026-10-24, same machine and toolchain. `plugins/vivy-basics` adds
five plugin types, selected by three recipe components (`search-tools`,
`prompt-templates`, `session-commands`); the `terminal` component turns on its
`tui` feature.

| Pi feature | Here | Evidence (built agent, scripted gateway) |
| --- | --- | --- |
| `grep`, `find`, `ls` tools | Rig's existing `search` (contents, or names with `files`; `.gitignore`-aware) and `list` tools, registered per call like Rig's coding tools | Offered on every request; a scripted `search` call returned `notes.txt:1: hi there`. Not ported: Pi's `ignoreCase`/`literal` flags (a regex expresses both), context lines, a settable limit. |
| Prompt templates | `.md` files directly in the project's `.rig/prompts` and `RIG_HOME/prompts` become `/name`; Pi's frontmatter (`description`, `argument-hint`), argument splitting and substitutions ported with tests | `/review 'the API' a b` sent `Review the API; extra: a b`, titled `template review`; `/help` lists it. Project templates load without Pi's trust step, as Rig's project skills do. |
| `/session` | Name, ID, directory, model, message and tool-call counts; tokens and cost stay in Rig's `/usage` | Shown for a resumed session. |
| `/copy` | Last reply: OSC 52 plus system clipboard in the terminal view, system clipboard otherwise | In print mode here it failed clearly ("no clipboard here: it needs wl-copy, xclip, …"), as this container has none. The terminal-view path is unverified. |
| `/export [path]` | HTML by default (`session-<id>.html` in the agent's directory), JSONL for a `.jsonl` path | Both written for a resumed session; the HTML escapes content (unit-tested). |
| `/clone`, `/tree` | Rig's `/fork` (at the end, or at a message), `/agents` and rewind | Not added as duplicates; their semantics (forks are agents of one session, not new session files) differ from Pi's tree. |

Adding the crate rebuilt the baseline in 85 s with the shared target directory.

After that, use [PI-BASELINE.md](PI-BASELINE.md) to add capabilities in usable
slices. Focus research on session semantics, hook behavior, terminal reuse, MCP,
and JavaScript Code Mode where existing libraries may save substantial effort.
Rig's Steel/Scheme mode is not Pi's JavaScript Code Mode equivalent.

## Windows coding species and Python workbench

Implemented on Windows x64, Rust 1.97.1, using the same pinned Rig revision.
This section supersedes earlier Windows and launcher gaps; the dated Linux
measurements above remain historical evidence.

- `vivy` starts the already-built baseline agent, forwarding its I/O, flags and
  exit status. It uses Rig's description and session-switch protocol, session
  locks, saved working directories, and existing store for `/new` and `/resume`.
  Missing artifacts produce the explicit build command. There is no automatic
  build, reload, crash restart, installer, or new session index.
- A fresh agent selects the configured gateway model. An explicit CLI model or
  restored model wins. The recipe omits Rig's remembered-model DefaultsPlugin
  so an unrelated stored preference does not override the gateway configuration.
- The basics plugin's `shell` keeps Rig's arguments, working-directory placement,
  live output, limits, and spill files. On Windows it discovers Git for Windows
  or reads `[shell].executable`; it never selects the WSL `bash.exe` alias.
  Other platforms continue using Rig's shell implementation.
- `crates/vivy-process` is the shared process-ownership helper needed by shell
  and Python. On Windows it starts a suspended process, attaches it to a
  kill-on-close Job Object, then resumes its primary thread. The calling future
  retains cancellation ownership while I/O runs on another thread. This covers
  descendants, timeout, cancellation and cleanup without a Rig fork. Commands
  remove agent-only state and `RIG_SECRET_*`; Cargo also filters those secrets.
- The independently selectable `vivy-python` crate embeds a stdlib-only CPython
  worker and exposes `python_repl(code, timeout_secs?)`. Its namespace is owned
  by one active agent, supports top-level await, and survives normal calls and
  ordinary Python exceptions. It is not serialized. Timeout (default 60 seconds),
  cancellation, worker failure, exit, or session switching discards the namespace.
  Fresh-worker results state that previous process/session variables are absent.
- The worker captures Python, raw-fd and subprocess stdout/stderr separately
  from JSONL control messages. It drains earlier output before returning the
  trailing expression, bounds encoded result frames, and retains Rig's normal
  tool-result output limits. No automatic package installation, background tasks,
  subagent API, snapshots, replay or programmatic tool bridge was added.

### Native build and checks

The previous MSVC native-dependency failures were resolved using a short target
directory and disabling dev debug data, without replacing TLS or Rig:

```powershell
$env:CARGO_TARGET_DIR = 'C:\vt'
$env:CARGO_PROFILE_DEV_DEBUG = '0'
cargo test --workspace --locked
cargo test --manifest-path plugins/vivy-basics/Cargo.toml --locked
cargo test --manifest-path plugins/vivy-gateway/Cargo.toml --locked
cargo test --manifest-path plugins/vivy-python/Cargo.toml --locked
python -m unittest discover -s plugins/vivy-python/tests
cargo run -p vivy -- build recipes/coding-agent.yaml
python scripts/smoke.py C:\vt\debug\vivy.exe
```

Root workspace tests do not include the separate plugin workspaces. Formatting
is checked for the root and each plugin manifest. The local smoke uses only a
scripted loopback Chat Completions server and disposable directories.

Verified behavior:

- Builder tests cover recipe selection, portable generated paths, the target
  directory override, secret filtering and missing-artifact guidance.
- Process/shell tests cover exit status, a working directory containing spaces,
  timeout and termination of a process with descendants. The empty-frontmatter
  parsing regression is covered with LF and CRLF inputs.
- Python tests cover state and agent separation, await, exception recovery,
  timeout/reset, cancellation by dropping the future, raw/subprocess output,
  printed-output ordering and large Unicode result frames.
- `scripts/smoke.py` runs the native launcher and assembled species through a
  streamed shell task, native read/edit calls, saved-session resume, successive Python calls, a fresh
  Python namespace after process resume, JSON output and a gateway 401. It checks
  the outbound raw model ID, offered tools and absence of the test key in session
  files. Print-mode session switching returns 75 without replaying its prompt.
- A Windows pseudoterminal smoke exercised the actual Rig TUI, a model/tool task,
  `/session`, `/new`, `/resume <id>` and `/quit`. The harness's own PTY process
  creation was unavailable, so temporary pywinpty 3.1.0 tooling supplied the test
  terminal; it is not a product dependency.
- A temporary recipe without the Python component rebuilt successfully, and
  `cargo tree` contained no `vivy-python`. `vivy-process` correctly remained for
  shell execution. Temporary variants are not a second maintained recipe source.

Independent review identified a closed-output timeout branch and two Python
frame/ordering edge cases; these were corrected and checked. This is functional
terminal reuse, not a visual Pi clone. A real gateway smoke remains pending an
authorized endpoint, model and key. There is no claim of complete Pi parity or
new macOS/Linux validation for this Windows delivery.

## Collaborative Plan mode (2026-10-25)

The owner selected DSH-style collaboration semantics, adapted through Rig's
public plugin interfaces. `vivy-basics::plan::PlanPlugin` is selected by the
recipe, with terminal presentation behind the existing `tui` feature. It adds
no runtime, database, SDK, Goal scheduler or dependency on a corpus crate.
It is an explicit addition beyond the Pi baseline, not a claim of Pi parity.

One saved `PlanState` per agent owns the effective mode and transition revision.
Running-turn selections are transient until the next model request or turn end;
idle selections apply immediately. Request preparation injects a single planning
section after other edits and before compaction. The tool catalog stays unchanged,
and existing permissions remain authoritative. This is advisory collaboration,
not a read-only sandbox. Mode follows Rig's current saved-component restore/fork
semantics; transient selections and reviews are not copied or auto-approved.

`exit_plan_mode(plan)` explicitly submits a nonempty Markdown plan with a `#`
title. Ordinary answers do not submit. Rig's existing `Ask` displays the exact
submitted content, binds it to the originating call and mode revision, and accepts
approval or feedback. Approval returns through that same tool call and removes
guidance on the next model request, without delivering a duplicate task. Feedback
retains Plan. Dismissal stops the current turn; manual exit withdraws review.
An expired answer cannot switch mode, and a missing reviewer stops visibly rather
than repeatedly asking. The terminal's modal free-text `/plan off` answer routes
through the same command as the ordinary input. Restart requires resubmitting an
unfinished review, never replaying an approval.

The terminal derives `PLAN`, pending-transition and `PLAN · REVIEW` markers from
that state and live review. Existing Ask supplies scrolling, selection and free
text. A small tool renderer hides raw plan JSON during review and shows the plan
and result as readable scrollback after settlement. The call and Ask records own
the submitted content; no independent `.rig/plan.md` copy is created.

References inspected: Rig's [Plan example](https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/corpus/shared/src/plan_mode.rs)
and DSH's [collaborative Plan implementation](https://github.com/deepseek-ai/deepseek-harness/blob/22ddee3792402c3e70ec5f86a63c3e3e895a2f60/packages/plan/plan-mode/src/index.ts).
Rig supplies the loop, effects, saved state, commands, Ask and TUI. DSH is semantic
reference material only. Verification commands and real terminal results for this
delivery are recorded in [HANDOFF.md](HANDOFF.md); earlier dated results remain historical.

## Compact vivy-test terminal (2026-10-26)

The owner approved a Codex-inspired visual refresh, English UI, consistent
vivy-test branding and complete tool output with existing primary keybindings.
The local Codex checkout at `970111f6cb` is presentation reference material only;
no Codex code or runtime dependency is added. Rig remains pinned at `55cd481a`.

`vivy_basics::tui::VivyTuiPlugin`, behind the existing `tui` feature, composes
`TuiViewPlugin` with the native transcript, editor and question strip. A separate
status line below the input stays visible with a draft. Its fitting priorities
keep activity and Plan ahead of model and usage details. Public canvas hooks
replace ordinary input hints and supply an empty-input placeholder, leaving
question titles, focus, caret and completion overlays to Rig.

The welcome reuses Rig's unsaved metadata aside and renders a small vivy-test
title with this plugin's version, current model/reasoning and directory.
Display-width truncation follows terminal resizes and keeps the directory's
project-name suffix. Shortcut hints fit as complete items. A terminal-write adapter
changes only Rig's application-title prefix; prompt marks, other control
sequences and title-stack restoration retain their owner. Normal transcript
and error content are not subject to a global brand substitution.

Neutral tool surfaces reduce visual noise while preserving terminal-derived
input/user shades, failure and diff colors, and NO_COLOR. Existing coding
renderers retain their complete results with compact state markers. Local
shell/search wrappers have no upstream renderer: small looks are installed
only for those tools found in the assembled runtime, preserving live output,
full results and failure status rather than displaying raw argument JSON.

The recipe selects `VivyPromptPlugin` for the default identity in all fronts.
It reuses Rig's default coding guidance with only `You are rig` changed to
`You are vivy-test`, and preserves the empty-role condition so explicit prompts
remain authoritative. The terminal component selects the presentation plugin
and its Cargo feature; omitting it excludes rig-tui without a runtime enable
flag. The underlying .rig paths, protocol names and environment variables stay
compatible. The same Plan, subagent and session mechanisms remain in use.

[rig-project]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/src/launcher/project.rs
[rig-harness]: https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/crates/rig-harness/README.md
