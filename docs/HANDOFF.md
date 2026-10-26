# Handoff: continue the independent assembly experiment

## Goal and current state

The owner wants a fast Rust experiment preserving Vivy's useful idea of a species
assembled from a YAML recipe at compile time. Rig supplies implementation;
Pi coding-agent supplies the functional reference. Model access is gateway first.
The current direction is a Pi-style coding core with direct tools, a Python
workbench for deferred capabilities, and Prime-anchored automatic evolution with
a narrow persona addition. Keep the implementation small and independently useful.

Repository: `https://github.com/mastwet/vivy-test`.
Current state: the builder, gateway, native Windows launcher, Git Bash process
adapter and auxiliary Python workbench are implemented. DESIGN's "Windows coding
species and Python workbench" section records implementation evidence. The owner
reported a successful manual run; that is not a newly recorded automated gateway
test. Gateway model facts (delivery step 1) are implemented and checked against
the scripted gateway. The owner's 2026-10-25 order in DESIGN's "Delivery slices
and stopping points" is the current plan: core first, Python for anything
advanced.
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

The owner also requested provider-management research on 2026-10-26. See
[provider management and plugin boundaries](research/2026-10-26-provider-management.md)
for Pi/DSH/Rig evidence and a proposed extension of the gateway-first surface.
The owner fixes provider scope to OpenAI-compatible and Anthropic-compatible
protocols only, with built-in presets and user-defined instances derived from
them. Subscription login/OAuth/token refresh and other native protocols are
excluded. DESIGN records this scope; the research records Rig's registration
boundary and an offline API-construction probe, with no live-provider claim.
Provider management is not implemented; the delivery order below is unchanged.

Follow DESIGN's "Delivery slices and stopping points" in order; do not start a
later step before the earlier one's check passes. Step 1 (gateway model facts)
is done offline. Next is step 2: one real-gateway smoke, then Pi and vivy
against the same mock gateway, recording a result in every PI-BASELINE core
row. Preserve the Windows launcher and Rig harness/TUI; do not write a
replacement agent loop. Prime evolution, persona, PEN, MCP in Rust and new
design documents wait until step 4 works.

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
- **Keep feedback useful.** Bounded task evidence feeds the accepted automatic
  refinement slice. Reuse Prime's state/proposal/history vocabulary; its separate
  incident CLI, daemon, service catalog and subagent platform are not prerequisites.
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
