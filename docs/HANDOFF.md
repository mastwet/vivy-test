# Handoff: continue the independent assembly experiment

## Goal and current state

The owner wants a fast Rust experiment preserving Vivy's useful idea of a species
assembled from a YAML recipe at compile time. Rig supplies the current coding
implementation. Model access is gateway first. The owner's 2026-10-27 direction
is two independent harness paths: classic Pi for coding, pi-durable for durable
execution and persistent companionship. Establish those foundations first, then
build a trimmed Prime-style Python capability loop against each consuming path.
Both paths use the same workspace and may recall the same domain-partitioned
MemPalace memory. Laputa owns persona; do not inject MemPalace's L0 identity.
Durable's layered context and ACTMEM/Recap remain a later memory slice; do not
apply mysoul's companionship context policy to coding by default. Domain wings,
topic rooms and source drawers are distinct from the optional L0-L3 context
stack. Preserve per-task state/namespace boundaries within the shared workspace.
Recursion, factory, a complete Prime daemon and automatic evolution are not
prerequisites. See DESIGN's **Separate durable and coding harnesses** for the
current scope. No second harness or new memory implementation is delivered yet.

### Saved discussion checkpoint (2026-10-27)

The owner asked to save today's decisions before further work. Resume from
DESIGN's current two-harness section and **Python-native memory candidate and
research checkpoint**, not the older full Prime/evolution direction. The order
remains: establish coding and durable harness foundations separately, then close
the trimmed Prime Python capability loop. Shared workspace/domain memory does
not merge their task state or replay semantics. Keep the implementation small.

The latest investigation is a native MemPalace library facade inside Python.
Programmatic search and collection access exist, but native write/chunk/read
equivalence is unverified. remember/search/get are proposed facade names, not
shipped APIs. Laputa supplies persona; MemPalace L0 is not injected. No MemPalace
fork, native-memory implementation or dependency change was made. Record costs
before treating optional-feature omission as a smaller installation/runtime.

Prime runtime `c4c238ca025e382d96fb5f37e6328ca77fb82ef3` passed seven isolated
Python protocol/lifecycle probe checks; that does not validate its Rust adapter,
MCP, snapshots or full background operation. Existing vivy worker tests passed
six cases; its asyncio tasks are still cancelled at cell end. DESIGN preserves
the exact inspected paths, pins, probe limits and mysoul reference locations.
mysoul remains read-only reference material; no sessions or memory data are
copied. Next work requires a bounded implementation scope, not another broad
capability inventory. Today's save contains documentation only.

Repository: `https://github.com/mastwet/vivy-test`.
The owner's subsequent Go/remake migration assessment is recorded in
[research section 18](research/2026-10-25-hybrid-agent-workbench-decisions.md#18-go-vivy-migration-assessment).
It checks pinned source sizes and donor overlap, proposes a minimum behavior
selection, and gives a conditional engineering allowance. It does not authorize
implementation or change the two-harness scope. Original source was inspected
read-only; no app data, source port or legacy data migration was involved.

The owner's next refinement reduces migration scope further: retire BML, use
the full upstream MemPalace engine in Python (still no L0 persona injection),
and rewrite Laputa. Small bundles and hooks remain intended capabilities.
Plan is implemented; external hook and saved session-goal examples exist in Rig
but are not selected. Autonomous Goal remains unwired. See DESIGN's native-memory
section and research 18.7; do not reuse the earlier broad estimate as a quote
for copying old features.

The owner requested a broad feature-by-feature reconciliation, including A2A.
[The Vivy feature inventory](research/2026-10-27-vivy-feature-inventory.md)
separates owner scope decisions from actual target implementation, covers all
16 remake lanes plus A2A/ACP/SSH and lists unresolved product choices. A2A has
accepted server source/evidence on its Go feature branch but is not selected in
vivy-test; its server and an outbound A2A client are separate capabilities.
Use that inventory rather than treating older suggestions to defer as decisions.

Current state: the builder, gateway, native Windows launcher, Git Bash process
adapter and auxiliary Python workbench are implemented. DESIGN's "Windows coding
species and Python workbench" section records implementation evidence. The owner
reported a successful manual run; that is not a newly recorded automated gateway
test. Gateway model facts (delivery step 1) are implemented and checked against
the scripted gateway. DESIGN's 2026-10-27 two-path decision supersedes conflicting
scope and ordering in its older "Delivery slices and stopping points" record.
The later owner-approved collaborative Plan addition is implemented in
`vivy-basics::plan::PlanPlugin` and selected in the baseline recipe. It includes
terminal mode markers, explicit plan submission and user review/feedback;
see DESIGN's "Collaborative Plan mode" and README usage. Goal and automatic
evolution remain separate work.
The owner-approved 2026-10-26 terminal refresh selects a compact vivy-test screen
on Rig's public widgets, a persistent footer and a branded default identity.
See DESIGN's "Compact vivy-test terminal" and the verification appendix below.
Read [DESIGN.md](DESIGN.md) for decisions and [PI-BASELINE.md](PI-BASELINE.md) for
coverage. Those documents replace the need to reconstruct the chat history.

Latest delivery (2026-10-25, `feat/subagents`): the recipe selects a Vivy wrapper
around Rig's native delegation. It provides roles, task/message/wait, bounded
parallel batches, chains, saved child follow-up and terminal progress/usage.
Children share the host process and project context; cwd stays within the project.
Default depth is one, with four active children per session. No Python bridge,
apixis, sandbox or automatic worktree was added. See DESIGN and
[delivery evidence](logs/2026-10-25-subagents.md); mirrored research section 17.7
supersedes the previous single-child scope. Commit c94f87f was fast-forward merged
into local main on 2026-10-25. Collaborative Plan commit b43d448 is integrated
alongside it on 2026-10-26. This integration has not been pushed.

## Reference setup

Clone **Pi and Rig themselves**, including source, into this checkout's ignored
`.workspace/`. Installed packages alone are insufficient for the investigation.
Do not copy old reference folders from the original Vivy checkout. Do not clone
Vivy: it is undergoing separate repair, and this repository must not inherit its
build, runtime, SDK or working-process dependencies.

Run from the `vivy-test` root, only for reference directories that do not already
exist. If a checkout already exists, inspect its status and remote before making
changes; do not overwrite somebody's work.

```sh
git clone --depth 1 https://github.com/earendil-works/pi.git .workspace/pi
git -C .workspace/pi fetch --depth 1 origin f321e5bdd26b3624bfb2ab182a334d0a5edb698a
git -C .workspace/pi checkout --detach f321e5bdd26b3624bfb2ab182a334d0a5edb698a

git clone --depth 1 https://github.com/0xPlaygrounds/rig.git .workspace/rig
git -C .workspace/rig fetch --depth 1 origin 55cd481a2421fd5da468b5bd6afc6ac876883599
git -C .workspace/rig checkout --detach 55cd481a2421fd5da468b5bd6afc6ac876883599
```

These ordinary source pins identify what was inspected. They do not introduce a
species hash system. If a revision cannot be fetched, report it; do not silently
substitute current HEAD and repeat the old findings. Updating the baseline is a
normal dependency decision after reviewing the relevant changes.

The commands above are setup instructions when references are missing. No upstream dependencies need
installation just to read source. Read each reference repository's own
instructions before running its tooling. Keep product changes in `vivy-test`.

## Where to look first

| Question | Source in the reference checkouts |
| --- | --- |
| How does Rig already generate an agent? | `rig/src/launcher/project.rs`, `build.rs`, `rig/crates/rig-harness/README.md` |
| What can the existing product reuse? | `rig/plugins/rig-basics/`, `rig/plugins/rig-tui/`, `rig/crates/rig-harness/PLUGINS.md` |
| What is the smaller library route? | `rig/crates/rig-agent/README.md`, its `src/run/` implementation, `rig/crates/rig-core/` |
| What do coding tools and session storage actually guarantee? | `rig/crates/rig-tools/`, `rig/crates/rig-memory/`, `rig/crates/rig-ecs/src/stage.rs` |
| What does Pi's loop do? | `pi/packages/agent/src/agent-loop.ts`, `pi/packages/coding-agent/src/core/agent-session.ts` |
| What must session and extension behavior preserve? | Pi coding-agent `docs/sessions.md`, `session-format.md`, `compaction.md`, `extensions.md` |
| What are the current MCP and Code Mode capabilities? | Pi coding-agent `docs/mcp.md`, `codemode.md`; Rig `crates/rig-rmcp/`, `plugins/rig-steel/` |

## Suggested next implementation task

The 2026-10-27 scope clarification takes precedence over older sequencing below.
Work separately on the classic-Pi coding harness and the pi-durable-referenced
durable harness; select a bounded acceptance case for each before implementation.
`pi-durable` is a TypeScript reference, not a Rust integration already present.
Then connect the trimmed Prime Python capability loop. Reuse existing coding
verification, but do not treat session resume as proof of durable task replay,
sleep/wake or waiter/abort semantics. No new universal harness abstraction is
required. mysoul's memory behavior is a durable-path reference, not a dependency
on its application, private data or runtime.

The owner also requested provider-management research on 2026-10-26. See
[provider management and plugin boundaries](research/2026-10-26-provider-management.md)
for Pi/DSH/Rig evidence and a proposed extension of the gateway-first surface.
The owner fixes provider scope to OpenAI-compatible and Anthropic-compatible
protocols only, with built-in presets and user-defined instances derived from
them. Subscription login/OAuth/token refresh and other native protocols are
excluded. DESIGN records this scope; the research records Rig's registration
boundary and an offline API-construction probe, with no live-provider claim.
Provider management commit `de10ee3` was fast-forward merged into local `main`
in `vivy-test` on 2026-10-26. The baseline selects ProvidersPlugin, two protocol
features and the full TUI editor. README documents commands and configuration;
the existing research document records verification and review corrections.
The native launcher and baseline agent were rebuilt from this main checkout.
The integration remains local. No new live-provider or non-Windows platform
claim is made.

The older coding-only delivery sequence and its verification gaps remain in
DESIGN and PI-BASELINE. Preserve the Windows launcher and Rig harness/TUI while
checking that path; keep actual gateway evidence distinct from mock checks.
Those coding records do not establish or select the durable implementation.
The current two-path harness work precedes the trimmed Prime capability loop;
automatic evolution, persona growth and PEN remain separate future scope.

Use existing authorized gateway configuration without printing credentials. If
essential configuration is unavailable, request only that missing information.
Do not guess secrets or present a scripted endpoint as a real gateway smoke.

## Research directions and traps

- **Measure iteration cost.** More upstream reuse can mean more dependencies but
  less owned product code. Check cold/warm/plugin-edit builds and debugging cost
  before deciding that a smaller-looking crate is the faster foundation.
- **Prove real optionality.** Inspect the resolved dependency graph for each
  generated agent. Shared upstream crates can contain several tools/providers;
  omitting a wrapper does not prove all their source was excluded.
- **Keep feedback useful.** Preserve bounded task evidence for useful errors and
  future memory/refinement work. The Prime workbench loop does not require its
  incident CLI, daemon, service catalog, subagent platform or automatic evolution.
- **Preserve semantics as coverage expands.** Branch-aware state, cancellation,
  extension interception, and tool-result pairing matter more than matching
  command names. Use representative behavior checks.
- **Keep source assertions modest.** Rig library availability does not prove
  integration with this gateway, terminal, platform, or recipe. New APIs at this
  pre-release revision may differ from older online examples.

## Evidence at handoff

DESIGN's dated implementation sections retain build, scripted-gateway and Windows
smoke evidence and limitations. They do not verify the new hybrid design. For the
2026-10-25 documentation delivery, review content/links and run `git diff --check`;
runtime CI is unnecessary because no executable behavior changed. Later code
deliveries must use the repository's focused checks and a representative real-path
smoke. Source availability and a successful manual run do not prove full parity.

## Collaborative Plan verification (2026-10-25, Windows)

The collaborative Plan delivery supersedes the documentation-only scope above.
Checks used Rust 1.97.1, `CARGO_TARGET_DIR=C:\vt` and
`CARGO_PROFILE_DEV_DEBUG=0`:

```powershell
cargo test --workspace --locked
cargo test --manifest-path plugins/vivy-basics/Cargo.toml --features tui --locked
cargo check --manifest-path plugins/vivy-basics/Cargo.toml --lib --no-default-features --locked
cargo run -p vivy -- build recipes/coding-agent.yaml
python scripts/smoke.py C:/vt/debug/vivy.exe
python scripts/plan_tui.py C:/vt/debug/vivy.exe
cargo fmt --manifest-path plugins/vivy-basics/Cargo.toml -- --check
git diff --check
```

Results: 20 workspace tests and 37 basics tests passed, including 14 Plan cases.
Plan cases cover unchanged tools and writable behavior, exact review/feedback,
cancellation, missing reviewer, stale approval, modal `/plan off`, running-turn
transitions, session restore and fork isolation. An unfinished review was also
reopened to verify it never replays approval. Core feature tests failed before
implementation; stale-approval and modal-command regressions failed before their
corrections, then passed.

The real Windows ConPTY terminal passed 16 scripted checkpoints: Plan entry and
ordinary clarification, mode/review markers, scrolling an 80-step plan, approval
followed by a real file write, feedback, Esc, manual exit inside the modal,
re-entry and process/session resume. Captured gateway requests confirmed guidance
changes and a stable tool catalog. The existing gateway/shell/session/Python/JSON/
error/secret-filtering/image/reasoning/compaction smoke passed. No live-model or
new macOS/Linux acceptance is claimed.

The terminal driver uses test-only `pyte==0.8.2`, `wcwidth==0.2.13` and, on
Windows, `pywinpty==3.1.0`; these are not agent/runtime dependencies. They were
installed in ignored scratch and exposed through `PYTHONPATH`. The Windows
driver selects UTF-8 before launch. Its optional evidence output contains only
synthetic test screens and requests and remains outside Git.

Two disposable recipes were built and exercised: without Plan, the model receives
no Plan tool/guidance; without the terminal, Plan submission stops with a clear
missing-reviewer message and `vivy-basics` has no normal `rig-tui` dependency.
Temporary recipe variants are not maintained configuration sources.

Fresh independent review found the modal command interception; the typed command
now routes through the same backend command and is covered in both unit and
terminal tests. Re-review reported no remaining substantiated findings. Debug
terminal startup also confirmed the explicit phase ordering removed the Plan/
compaction ambiguity. Existing Windows fixture assumptions in context/trust
tests and a smoke assertion assuming every JSON `text` field was a string were
corrected in test code; related product behavior was unchanged.
Formatting and Git whitespace checks passed. Clippy completed with one existing
`collapsible_if` suggestion in the Windows shell implementation, outside this change.

## Combined Plan and subagent integration (2026-10-26)

The owner approved merging collaborative Plan commit b43d448 into local main,
which already contains subagent implementation c94f87f. Conflicts were limited
to README composition, plugin documentation and a trust-fixture comment. Both
plugin exports and recipe components are retained; the later empty-discovery
trust correction remains intact. Plan retains its advisory per-agent semantics
and does not introduce another permission or agent execution path.

Integration verification used CARGO_TARGET_DIR=C:/vt and
CARGO_PROFILE_DEV_DEBUG=0. The root workspace's 20 tests and the basics plugin's
61 tui-enabled tests passed, including a new combined native-stage-order check.
The no-default-features library check, root/basics formatting and Git whitespace
checks passed. The baseline recipe rebuilt successfully for the existing start.cmd.

Both scripts/plan_tui.py and scripts/subagents_smoke.py --tui passed on the actual
Windows ConPTY terminal. Checks included Plan approval followed by a real write,
feedback, modal exit, restart/resume, parallel/chain delegation, role trust,
usage, and cancellation of four shell descendants without starting queued tasks.
The existing scripts/smoke.py passed gateway, shell, session, Python, JSON,
failure, credentials and model-facts checks. Test dependencies were reused from
ignored scratch; none were added to the product.

The Plan terminal script initially hit stale slash-command completion at different
steps. Outside a review, it now waits for the exact editor text, dismisses
completion and then submits; the full scenario passed. Modal command behavior
and product keybindings were unchanged. This scripted gateway verification does
not establish real-provider compatibility. No push was performed.

## Compact terminal verification (2026-10-26, Windows)

The recipe now selects `VivyPromptPlugin` and the feature-gated `VivyTuiPlugin`.
The local Codex source at `970111f6cb` was inspected for presentation; neither
reference checkout was edited or made a new build/runtime dependency. Rig's
native execution, session, question, editor and transcript paths are retained.

Checks used the pinned Rust 1.97.1 toolchain, `CARGO_TARGET_DIR=C:/vt` and
`CARGO_PROFILE_DEV_DEBUG=0`:

```powershell
cargo test --workspace --locked
cargo test --manifest-path plugins/vivy-basics/Cargo.toml --features tui --locked
cargo check --manifest-path plugins/vivy-basics/Cargo.toml --lib --no-default-features --locked
cargo run -p vivy -- build recipes/coding-agent.yaml
cargo fmt --all -- --check
cargo fmt --manifest-path plugins/vivy-basics/Cargo.toml -- --check
git diff --check
python scripts/branding_tui.py C:/vt/debug/vivy.exe
python scripts/plan_tui.py C:/vt/debug/vivy.exe
python scripts/subagents_smoke.py C:/vt/debug/vivy.exe --tui
python scripts/smoke.py C:/vt/debug/vivy.exe
```

The 20 root tests and 73 tui-enabled basics tests passed. The new tests cover
model-visible default identity and preservation of an explicit role, changing
welcome metadata, Unicode and resize fitting, persistent draft-time status,
NO_COLOR, dark/light input surfaces with preserved diff/failure colors, native
multiline Paste events, title restoration and unrelated control sequences,
and complete/live tool output through actual registered renderers. Brand tests
first failed against upstream defaults. A stale input-hint redraw and missing
shell renderer were reproduced before correction, then passed.
Final visual QA retained the project name at the end of shortened directories
and fitted whole shortcut hints, with an additional failing-then-passing test.

The baseline rebuilt successfully for the existing start.cmd. A disposable
recipe with no terminal component also built, and its normal Cargo dependency
graph contained neither rig-tui nor ratatui. The no-default-features library
check, formatting and Git whitespace checks passed. Native linking emitted
MSVC's library/export-file informational warning; no new compiler warning was
introduced by the implementation.

The compact-screen script passed on real Windows ConPTY at 120x40 and 50x20,
both with the inherited NO_COLOR setting and with truecolor explicitly enabled:
branded startup/title, Plan-visible drafts, Unicode multiline continuation,
actual shell output, exit-7 failure cause, running output and Esc cancellation.
Synthetic terminal streams were rasterized and visually inspected at both sizes;
the images are ignored test artifacts, not screenshots of a real-provider task.
The scripted gateway checked that requests use the vivy-test default identity.
The existing Plan terminal script passed approval/write, feedback, scrolling,
modal exit and restart/resume. Subagent acceptance passed child views, usage,
parallel/chain workflows and cancellation of four shell descendants. The
existing general smoke passed model, file, shell, session, Python, JSON,
failure, credential-filtering, image, effort and compaction scenarios.

Test-only pyte 0.8.2, wcwidth 0.2.13 and pywinpty 3.1.0 were reused/installed in
ignored scratch and exposed through PYTHONPATH; no product dependency was added.
The new script closes its terminal before deleting its scratch directory and
uses process liveness on Windows, since pywinpty's output reader may wait after
the process exits. ConPTY multiline checks use the native backslash+Enter
fallback; the offline Paste test separately covers newlines and Unicode.

A fresh read-only source review found no substantiated issue. This review is
distinct from the builds and terminal runs above. All network acceptance here
used a synthetic loopback gateway; no new real-provider or Linux/macOS result
is claimed.
