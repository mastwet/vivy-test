# Research and decision snapshot: Rig, Vivy, Prime Agent, and the Python workbench

Date: 2026-10-25

Status: Research checkpoint with an accepted hybrid architecture, Prime-anchored automatic evolution and Vivy persona extension. Section 17.7 records the separately authorized native Rig subagent implementation in vivy-test; Python/apixis remains future work. This mirror does not authorize original Vivy runtime changes.

Update, 2026-10-27: DESIGN's two-harness and trimmed-workbench decisions supersede
the older evolution/orchestration scope below. Section 18 adds a read-only Go
Vivy migration assessment in vivy-test only; it is not mirrored to the original
checkout and does not authorize implementation or change the accepted scope.

Scope: Preserve this conversation's findings and latest direction in both repositories.

This report is intentionally saved identically under `docs/research/` in
`vivy-test` and `agent-vivy` at the owner's explicit request. It is a dated
handoff snapshot, not a second runtime configuration or a new architecture
contract. Subsequent experiment implementation belongs in `vivy-test`.
Nothing here authorizes changing the original Vivy repair work or its contracts.

## 1. Latest owner direction

The latest clarification takes precedence over the earlier proposal to move
every capability into Python:

- Keep Pi-style, frequently used coding tools directly available to the model.
- Reuse capabilities already supplied by Rig as selected plugins; wrap only
  where actual product behavior or recipe selection requires it.
- Put advanced, external, and long-tail capabilities into the Python workbench.
  MCP is an example of this deferred surface, not a reason to reproduce Prime's
  entire service catalog, OAuth UI, skill installer, or management platform.
- Carry forward Vivy's useful deferred-tool idea: discover detailed capabilities
  when needed instead of injecting all tool schemas into every model request.
- The workbench is preparation for the owner's PEN system. PEN's meaning,
  contracts, and requirements have not yet been explained. Do not invent them.
- The owner asked to preserve the research and subsequently accepted a miniature
  architecture diagram. Section 16 records the documentation/review checkpoint;
  runtime implementation requires a subsequent implementation task.
- Latest evolution decision: use Prime's concrete continual-harness formats and
  refinement lifecycle as the anchor, with automatic evolution and a focused
  Vivy persona-evolution addition. Section 15 supersedes earlier suggestions of
  a bespoke evolution format or mandatory manual review for every change.

Earlier accepted experiment constraints remain:

- Independent Rust experiment, one YAML recipe and one build-time composition
  path; gateway-first model access.
- Pi coding-agent is the functional reference, with incremental delivery.
  Equivalent Rust/Vivy extensions are wanted, not execution of existing Pi
  TypeScript extensions.
- More, Fast, Good, Frugal: reuse existing components, run useful tasks early,
  keep owned interfaces and total maintenance small.
- No species hash enforcement, standalone plugin SDK, universal Port catalog,
  provenance/conformance platform, or Studio lifecycle in the experiment.
- Do not turn the original Vivy checkout into an experiment dependency.
  Original Vivy repair is a separate effort.

## 2. Inspected baselines and evidence limits

| Project | Inspected revision or source | Interpretation |
| --- | --- | --- |
| vivy-test | Local main `2b03b44`, 2026-10-25 | Existing experiment; latest commit fixes UTF-8 terminal startup. |
| Rig | `55cd481a2421fd5da468b5bd6afc6ac876883599` | Pin used by the experiment; do not substitute historical Rig API assumptions. |
| Pi | `f321e5bdd26b3624bfb2ab182a334d0a5edb698a`, coding-agent 1.4.0 | Prior source inventory; see vivy-test's existing PI-BASELINE.md. |
| Original Vivy local main | `525d50a67152d6e9f5cc2eb469c889c6f26c21ab` | Older local checkout, not the current remote product baseline. |
| Original Vivy remote main | `9cd2801f8090723a652d3e835f72d83715b82121`, 2026-10-20 | Inspected separately through Git objects and official sources; root was not updated. |
| Prime Agent | `5c898d1b27b109bd2d383c68d60a30536e39cc22`, 2026-10-24 | Source, runtime protocol, prompts, and historical changes inspected. |

Research used source/docs/Git history inspection. It did not run a new product
test, benchmark, or model comparison. Existing experiment verification belongs
to its earlier implementation records. The owner reported that the project
passed their test; this report does not infer details of that test.

Source inspection used reference material already available in agent-vivy's
ignored workspace. Successors working on vivy-test should follow its own
reference-setup instructions, not clone or vendor the original Vivy project.

## 3. What Rig supplies, and what belongs to Vivy

At the inspected revision, distinguish the crates:

- `rig-core`: model/provider, completion/streaming, tool, conversation and
  related foundation contracts.
- `rig-agent` and `rig-memory`: agent execution, hooks, tool dispatch,
  conversation/state handling, compaction and related primitives.
- `rig-harness`, `rig-ecs`, `rig-basics`, `rig-tui`: the broader coding-agent
  and plugin environment already reused by vivy-test.
- `rig-rmcp`: an available MCP integration option. The owner's latest choice
  favors the advanced Python surface; do not add both MCP stacks by default.

Rig does not provide Vivy's exact Service.Run/Journal/Policy contract, typed
Port authority model, Grant semantics, or Generation evidence pipeline.
Its reusable coding-agent parts must not be described as full Vivy parity.

Earlier chat estimates were roughly 20-25/100 for rig-core alone, 55/100 for
direct reuse across the broad Vivy product, and 80-90/100 for the coding-terminal
slice using the full Rig ecosystem. These were subjective scope estimates,
not measured feature coverage, effort estimates, or acceptance results.
The later narrowed hybrid direction has a different denominator; do not use
these numbers to justify an implementation schedule.

Rig's upstream launcher documentation described Linux/macOS support. The
experiment's native Windows path is local adaptation with its own recorded
evidence; it must not be confused with an upstream platform guarantee.

## 4. Original Vivy and the remake donor branches

Vivy's useful composition concept is:

```text
Recipe -> selected Modules -> typed wiring -> compiled artifact
```

The original implementation also carries Port ownership/cardinality, trust,
Grants, source identity, lifecycle, conformance and Inspect obligations.
The experiment retains recipe-selected compilation without inheriting that
whole platform.

The local engine separates fixed-visible and deferred active tools and uses
Eino's `adk/middlewares/dynamictool/toolsearch`. This is evidence for the
owner's remembered deferred-tool mechanism, not a claim that the future
Python workbench already implements it.

The remake refs are not 16 accepted replacement implementations:

- 16 feature donor branches plus integration and archive refs were inspected.
- Feature areas include core fixes, providers, Notebook, reports, toolkits,
  BML, Recap, cognition, skill evolution, persona, workflow, ACP, tool
  parallelism, chat UI, miscellaneous changes, and warming.
- The integration plan says the published feature refs are raw donors and
  are not build/product-accepted. R10-R24 are blocked; R25 is a historical
  warming handoff.
- Feature branches share source base `202aac38`. The archive preserves the
  frozen history; integration is the planning/contract line.
- Code existing in the historical main tree does not prove the reorganized
  donor feature has been accepted against the repaired core.

Remote main's TODO also leaves material work open: toolkits acceptance,
unified memory/Recap integration, Eino child/DAG lifecycle gates, session
continuity, ACP integration/platform checks, and provider migration.
Do not present directory presence or donor branch counts as shipped capability.

## 5. Prime's actual evolution: correction to earlier discussion

An earlier answer treated retained bash/edit implementation files and stale
instruction text as evidence that those tools remain default model tools.
That was incorrect. Actual tool registration, the current core prompt, and
historical changes show the normal built-in model surface is `python_repl`.
SDK/custom tooling and old transcript rendering are separate concerns.

| Change | Evidence and meaning |
| --- | --- |
| 2026-05-08, PR #3, `4a7a55c9a` | Made ipython the only default-active tool; removed read/write/grep/find/ls; bash/edit remained explicitly selectable. The author's stated reason was matching the RLM-trained tool surface. |
| 2026-07-06, PR #323, `a20988132` | Removed bash/edit from the built-in tool registry. Shell work and editing continued through Python; some legacy implementations remained for custom use/replay. |
| 2026-10-01, PR #3211, `a3700282c` | Removed ineffective tool-selection flags. |
| Inspected current source | The core prompt says one tool, python_repl; session construction adds it to the configured tool set. |

Prime selected Python-first interaction early. The reviewed records do not
establish that a long-running hybrid design was benchmarked and found inferior.
Training-interface consistency is an explicit motive. Keeping large data
outside the main model context and enabling programmable composition are
documented RLM objectives. Reduced duplicate interfaces is an engineering
inference, not a published measured benefit.

"Python-only" here describes the normal model-facing action interface.
Prime's host, supervision, sessions and much of its management are Rust.

## 6. Prime's skills, MCP, and harness state are distinct

### Skills

Rust discovers Markdown skills and Python skill packages, parses metadata,
handles collisions, and advertises a compact inventory. Python skills have
SKILL.md, pyproject.toml, and an importable package. Host bootstrap uses uv
editable installs with incremental manifest checks and reports unavailable
packages. A Python skill does not need its own model tool schema.

Some core Python packages are only host-request wrappers: compact, goal,
refine, agent_message, agent_observe and rlm_heartbeat rely on Rust handlers.
Copying those Python packages alone does not transfer the capabilities.

### MCP

A generic Python MCP client offers catalog/connection discovery, live tool
search/description and calls. Adding a service is generally configuration plus
authentication, not a new per-service skill package.

Rust owns configuration, service/connection management and authentication.
Python resolves configuration through host requests, opens stdio or HTTP
connections, discovers schemas and performs calls. It reuses matching
connections and rebuilds changed ones, applies tool filters and timeouts, and
reports incomplete discovery as failure. Calls on the same connection are
serialized in the inspected implementation.

This division is evidence, not a requirement to copy Prime's entire
management surface into the experiment.

### Harness state

Harness entries store prompt notes, memories, skill callable references,
subagent descriptions and refinement history. A harness "skill" entry is not
installation of executable code. Python owns a file-backed harness-state
implementation; refinement scheduling and integration with the main run use
host facilities. Do not conflate this store with the session transcript.

## 7. RLM and long-context handling

RLM means Recursive Language Model. In this discussion it is a harness/execution
method around existing models, optionally supported by model training for that
environment. It is not a requirement for a new base-model architecture.

A characteristic flow is:

```text
Goal + reference to large data
  -> model writes Python to inspect/filter/split it
  -> intermediate data stays in variables/files
  -> selected evidence enters model context
  -> optional child-model analysis
  -> integration and further inspection
```

Assigning a large object to a Python variable does not make the model see its
contents. Printed output, displayed values, attachments and explicit messages
are the parts that can enter model context. Printing the entire object loses
the context-saving benefit.

Important boundaries:

- Prime's inspected ordinary text admission constructs a user message. Do not
  claim arbitrary pasted long text is automatically externalized.
- To avoid paying for a huge first model input, ingestion must store the material
  before that request and provide a reference plus necessary description.
- Modern conventional coding agents can also search files, retrieve excerpts,
  compact history and delegate. Python does not uniquely enable those behaviors.
- Exhaustive semantic analysis still requires reading the relevant material.
  Recursive calls can increase cost and latency; no benchmark here proves savings.
- A Python workbench is useful without recursive subagents. RLM is a possible
  later capability, not the current experiment's minimum requirement.

## 8. Proposed minimal workbench boundary

This section records assistant recommendations, not newly accepted APIs.

Keep the model-facing frequent coding tools and add the optional workbench:

```text
Model -> direct read/write/edit/shell tools -> selected Rig/Vivy plugins
      -> python_repl -> deferred capability discovery/calls + data composition
```

The useful workbench primitives are persistent execution, named intermediate
state, bounded inspection/output, external capability discovery and invocation,
managed long-running work, and file/artifact delivery.

- Reuse an existing tool implementation if Python also needs that capability.
  Do not maintain independent edit/shell semantics on both sides.
- Keep one host owner for the agent loop, gateway model calls, transcript,
  cancellation and execution lifecycle.
- Use ordinary Python functions and packages where sufficient. A universal
  Plugin interface or public SDK is not a prerequisite.
- Persistence between cells does not imply recovery of arbitrary Python objects
  after worker restart. Preserve authoritative data in files when necessary.
- A cell is not a transaction. Partial writes or remote effects can precede an
  exception; do not automatically replay failed cells.
- Keep structured protocol traffic separate from stdout/stderr. Capture output
  honestly, preserve call identity, and make cancellation/status observable.
- A trusted Python process with OS access is not a security sandbox.

The current experiment already has an optional stdlib-only Python worker,
top-level await, state across ordinary calls, bounded output and Windows process
ownership. Its DESIGN records namespace loss on timeout/cancellation/restart
and explicitly excludes automatic installs, background tasks, snapshots,
subagent APIs and programmatic tool bridges. Those exclusions must not be
reported as implemented merely because they were discussed today.

## 9. Recipe/build semantics and unresolved choices

The established Rust recipe continues to choose compiled components. Selecting
a Python workbench plugin at build time is compatible with that experiment.

A future recipe selecting Python capability packages would also need to package
those sources and dependencies. That can be one assembly/build delivery, but
it does not make interpreted Python packages native Rust code compiled into
one executable. The earlier suggestion to ship a fixed Python environment was
a proposal, not an accepted packaging decision.

Still unresolved, awaiting the owner:

- PEN's actual purpose, requirements and relationship to the workbench.
- Which external capabilities should enter the first deferred catalog.
- Whether advanced tool descriptions stay exclusively in Python or can be
  promoted into the direct model tool set.
- Minimum metadata/discovery interface and necessary host bridge operations.
- Packaging, dependencies, restart semantics and observability for advanced work.

No implementation order, migration, public API, or full-Python conversion is
authorized by this checkpoint. Preserve the working experiment while the owner
explains the next idea.

## 10. Source map

- [Rig pinned overview](https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/README.md)
- [Vivy Assembly contract at inspected remote main](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/docs/architecture/VIVY-ASSEMBLY.md)
- [Remake integration plan at inspected ref](https://github.com/ProjectViVy/agent-vivy/blob/0059b73138b3830fd3bfc1c041b9bc8353e93a33/docs/superpowers/plans/remake-20261019/index.md)
- [Vivy remote-main open board](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/docs/TODO.md)
- [Prime PR #3: explicit RLM training-interface rationale](https://github.com/PrimeIntellect-ai/prime-agent/pull/3)
- [Prime PR #323: removal of bash/edit built-in entries](https://github.com/PrimeIntellect-ai/prime-agent/pull/323)
- [Prime PR #3211: tool-selection flag removal](https://github.com/PrimeIntellect-ai/prime-agent/pull/3211)
- [Prime's RLM explanation](https://www.primeintellect.ai/blog/rlm)
- [Prime core model-facing interface](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/prompts/layers/core.md)
- [Prime workbench usage rules](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/prompts/layers/usage.md)
- [Prime REPL protocol](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/prime-agent-runtime/src/rlm/repl.md)
- [Prime generic MCP client](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/prime-agent-runtime/src/rlm/mcp.py)
- [Prime skill discovery](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/skills/discovery.rs)
- [Prime Python skill bootstrap](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/kernel/bootstrap/venv.rs)
- [Prime ordinary prompt admission](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/session_engine/admission.rs)

This delivery changes research documentation only. Validation is content review,
identical-copy comparison and Git whitespace checks. It does not establish new
runtime behavior or supersede either repository's implementation evidence.

## 11. Later discussion: embodied-agent workflows

The owner clarified that the Gemini example concerned a model operating a
physical robot, constructing a simulation environment, and learning execution.
The following are vendor-reported demonstrations, not independently reproduced
results. Keep the two experiments separate:

- Google's October 13, 2026 Argon announcement describes a human cornhole video
  being turned into environment coordinates, whole-body joint motions, and
  simulation code, then transferred to Apptronik's Apollo. Google reports seven
  consecutive successful shots. The announcement alone does not establish that
  this example trained a new neural controller, nor expose complete trial counts
  or controller implementation. Generating a simulation scene/program does not
  mean implementing a new physics engine.
- Google's robotics article describes a separate tabletop experiment: an RGB
  photo becomes a MuJoCo twin, tasks and a closed-loop trajectory generator;
  4,500+ randomized simulation episodes are exported for VLA fine-tuning. Google
  reports real-robot success increasing from 56% to 75%, and completion time
  decreasing from 35 to 24 seconds. The article does not fully specify the model,
  fine-tuning time, or who launched every training/deployment step.

Neither account establishes that Argon updated its own weights during the task.
Do not combine the VLA metrics with the Apollo demonstration, or claim a fully
unattended, reproducible training system from these announcements.

Architectural inference: a workbench can produce reusable executable capabilities
(scripts, optimized parameters, datasets, or model artifacts) through experiment
and feedback. This supports exploring the hybrid workbench; it does not establish
that every tool must move to Python, that RLM is required, or that a new plugin
platform is necessary. Rust/Rig can retain runtime ownership while Python runs
the relevant data-processing, simulation, optimization, or training programs.

Sources inspected on 2026-10-25:

- [Google announcement: Speeding up robotics development](https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/)
- [Google robotics article: Action improvement via simulation](https://developers.googleblog.com/gemini-4-argon-for-robotics/)

## 12. MCP SDK selection

The official SDK list inspected today places both Python and Rust in Tier 1.
This reflects protocol support and maintenance commitments, not a measured
comparison of production failure rates. Do not justify Python by claiming the
official Rust SDK is inherently immature or unreliable.

Python's current stable line is v2, with a breaking migration from v1; v1 is in
maintenance. Rust's rmcp provides client support, child-process stdio and
Streamable HTTP. Pin the chosen dependency and verify the actual target servers;
old tutorials and examples are not sufficient compatibility evidence.

Recommendation for this experiment: put MCP client connection ownership in the
Python workbench and avoid a duplicate Rust MCP connection manager. This fits
the deferred-tool boundary and lets large results remain in Python for local
processing. A Python client can call servers implemented in other languages.
This recommendation is not an implemented MCP bridge or a finalized public API.

Acceptance should exercise connection, tool discovery/calls, timeout,
cancellation and child-process cleanup, and reconnection after worker restart.
No comparative SDK reliability benchmark or new MCP integration test was run.

- [Official SDK tiers](https://modelcontextprotocol.io/docs/sdk)
- [Python v2 changes](https://github.com/modelcontextprotocol/python-sdk/blob/main/docs/whats-new.md)
- [Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)

## 13. Data collection and event-recording direction

The owner identified poor Journal modeling in original Vivy as a concern and
stated that future Vivy requires excellent event records. This discussion did
not audit the original Journal, approve its replacement, or change its canonical
contracts. The owner accepted preserving this discussion; the design below is
a proposed experiment direction, not a shipped feature or frozen schema.

### 13.1 What Prime actually records

At the inspected Prime revision:

- The session is append-only JSONL with typed entries, IDs, parent IDs and
  timestamps. It includes conversation messages, tool-start/progress entries,
  model changes, compaction and other session facts.
- Python results carry status, duration, stdout/stderr, exceptions, kernel
  restart information and selected shell/agent-message details. These are richer
  than a single combined output string, but are not an audit of every operation
  performed by arbitrary Python code.
- Worker recovery and orphan-process journals serve lifecycle recovery separately
  from the conversation record.
- Session telemetry explicitly emits counts, durations and categories, excluding
  prompt text, model output, tool arguments and results. It is not a training
  trajectory store.
- The inspected Python MCP call paths return results to Python without an
  explicit per-call persistent event write at those boundaries. This is a scoped
  source finding, not proof that no other instrumentation exists anywhere.

Primary code references at `5c898d1b27b109bd2d383c68d60a30536e39cc22`:

- [Session entries](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-types/src/session.rs)
- [Python tool results](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/tools/ipython.rs)
- [Recovery journals](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-daemon/src/journal.rs)
- [Telemetry contract](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/session_engine/telemetry.rs)
- [MCP dispatch](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/prime-agent-runtime/src/rlm/mcp.py)
- [MCP base client](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/prime-agent-runtime/src/rlm/mcp_base.py)

### 13.2 Minimum useful recording model

Keep the model-visible tool set small without making execution records coarse.
Preserve the chain of observation, action, environment response and evaluation:

```text
User goal
  -> model request (actual visible context and configuration)
  -> model response / native tool or Python cell
       -> nested MCP call(s), results, errors
       -> cell output and produced artifacts
  -> subsequent model observations
  -> test result or human feedback, with its source
```

Distinguish data available inside Python from data actually shown to the model.
A large MCP response reduced to a short summary must not later be treated as if
the model saw the full response. Record effective prompts, tool definitions and
settings through durable references or snapshots sufficient to reconstruct the
model-visible request, subject to declared retention and redaction.

Reuse Rig's existing session persistence for conversations. Supplement it with
internal workbench events linked to existing message/tool-call IDs; do not make
a second authority for conversation state. Let Python emit structured events
over the existing bridge and let the Rust host own correlation and persistence.
Choose the concrete storage hook only after inspecting the pinned implementation.

Proposed automatic collection boundaries:

| Boundary | Minimum facts |
| --- | --- |
| Model request/response | Actual input references, model/settings, output, usage, error |
| Native tool / Python cell | Call identity, arguments/code, start/end, status, duration |
| MCP call inside Python | Parent cell, server/tool identity, arguments, result, error |
| Workbench lifecycle | Start, exit, reset, new kernel-instance identity |
| Evaluation | Test or human feedback, source, associated task/run |

A previously suggested candidate execution-event envelope is `version`, `event_id`, `session_id`, `run_id`,
`parent_id`, `operation_id`, `seq`, `timestamp`, `kind`, and `payload`. This is
illustrative, not a required SDK or an accepted replacement for Prime's harness
and refinement formats. Section 15 governs the evolution-format decision.
Reuse existing identifiers where possible.
The host sequence records receipt order; parent/operation links express causality
and pair start/end events. Timestamps alone do not order concurrent causes.

Collection belongs in runtime/adapters, not in model-authored voluntary logging.
Do not instrument every Python function or variable assignment. Arbitrary direct
HTTP/filesystem calls that bypass managed wrappers are outside detailed tracing;
cell code and captured output do not prove a complete side-effect audit.

Keep events small. Store large results, images, patches, datasets and model files
as referenced artifacts. Separate model-context truncation from storage retention;
mark truncation, omitted/redacted content and missing artifacts explicitly. Do not
claim full reproducibility when relevant data was not retained. Redact credentials
at collection boundaries and keep collected data local by default.

### 13.3 Failure semantics and training-data boundaries

- Persist an operation's start before a managed side effect. A start without a
  terminal record means its outcome is unknown, not necessarily failed. Surface
  persistence failures; do not claim durable recording when a write failed.
- Never automatically repeat a potentially effectful operation just because its
  completion record is absent. Local logging cannot guarantee exactly-once remote
  execution across a crash.
- Restoring a transcript does not restore Python memory. Record kernel resets,
  preserve necessary data as explicit artifacts, and do not add arbitrary object
  snapshots for this first experiment.
- Keep model declarations, process exit status, test outcomes and human acceptance
  distinct. They are different evidence, not one undifferentiated success flag.
- Derive training/evaluation datasets later from retained trajectories and sourced
  labels. Do not make a training-data schema the runtime's primary record format.
- No event bus, standalone collection service, general event-sourcing framework,
  or new Journal platform is required by this proposal.

### 13.4 Current implementation and next evidence

In vivy-test, `plugins/vivy-python/src/worker.py` already emits separate stdout,
stderr, result, error and done frames. The Rust receiver in
`plugins/vivy-python/src/lib.rs` largely combines text into tool output. The
existing `/export` JSONL in `plugins/vivy-basics/src/session.rs` exports conversation
entries; it does not establish detailed internal workbench tracing.

The inspected Rig harness has session persistence (`rig-ecs/src/session_log.rs`
and `rig-harness/src/host/session.rs`). Before implementing, verify the exact pin,
extension seam, durability and failure behavior instead of duplicating that path.

Future acceptance should show one run containing a native call, a Python cell,
nested MCP success/failure, a large artifact and a worker reset, all correctly
correlated; it should also show an interrupted operation remaining unknown and
the precise subset of data exposed to the model. These checks have not been run.

This update records research only. The original Vivy repair, PEN requirements,
MCP integration, event APIs and implementation scheduling remain separate work.

## 14. Correction: original Vivy already has a DIVA evolution path

The subsequent inspection corrected an incomplete comparison with old Agent
Diva: original Vivy's remote main `9cd2801f8090723a652d3e835f72d83715b82121`
already contains a real local Skill proposal bridge. Its locked Laputa source
is `0e0027c5f60ad67614d3613c2d5e21d567c4b412`. The legacy Evolution page using
demo data does not mean that all backend evolution is a demo.

The inspected path is:

```text
Committed primary-run terminal activity
  -> ObserverHost capture with stable IDs and receipts
  -> input watermark / trigger policy / busy-state checks
  -> compiled diva/v1 strategy
  -> collect -> prepare -> reconcile -> reflect -> effects -> finish
  -> typed memory/persona/Skill/etc. effects, each with its own authority
  -> local Skill revision + Review Center
  -> human approval -> apply -> later reads use the changed Skill
```

Capture excludes workflow/inference output and the cognitive supervisor's own
bookkeeping runs, avoiding their reuse as fresh conversation evidence. DIVA
selects an enabled user-owned Skill before proposing an update to the same
identity. The host retains source evidence, rationale, operation and approval
identity; rejection, expiry, cancellation, stale revisions and application
failures have durable outcomes. Restart reconciliation handles pending reviews
and approved-but-unapplied revisions. Approval timeout never grants consent.

These are source findings, not a new end-to-end acceptance result. The remake
R18 skill-evolution lane remains blocked for re-intake against the repaired core.
Its plan calls for semantic revision checks under the no-hash contract, local
Skill evolution by default, and EvoMap experimental/default-off. Generic strategy
composition is separate. Historical implementation does not prove current repair
acceptance, and a reviewed modification does not prove improved task performance.

Sources:

- [Vivy cognitive capture](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/cognitive_service.go)
- [Vivy strategy binding](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/cognitive_binding.go)
- [Vivy Skill proposal bridge](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/cognitive_skill_proposal.go)
- [Pinned DIVA strategy](https://github.com/ProjectViVy/laputa/blob/0e0027c5f60ad67614d3613c2d5e21d567c4b412/laputa/evolution/diva/strategy.go)
- [Remake R18](https://github.com/ProjectViVy/agent-vivy/blob/979a10c1/docs/superpowers/plans/remake-20261019/lanes/R18-skill-evolution.md)

## 15. Accepted direction: Prime formats first, automatic evolution, Vivy persona

The owner explicitly clarified the anchor: use Prime's format, then add Vivy's
distinctive persona evolution. This is more specific than merely borrowing a
Prime-like philosophy. Do not design a new evolution format and label it similar
to Prime, or port DIVA's full workflow and approval platform into the experiment.

### 15.1 Concrete upstream baseline

Start from Prime `5c898d1b27b109bd2d383c68d60a30536e39cc22`:

- `harness/harness_state.json`: `schema`, `entries` keyed by kind then ID, and
  `refinements`. Existing kinds are `prompt`, `memory`, `skill`, `subagent`, and
  `factory`. Their presence in the format does not require implementing every
  runtime capability in the first experiment.
- `HarnessEntry`: preserve the upstream vocabulary, including `id`, `kind`,
  `title`, `content`, `path`, `scope`, `reference`, `arguments`, `metadata`,
  `source`, `created_at`, `updated_at`, and `version`.
- Refinement proposals: `summary`, `rationale`, `expectedOutcome`, and `edits`;
  edits use `create`, `update`, or `delete` over the selected entry kinds.
- Refinement history: `refinement_history.jsonl`, applied before/after records,
  scope and rollback linkage. Keep expected benefit distinct from measured
  evaluation; Prime's event `outcome` can contain the proposed expected outcome.
- Upstream scopes are `local` and `global`. Earlier suggestions of additional
  project/persona scopes are not a frozen replacement enum. Resolve ownership
  and directory mapping without casually changing the baseline format.

Inspect actual readers/writers before copying a schema. Preserve supported
semantics and explicitly document any extension or unsupported behavior. This
decision does not promise untested import/export compatibility with upstream,
nor require replacing Rig's entire session store with Prime's implementation.
It anchors evolution state/proposals/history; execution events must link to that
state without creating a competing source of truth.

### 15.2 Automatic lifecycle and proportionate checks

Use one small automatic lifecycle:

```text
New task evidence -> decide whether refinement is useful -> small typed edits
  -> checks appropriate to the changed content -> apply at a task boundary
  -> retain history -> observe later outcomes and support rollback
```

Automatic evolution is the desired normal behavior, not a mandatory approval
queue. Budget/cooldown and a legitimate no-change result keep it bounded. The
exact trigger thresholds remain implementation choices. Do not promote transient
task state into global memory, and keep scope assignment under host ownership.

Checks match the modification:

- A sourced preference or declarative lesson needs evidence, scope and conflict
  checks, not a full benchmark.
- An executable Python helper needs relevant isolated functional checks before
  automatic adoption. Reusing Prime's skill-reference format does not itself
  install or validate Python code; artifact creation is a separate operation.
- A claimed performance improvement needs actual evaluation evidence. Do not
  claim measured gains merely because a change applied successfully.

The earlier suggestion to begin with manual-only refinement and require full
baseline/candidate evaluation for every edit is superseded by this direction.
This is not authorization to bypass existing original-Vivy approval contracts;
the new policy belongs to the independent experiment.

### 15.3 Focused persona addition

Add persona growth to the same refinement lifecycle rather than building another
evolution engine. Evolving observations about the user, expression preferences,
relationship experience and self-understanding should remain evidence-linked,
correctable and versioned. Explicit owner-defined identity constraints and
mission remain fixed inputs unless the owner changes them.

Prime has no native `persona` kind at the inspected revision. The representation
is not yet frozen: inspect whether existing entry metadata/reference fields are
sufficient; if a distinct kind is needed, make it an explicit versioned Vivy
extension and update validation/readers accordingly. Do not assume upstream
readers preserve unknown kinds. Keep the new surface limited to persona needs.

Rust plugin and recipe changes still require rebuilding and testing the selected
artifact. Automatic behavioral evolution does not silently replace build-time
species composition. Model-weight training remains outside the initial scope.

### 15.4 Delivery boundary and references

Implement later in vivy-test, preserving the working coding core and Python
workbench. Do not alter original Vivy's repair lane, introduce a standalone SDK,
general evolution platform, new approval center, or species hash enforcement.
The owner requested a durable decision record, not implementation in this turn.

- [Prime harness formats and storage](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/refinement/mod.rs)
- [Prime proposal schema and application](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/refinement/planner.rs)
- [Prime refinement execution](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/session_engine/refine.rs)
- [Existing DIVA persona-effect boundary](https://github.com/ProjectViVy/laputa/blob/0e0027c5f60ad67614d3613c2d5e21d567c4b412/laputa/evolution/effects.go)

Verification for this documentation update: inspect the source-backed claims,
review the diff, compare both saved copies, and run Git whitespace checks. No
runtime behavior or new automatic-evolution capability is delivered here.

## 16. Accepted diagram and architecture review checkpoint

The owner approved documenting the miniature architecture and requested a final
review focused on hidden architectural risk and low-value code growth. The
implementation authority is `vivy-test/docs/DESIGN.md`, section "Hybrid architecture
and first-stage boundary". The original sketch is preserved in vivy-test at
`docs/research/assets/2026-10-25-owner-architecture.png`. This mirrored research
report records the decision; it is not a second specification to keep synchronized.

The accepted shape is one recipe-built Rig host with direct coding tools and the
existing optional Python worker. Rust retains the main model/gateway path, sessions,
terminal, context and refinement commits. Python supplies deferred MCP/data work
and future consumers when specified. Rig skills remain file-backed; Prime-shaped
harness state owns learned entries. ACTMEM is a view of local working state, not
another memory service. Python variables are ephemeral. External memory and PEN
remain unspecified and are not prerequisites for this experiment.

Three useful slices set the boundary: preserve/verify the Pi-style coding core;
deliver one deferred MCP task with lifecycle and correlated evidence; then add
bounded automatic Prime-shaped refinement and persona growth. The full Pi inventory
remains in `vivy-test/docs/PI-BASELINE.md`. A postponed capability is a gap, not
parity. There is no new SDK, Port catalog, custom TUI, daemon, generic RPC platform,
automatic package installer, model training or complete Prime port in these slices.

The review inspected the current workbench seam: `worker.py` cancels remaining
asyncio tasks after each cell. Persistent MCP clients therefore need deliberate
lifecycle ownership. The design also requires branch/reset handling, host callback
dispatch while awaiting Python, bounded context and artifacts, and a single writer
with crash reconciliation for harness state/history. Arbitrary Python I/O is not
a sandboxed or completely traced managed effect. These are design constraints,
not runtime fixes delivered by this document update.

Fresh independent review found two gaps, resolved in the design: local learned
state must follow the conversation branch (compatible revision or explicit reset),
and refinement bookkeeping must remember considered evidence, including no-change
outcomes, to prevent duplicate consumption after resume. Shared profile learning
persists across rewind and uses explicit rollback. The pinned Rig log reports write
failure without stopping effects; the new Python/MCP bridge needs an acknowledged
record-before-dispatch seam. This is not a claim of global fail-closed native tools,
nor a mandate to replace Rig's persistence with a second Journal.

Every new owned layer must deliver a concrete behavior that direct Rig/SDK reuse
cannot supply. Review owned-code/dependency changes and remove superseded owners;
do not set a line-count quota or reward additional infrastructure. Known remaining
implementation choices include the Rig append hook, minimal bridge frames and the
state/history commit procedure. Verify those with actual API and failure-path
evidence before claiming the corresponding slice complete.

Verification for this checkpoint: fresh read-only architecture review, document
consistency and local-link checks, original sketch byte comparison, identical
research copies and `git diff --check`. No runtime code, original-Vivy product
contract or repair-lane behavior changes. No runtime CI is needed for this
documentation-only checkpoint; implementation claims still require real-path tests.

## 17. Recursive Rig agents with Python/apixis orchestration

Date: 2026-10-25. Status: accepted architecture direction for a minimal experiment;
not an implemented subagent capability or a completed implementation plan. The
owner explicitly requested the same decision record in both repositories.

Latest scope: section 17.7 supersedes section 17.6's single-child acceptance.
Python/apixis remains the future direction; the native delivery adds Pi-example
roles, batches and chains, with default depth one and no graph dependency.

### 17.1 Clarification and decision

The owner corrected an intermediate interpretation: Python should orchestrate
Rig subagents, not run a separate Python agent engine. The earlier question about
where a Python-owned agent loop sends model requests is therefore superseded.

One YAML recipe compiles an agent unit with selected capabilities. A subagent is
another instance of that same Rig harness, with its own task, conversation and,
when used, Python execution namespace. It may have a different model or a narrower
tool selection. Recursive delegation reuses the implementation; it does not clone
the repository, copy the parent's complete context, or launch another terminal.

```text
Any Rig agent
  -> its Python workbench
       -> simple delegation, or optional apixis flow
            -> correlated Python-to-host requests
                 -> Rust/Rig creates, runs, waits for and stops subagents
                      -> child Rig agent with its own context/workbench
                           -> may delegate again within host limits
  <- bounded results, task identities and artifact references

Rust-owned execution state -> existing TUI status and agent views
```

Recursion and graph orchestration are separate capabilities. Recursive agents can
delegate without a graph. apixis becomes useful for parallel investigations,
dependency ordering, joins, review and conditional rework; it is not mandatory
for every delegation. Treat it as an optional Python library, not a second global
agent runtime or a replacement for recipe-driven Rust assembly.

RLM additionally treats large context as data in the workbench: code can select,
transform and delegate bounded pieces. Recursive spawning alone does not establish
the complete RLM context-processing behavior.

### 17.2 Ownership and implementation boundaries

- Reuse Rig's optional `rig_basics::subagents::SubagentsPlugin` and its
  `SpawnSubagent` execution path. Native delegation and Python-originated
  delegation must converge on that same host lifecycle.
- Rust/Rig owns agent identities, conversations, model/gateway access, credentials,
  usage, admission limits, cancellation and authoritative execution records.
  Python owns orchestration code and invocation-local flow state; graph state
  references host task IDs rather than maintaining a competing task registry.
- A small bidirectional bridge needs create, wait/collect and cancel behavior.
  Operation names, wire frames and the exact Rig adapter are not frozen by this
  record. Service host requests while a Python cell awaits results, preserving
  parent identity and cancellation; do not block the host event loop or re-enter
  the same locked Python worker for a child.
- Separate each agent's workbench state. Keep graph state small: task handles,
  summaries and artifact references, not copied transcripts, large datasets or
  live MCP clients. Actual model-visible context and internal Python data remain
  distinct.
- Project status from host state into the existing TUI. A Python orchestration
  interface does not justify an invisible backend-only feature or a second TUI.
- Bound nesting, concurrent work and execution budgets at the host. Exact limits
  belong in the later implementation plan; the graph's step counter is not a
  token, cost or agent-concurrency limit.
- The initial acceptance case is foreground, read-only research. Process survival,
  durable graph restoration, arbitrary namespace snapshots and coordinated
  parallel code edits are not part of that first case. This records a candidate
  next experiment without silently reordering the existing delivery gates.

Context/tool selection and Python process separation are not an OS sandbox.
Rig subagents at the inspected pin run within the host; do not claim each has
Prime's worker-process isolation. Arbitrary Python execution can access resources
outside a managed tool surface, so a tool allowlist is not a complete permission
boundary. Original Vivy's repair and Go/Eino contracts remain outside this design.

### 17.3 What Prime actually does

At Prime revision `5c898d1b27b109bd2d383c68d60a30536e39cc22`, Python's
`rlm.spawn()` sends `host_request("rlm.run", ...)`. Rust's `RlmHostBridge`
validates and routes the request to the child-session host;
`SupervisorChildSessions` creates and supervises child workers. Python factory
code combines `rlm.spawn`, `rlm.collect` and `rlm.delete_subagent` into workflows.
Thus the accepted split is closely related to Prime's actual architecture:
Python composes work while the host executes agents.

Prime's factory is its own state-machine/DAG implementation, not apixis. Its
executor registry lives in kernel memory and does not survive kernel restart;
supervisor-owned children can survive that kernel restart. Do not copy those
continuity promises into the first vivy-test experiment. Prime also implements
daemon continuity, worker supervision, scheduling, messaging, model adapters and
an extensive terminal product. A small model-visible Python tool surface does
not remove those product costs.

### 17.4 apixis fit and concrete constraints

Inspected apixis revision: `411c0e22e9f46520e407a8b7fc18116093055a73`.
Source inspection found Command-driven routing, concurrent batches, ordered
result merging, conflicting-field checks, node timeouts, streaming, interruption
hooks and graph/context ownership checks. No integration run or comparative
performance benchmark was performed; "advanced" is not a verified rating.

- Requires Python 3.12+, whereas the current workbench accepts 3.11+; select an
  explicit compatible optional environment before integration.
- `abort` resolves the graph invocation at its checkpoint, but an already-started
  node may continue running. Stop host-owned subagents explicitly and account for
  effects already performed; discarding graph results is not cancellation.
- Snapshots deep-copy state and restoration belongs to the same compiled graph
  identity. The API does not by itself supply cross-process durable recovery or
  rollback of external effects.
- Default snapshots can retain large copied state; store references and bounded
  summaries. Graph step limits count node batches, including a parallel batch as
  one step, rather than model or financial budgets.
- The existing worker cancels outstanding asyncio tasks after each cell. Start
  with an awaited foreground flow and deliberately handle the event runtime's
  task lifecycle; do not assume graph/event/MCP background tasks survive cells.

### 17.5 First acceptance case, scale and evidence

Suggested minimal acceptance: a parent uses apixis to run two research subagents
in parallel; one delegates a smaller task to a grandchild; results join into one
report. Verify distinct child contexts, visible TUI states, correlated results,
representative child failure and cancellation of the related work when the parent
is stopped. Admission at a configured depth/concurrency limit must be observable.
Use scripted model responses for deterministic checks, then a real-gateway smoke.
This is a proposed acceptance case, not a claim it has passed.

Inspection counted about 1,770 physical lines in Rig's subagent implementation and
895 in its dedicated test file, including comments and blanks. Reuse those
capabilities; do not reproduce them as Vivy-owned infrastructure. The discussion's
2,000-5,000 additional owned production-line range was an unvalidated planning
estimate for the bounded foreground integration, excluding tests and scripts. It
is not a quota, schedule or estimate for complete Pi parity. The bridge, lifecycle
and acknowledged recording seam need a focused feasibility check before a firm
implementation estimate.

Pi coverage remains in `vivy-test/docs/PI-BASELINE.md`. Recursive subagents and
graph orchestration are additions beyond Pi's default coding core at the inspected
revision; they do not complete its MCP, Code Mode, RPC, terminal-extension or
session semantics. Gateway-first and Rust functional-equivalence decisions remain.

At recording time, vivy-test main `c7ca8c9` and the separate Plan worktree commit
`b43d448` did not select `SubagentsPlugin` and had no Python subagent bridge or
apixis integration. No runtime capability is added by this documentation update.
The implementation authority stays in vivy-test's DESIGN; this identical research
snapshot in agent-vivy is for handoff only. Validation for this record consists
of source/diff review, matching mirror bytes and Git whitespace checks, not
runtime or product CI.

### 17.6 Owner simplification: Pi-like delegation first

The owner subsequently requested a simpler first iteration: delegation comparable
to Pi's subagent example, without complex orchestration or making a new gate
platform a prerequisite. This supersedes the earlier parallel-children/grandchild
acceptance case in sections 17.2 and 17.5, not the future Python/apixis direction.

The minimal implementation choice is direct reuse of Rig's native subagent plugin:
assign a self-contained task, run it with independent context, and receive its
result. Use existing `task`, `message`, `wait` and terminal views. A Python bridge,
apixis dependency, parallel joins, automatic review/rework, daemon continuity and
graph restoration are not first-iteration requirements. No new loop or registry.

First acceptance: one foreground read-only research child, visible progress and
result, a representative failure, and parent cancellation reaching the child.
Use Rig's existing `SubagentPolicy.max_depth = 1` in the initial recipe to prevent
grandchild delegation. The one-child acceptance scenario does not imply a new
global concurrency enforcement mechanism. Select appropriate read-only tools for
the acceptance case; a prompt alone does not enforce read-only behavior.

Keep existing trust/approval and failure/cancellation behavior. Rig supplies depth
checks and approval extension points, but their presence is not proof that every
tool has a configured permission policy. Do not construct a new authorization
framework to unblock this small slice, and do not claim OS sandboxing. If later
requirements need stronger admission controls, add the specific missing control.

Pi's pinned subagent extension example supports single, parallel and chain modes,
with independent Pi subprocesses and streamed progress. Only basic delegation is
the initial reference; parallel/chain modes and exact process/UI/API compatibility
are not first-delivery obligations. It is an example extension, not Pi's default
built-in agent mode. Source: [Pi subagent example](https://github.com/earendil-works/pi/blob/f321e5bdd26b3624bfb2ab182a334d0a5edb698a/packages/coding-agent/examples/extensions/subagent/README.md).

This scope removes Python/Rust bridging and graph lifecycle from the initial
integration, so the earlier 2,000-5,000-line estimate for the larger slice must not
be reused as an estimate for this one. Actual implementation remains pending;
this update changes documentation only and preserves the prior delivery gates.

Pinned source references:

- [Rig subagent plugin](https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/plugins/rig-basics/src/subagents.rs)
- [Rig spawning and inheritance](https://github.com/0xPlaygrounds/rig/blob/55cd481a2421fd5da468b5bd6afc6ac876883599/plugins/rig-basics/src/subagents/spawn.rs)
- [Prime Python spawn bridge](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/prime-agent-runtime/src/rlm/__init__.py)
- [Prime Rust host dispatch](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-core/src/session_engine/rlm_host.rs)
- [Prime child supervisor](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/crates/pa-daemon/src/rlm_children/host.rs)
- [Prime factory ownership](https://github.com/PrimeIntellect-ai/prime-agent/blob/5c898d1b27b109bd2d383c68d60a30536e39cc22/prime-agent-runtime/src/rlm/factory.py)
- [apixis graph lifecycle](https://github.com/JJJJSTIYYYY/apixis/blob/411c0e22e9f46520e407a8b7fc18116093055a73/docs/core/graph/README.md)
- [apixis state and concurrency](https://github.com/JJJJSTIYYYY/apixis/blob/411c0e22e9f46520e407a8b7fc18116093055a73/docs/core/graph/state.md)
- [apixis Python requirement](https://github.com/JJJJSTIYYYY/apixis/blob/411c0e22e9f46520e407a8b7fc18116093055a73/pyproject.toml)

### 17.7 Owner-approved native delegation delivery

The owner expanded the initial slice to Pi-example-level delegation, approved a
Rig-native Vivy plugin, and explicitly requested implementation. Independent
`vivy-test` branch `feat/subagents`, based on `5477eee`, owns this runtime change;
`agent-vivy` receives only this matching archive. No merge or push is part of this
delivery, and the separate Plan branch remains untouched.

Simplest viable choice: wrap Rig's pinned SubagentsPlugin and SpawnSubagent in a
recipe-selected component, retaining its task/message/wait, conversations,
reports, usage and persistence. Add only role snapshots, agent-local cwd,
bounded admission and finite parallel/chain adapters. Python orchestration would
require a bidirectional bridge and abort ownership; separate worker processes
would add transport and lifecycle machinery. Neither is needed for the approved
same-process, same-project experiment. The cost is shared process failure scope
and shared workspace writes; these are explicit experiment boundaries.

Approved and implemented scope:

- Built-in scout, planner, reviewer and worker; home agents/*.md, then trusted
  project .rig/agents/*.md. YAML role metadata reuses the existing parser.
  Roles refresh on delegation; active/restored children keep frozen prompts.
  Invalid overrides disable the role; an optional name must match its filename.
- Native single delegation and follow-up, task_parallel with 1-8 tasks, and
  task_chain with {previous} substitution and stop-on-failure. Four children
  run per session, including follow-up turns; root launches queue. Nested spawn
  at full capacity is refused to avoid ancestors blocking all descendant slots.
- Default max_depth one, preserving explicit native configuration. Role/request
  tools cannot widen parent/role access. Reviewer has shell, so it is not a
  read-only sandbox; worker inherits parent tools.
- Independent conversations in the same process. Optional cwd is canonicalized
  within the current project and applied through native WorkDir. Instructions,
  skills and trust remain the host project's context; no host chdir, cross-project
  context reload, automatic Git worktree or OS isolation is implied.
- Workflow-owned reports are claimed before parent acceptance and produce one
  aggregate. Parallel success survives another child's failure. Child previews
  are at most 50 KiB including truncation/reference text; full native child
  transcripts remain available. There is no second transcript store.
- Parent stop cancels active children and pending launches/follow-ups. Resume
  does not replay interrupted effects; completed children retain conversations,
  role and cwd for follow-up. No persistent background daemon was added.
- /roles, native /agents and /usage, readable result cards, live progress,
  child tool activity, status and token/cost information. Unknown pricing stays
  unknown. Esc is Rig's terminal stop key; Ctrl+C clears the input.

Evidence: 46 tui-enabled basics tests and 20 root workspace tests passed. Actual
Windows species checks exercised file reads, worker file writes, reviewer reads,
single/parallel/chain outcomes, partial HTTP failure, role trust, child access,
terminal cards/views/usage, and four real shell descendants terminated on Esc
without starting the two queued tasks. Headless delegation passed with no rig-tui
in the normal dependency graph; removing the subagent component removed all five
delegation tools. Existing headless smoke passed shell, sessions, Python state,
JSON, errors, credentials and model facts. The older sessions_tui.py is labelled
Unix-only and failed its Windows run; it is not claimed as Windows evidence.
The new targeted ConPTY smoke provides this delivery's terminal evidence.

Independent review found and corrected premature workflow completion while a
child awaited a descendant, and automatic trust of late-created roles when initial
resource discovery was empty. Failing regression tests became green; workflow
completion now follows the correlated native report, and trust requires an
affirmative decision. The actual executable also rejects late untrusted role
injection. Headless question/admission ordering uses native PrintSystems.

Real gateway attempts failed at the initial HTTP request with zero model calls;
there is no verified real-provider delegation. Details and final review outcomes
belong in vivy-test's dated delivery record. Scripted tests are not real-provider evidence. No Pi TypeScript
extension compatibility, complete Pi parity, Python bridge, apixis integration,
new permission platform or original Vivy runtime modification is claimed.

## 18. Go Vivy migration assessment

Recorded 2026-10-27 at the owner's request. The target considered here is the
current independent Rust/Python experiment, retaining minimum useful behavior
from Go Vivy and its remake donors. Complete feature equivalence and historical
data compatibility are separate, substantially larger scopes.

### 18.1 Source boundaries and measured size

The original repository at `C:/Users/Administrator/Desktop/agent-vivy` was read
through Git objects, without checkout changes, builds, app launches or access to
sessions, databases, credentials or production data. `git ls-remote --heads`
confirmed the inspected remote main/remake heads on 2026-10-27; no fetch or push
was performed. The local main is a different source snapshot, not the remote
main with a newer date applied to the same code.

| Snapshot | Exact revision | Go product source | All selected product source | Tests and fixtures | Generated | Other counted source |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Local main | `1df588a1a8e538524a6f0e757ac02a68f14d50ca` | 131,570 | 168,875 | 118,538 | 1,867 | 1,607 |
| Remote main | `9cd2801f8090723a652d3e835f72d83715b82121` | 184,507 | 234,319 | 186,129 | 1,972 | 4,446 |
| Remake integration | `0059b73138b3830fd3bfc1c041b9bc8353e93a33` | 153,550 | 196,382 | 138,987 | 1,913 | 2,451 |
| Remake archive | `495caf6eba2063a71f16e7a8066c9935ff241839` | 184,507 | 234,319 | 186,129 | 1,972 | 4,446 |

Go product source is a subset of all selected product source. Counts are physical
lines, including blank lines and comments, obtained from `git ls-tree -r` and
`git cat-file --batch` blobs, not non-comment executable statements. Selected
extensions: Go, TS/TSX, JS/JSX, Vue, Svelte, Rust, Python, SQL, Proto, C/C++/headers
and shell/PowerShell/batch scripts. Markdown, JSON/YAML, lockfiles, CSS/HTML,
binary assets, untracked files and submodule contents are outside this count.

Classification is path/header based: generated-code markers, `.pb.go`,
`_generated.go`, `.gen.ts(x)`, bindings and wailsjs; then third_party/vendor;
then scripts/docs/agent-support/spikes and shell launchers; then `_test.go`,
`.test/.spec` and tests/testdata/fixtures/conformance/testsupport/e2e directories;
remaining source is product source. This separates reusable conformance helpers
even where their filename is not `_test.go`; ordinary product files can still
contain small test helpers. The numbers are a reproducible scope indicator,
not an exact measure of essential runtime logic.

Remote main contains 167,202 Go test/helper lines separately from its 184,507 Go
product lines. Generated source is under 0.5% of the total selected 426,866 lines.
The large footprint is therefore not explained by generated code. Conversely,
test volume is not evidence that the corresponding behavior is unnecessary.

The source tree also excludes external Laputa/INOFY implementations and the
`studio` submodule. Main's go.mod pins Laputa garden/mentle/laputa and redirects
them to sibling Go modules. Their native implementation does not automatically
become reusable Rust or Python code. These counts understate a complete product
ecosystem migration, rather than including it implicitly.

### 18.2 Remake is primarily a split of existing source

All 18 remote remake refs were checked: 16 donor branches plus integration and
archive. Relative to shared original base `202aac3885cb1479998127f0fb9004bea54aca57`,
the 16 donors change 624 distinct selected source paths and contain 626 distinct
`(path, resulting blob)` versions, including deletion markers. Of those, 617
exactly match the corresponding remote-main path/blob. This is source-identity
evidence, not proof that the branches integrate or pass together. This donor
comparison excludes source snippets under docs and agent-support directories.

The nine different versions concern engine/tool adaptation and ordering,
Plan/work control, and associated tests. They are not nine whole missing
features. Archive has the same counted source categories as remote main;
integration has the same product/test source as base A, with additional support
scripts. Adding every branch's full line count would massively double count.

The pinned [remake index](https://github.com/ProjectViVy/agent-vivy/blob/0059b73138b3830fd3bfc1c041b9bc8353e93a33/docs/superpowers/plans/remake-20261019/index.md)
explicitly calls these donor branches, not build/product-accepted features.
R10-R24 wait for accepted core/contracts; R25 is a warming-history handoff.
Those are the recorded states at the inspected commit, not a fresh runtime test.
Remake contributes behavioral requirements and regression examples, not a second
entire product that must be added to main's migration inventory.

### 18.3 What accounts for the footprint

Remote-main product source under the same classifier:

| Area | Lines | Migration implication |
| --- | ---: | --- |
| `internal/runtime` | 49,537 | Interactive runs, durable work, child agents, workflow, reports, context, memory, tools and recovery coexist here. This is much more than an LLM loop. |
| `internal/storage` | 29,661 | SQLite and PostgreSQL, multiple product stores, journal/history, recovery and paired migrations. One minimal local backend avoids reproducing both implementations. |
| `ui` | 32,687 | Browser product and its configuration. Reusing React presentation still requires adapting transport, stores and feature assumptions. |
| `sdk/tui` | 16,413 | A substantial terminal product lives inside sdk; the whole sdk directory cannot fairly be called assembly overhead. Rig's terminal is the replacement candidate. |
| `internal/rpc` | 12,301 | Control/query/event projection for the broad product, including a 7,345-line `control.go`. A smaller product exposes fewer commands. |
| `internal/app` | 8,558 | Composition and lifecycle, including a 2,458-line app.go and many feature bridges. |
| `plugins` | 24,508 | Multiple channels, UI modules, ACP, notebook, workflows, memory, persona and other integrations. |

These are disjoint examples, not the full sum. The 28,064-line sdk product total
includes the 16,413-line TUI as well as assembly, frontend, Ports and launch code.
The runtime Service alone is 5,752 lines, with shared state for admissions,
reviews, Goal, tools, bundles, Recap and shutdown. This concentration makes
cross-feature adaptation expensive even when an individual tool is small.

The architectural diagnosis is scope accumulation plus the cross-product cost
of state, interfaces and compatibility: every feature gains persistence,
RPC, UI, module wiring, lifecycle and regression work. Broad provider/channel
support, two storage backends, three product faces and a distribution/plugin
platform multiply that cost. This inspection does not establish what fraction
is duplicate or removable dead code; no blanket deletion percentage is justified.

Pinned source anchors: [runtime Service](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/service.go),
[engine wiring](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/engine.go),
[storage contracts](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/storage/contracts.go)
and [Go dependencies](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/go.mod).

### 18.4 Suggested minimum behavior selection

This table is assessment advice against the owner's saved direction, not approval
to delete or implement features. Retain the behavior and failure invariants;
do not port an old subsystem merely because its branch name survives.

| Remake area | Minimum replacement or disposition |
| --- | --- |
| core-fixes | Translate applicable correctness regressions: one terminal outcome, cancellation, cleanup, failed/unknown effects and recovery. Do not carry all old Eino glue. |
| provider | Use the existing gateway/Rig path. Keep the configured model identifier, streaming, tool-call fidelity and credential redaction; defer the old provider catalog/routing breadth. |
| notebook | Start with ordinary workspace files and Python libraries when needed; defer a notebook database/editor and its separate provenance/RPC system. |
| reports | Keep one requested report workflow as a Python capability if useful; defer the complete report product/settings/publication system. |
| toolkits | Replace the initial need with discoverable Python capabilities, explicit dependencies and MCP configuration. Defer content-package acquisition, release, rollback, revocation and management UI. Python alone does not implement that package lifecycle. |
| bml | Use the selected partitioned MemPalace long-term-memory direction; do not maintain a second authoritative BML store. Historical BML import is separate. |
| recap | Preserve durable's ACTMEM/Recap intent: versioned working state, source association, commit before discarding context, bounded recall and correction. Use mysoul's later results as behavioral reference without imposing them on coding. |
| cognition | Retain only concrete durable task/continuity responsibilities. Defer Mission and autonomous cognition/evolution machinery as first-release gates. |
| skill-evolution | Keep ordinary skill discovery/use. Reviewed or automatic mutation is a later independent capability. |
| persona | Preserve Laputa as the sole persona authority; implement the minimum consuming boundary. Its Go implementation is not already present in this Rust runtime. MemPalace L0 remains excluded. |
| workflow | Express specific useful compositions in Python; defer INOFY compatibility, general DAG storage, revisions and visual editing. |
| acp | Defer unless an actual client requires it. No ACP compatibility promise is needed to validate the two harnesses. |
| tool-parallelism | Reuse framework support for bounded independent calls when required; preserve ordering and cancellation for effectful work. |
| chat-ui | Keep a usable entry, streaming, stop, failures and resume visibility; existing TUI is the first entry. A browser surface is a separate selected slice, not implied full UI migration. |
| misc | Select fixes relevant to the new code and Windows distribution; no bulk replay of maintenance commits. |
| warming | Defer until measured latency/cost warrants it; no separate nudge/warming/steering framework prerequisite. |

The minimum useful replacement is two distinct harnesses using one workspace:
classic interactive coding; durable task admission/checkpoint/wake/abort; one
trimmed Python capability/MCP loop usable by each; Laputa persona plus durable
ACTMEM and domain-partitioned long-term recall. Keep compilation-based recipe
selection and one model gateway. Exclude complete old UI/settings parity,
multi-channel delivery, Studio, public SDK/Port compatibility, generalized
workflow/evolution platforms and legacy database/checkpoint import from the
initial estimate. These omitted features remain possible later scope, not
requirements the owner has agreed to discard permanently.

### 18.5 Reuse and the remaining hard boundaries

Current vivy-test already supplies assembly, launcher, gateway, classic coding
tools, context-file loading, session/branch support, skills/trust, Plan and
bounded Rig-native delegation. Its tracked Rust/Python source files total
11,733 physical lines: 9,249 in implementation files and 2,484 in separate test
files. Inline Rust tests remain in the first number, so this is not an exact
production-only comparison or proof of equivalent coverage. Existing recorded
checks in HANDOFF apply; no new app/gateway runs were made for this assessment.

Reuse candidates are Rig's coding/session/terminal facilities; Prime's Python
protocol/event-loop and managed MCP patterns; the Python MCP SDK; native
MemPalace library calls; and concrete mysoul memory semantics. Pi-durable is a
TypeScript behavior reference, not a drop-in Rust durable engine. The native
MemPalace facade and Laputa Rust-consuming boundary remain unimplemented.

The following responsibilities survive any reduction in tool-development work:

1. **Durable authority:** admitted task identity, committed phase, deduplication,
   explicit abort, persisted wake times and host restart. A cancelled waiter is
   not a cancelled task; a timer in an exited process cannot wake anything.
2. **Effect outcomes:** a Python/remote operation may finish just before a crash.
   Unknown outcome must not become automatic replay. Persist logical operation
   identity/outcome and define replay eligibility for retained operations.
3. **Workbench lifetime:** the current worker preserves variables but cancels
   asyncio tasks at every cell end. MCP sessions and long-lived clients need a
   continuously serviced loop, managed resources and an unblocked cancel/control
   path; execution/output alone does not close this loop.
4. **Memory commits:** MemPalace retrieval is not authoritative task state.
   Preserve Recap revision/source association and archive/commit ordering;
   coordinate shared-workspace writes and distinguish memory partitions from
   permission boundaries. Do not copy the original subject/session schema
   wholesale just to retain these invariants.
5. **Language boundary:** Python expands capability libraries, but Rust must
   still own the chosen task lifecycle, cancellation and bounded admission of
   outputs. Duplicating the model/agent loop in Python would defeat the target.

Go's `internal/runtime/checkpoint.go` explicitly binds opaque Eino checkpoints
to engine version and prompt identity. Such checkpoints are not portable Rig
continuations. Reading old exported conversation text, importing structured
memory and resuming an interrupted effect are three different compatibility
tasks. No legacy data inspection or migration is included here.

Go already has cron: `internal/runtime/cron_scheduler.go`, CronStore and matching
RPC/UI/storage paths. Its recorded startup policy skips missed recurring
occurrences and disables overdue one-shots; that is not pi-durable's persisted
sleep/wake contract. A minimal new scheduler should admit due work into the
durable harness with explicit missed-run/overlap policy, rather than invoke
Python in an unrelated second execution path. Rust retains durable scheduling
authority; Python can expose schedule management. Live-host or OS wake support
must be selected before claiming tasks run while the product is closed.

Pinned behavior anchors: [checkpoint binding](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/checkpoint.go),
[Recap compaction](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/recap_compaction.go)
and [cron lifecycle](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/runtime/cron_scheduler.go).

### 18.6 Difficulty and planning allowance

**Full Go/remake feature equivalence is high difficulty and is a product rewrite.**
Changing language/framework while preserving all data, approval checkpoints,
protocols, interfaces and plugin behavior retains most of the expensive contract
work. This audit does not justify a reliable full-equivalence completion date.
It would require per-surface acceptance and data-migration investigation.

**The owner's minimal two-harness replacement is medium-to-high difficulty,
with a bounded scope.** The coding base is substantially underway; durable,
Python lifecycle and memory commit/recovery are the main remaining work. Do not
convert 180,000 Go lines into a Rust-line estimate or assume Python removes
state-management work.

Illustrative engineering allowance, not a measured delivery commitment: one
experienced contributor familiar with Rust/Python, using the current experiment,
Windows as the initial platform, one gateway, local storage, no legacy data
compatibility and no new browser/Studio product:

| Work package | Focused engineering days | Completion evidence |
| --- | ---: | --- |
| Verify and close the retained coding path | 3-6 | Representative real coding task, tool/model failure, stop, explicit resume and session/compaction behavior. |
| Persistent Python capability lifecycle | 4-8 | Discovery/instructions, cross-cell async client, host callback, bounded result, failure/reuse, targeted cancel and shutdown. |
| One managed MCP integration | 3-6 | Configure/connect/list/call, timeout/disconnect, reload/close and child-process cleanup. |
| Minimal durable harness and scheduling | 8-15 | Admission dedup, committed checkpoint reopen, sleep/wake, explicit abort and safe treatment of interrupted effects. |
| Laputa boundary, ACTMEM and native MemPalace | 6-12 | Persona authority, versioned Recap commit/correction, scoped remember/search/get and cross-harness partition behavior. |
| Combined recovery and Windows packaging | 5-10 | Kill/reopen at selected boundaries, install/start diagnostics, resource cleanup and one useful end-to-end task per path. |
| Total | 29-57 | Roughly 6-12 five-day engineering weeks; not a calendar or staffing promise. |

These are judgment ranges inferred from source/contract gaps, not historical
velocity measurements. Packages may overlap; changing the durable backend,
Laputa integration or native-memory write contract can move the total outside
the range. A coding-plus-Python demonstration can precede the two-harness
replacement, but does not establish the promised persistent companion/memory
behavior. A browser face, channels, compatibility/import or autonomous evolution
would need their own estimate after selection.

The recommended next commitment is one bounded vertical slice with measured
failure/recovery evidence, then re-estimate the remaining packages. Continue
the saved order: establish coding and durable foundations separately, close
the trimmed Prime capability loop, then the durable memory slice. Treat the
old regressions as a source of obligations; do not require restoring every
layer built to serve the old product's much broader ambitions.

Verification for this assessment: source/ref identity and physical-line counts
were checked; the original Go product and remake donors were not built or run.
Documentation changes are checked separately with `git diff --check`. No source
port, dependency installation, private-data migration or upstream push is part
of this deliverable.

### 18.7 Owner refinement and remaining integration work

The owner clarified that the intended feature transfer is substantially smaller:
retain small bundle and hook capabilities, build on existing Plan/Goal
infrastructure, rewrite Laputa, retire BML and adopt the full upstream MemPalace
engine. This supersedes any reading of 18.4 as deferring every bundle/hook
feature or requiring migration of the old toolkit/cognition architecture.
The earlier 29-57 engineering-day allowance combines new harness/memory work
and replacement-product verification; it is not an estimate of old-code transfer
and should not be reused as a quote for this narrower selection.

Source check at Rig `55cd481a2421fd5da468b5bd6afc6ac876883599`:
`corpus/sn/src/hook_scripts.rs` provides session_start, prompt_submit,
before_tool, after_tool and turn_end external scripts, timeout/cancellation and
project trust. `corpus/base/src/session_goal.rs` provides saved user-set goal
text, prompt/status projection and restore. Neither plugin is selected in the
current coding recipe. The delivered Vivy PlanPlugin already uses Rig's request,
review, cancellation and saved-state boundaries. These are a strong foundation;
a saved goal does not yet supply autonomous advancement or durable execution.

Suggested remaining integration inventory, without creating new platforms:

- One content-bundle import/list/enable convention for the actually retained
  skills, prompts, hook declarations and Python/MCP configuration. Keep compiled
  Rust selection in recipes. Exact format/lifecycle is not decided here.
- A managed Python environment and capability lifecycle: interpreter/dependency
  reproducibility, discoverable instructions, live asyncio/MCP resources,
  bounded results, cancellation and shutdown. Existing shell/Python process
  support is reusable, not a second installer/service platform requirement.
- Durable host availability, persisted task/wake state, abort and recovery;
  Goal's advance/finish/stop behavior uses that path when persistence is needed.
- Rewritten Laputa's persona/context boundary, versioned ACTMEM/Recap and full
  MemPalace integration. Preserve native write/read semantics and domain/source
  association. Full engine availability does not activate a second persona L0
  or make MemPalace the authoritative in-flight task store.
- Shared workspace/task identity and coordination of conflicting writes.
  Python's outer execute hook does not observe every operation performed by
  libraries inside a cell; expose capability-level events only where the
  retained product actually needs hook visibility/control.
- One chosen daily-use entry and useful task status/results. Existing coding
  TUI is available; a companion channel/browser face is a later explicit choice.

Reusing the old web UI, all channels, LSP/ACP, historical data, general workflow,
Studio, automatic evolution and distribution governance is not newly selected
by this clarification. No implementation or upstream data change was made.
