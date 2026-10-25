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
Read [DESIGN.md](DESIGN.md) for decisions and [PI-BASELINE.md](PI-BASELINE.md) for
coverage. Those documents replace the need to reconstruct the chat history.

Latest scope reduction (2026-10-25): first deliver simple Pi-like delegation using
Rig's native subagent plugin: one foreground child with independent context,
visible progress/result, truthful failure and propagated cancellation. Set the
initial nesting depth to one; no graph, grandchild or Python bridge is required.
Python/apixis remains a later composition direction over that same Rig executor.
See DESIGN's "Recursive delegation and optional graph orchestration" and mirrored
research section 17.6. No new approval platform is a prerequisite; do not claim
unverified permission enforcement. None of these subagent features is integrated
yet, and the existing delivery gates below still apply.

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
