# Handoff: start an independent assembly experiment

## Goal and current state

The owner wants a fast Rust experiment preserving Vivy's useful idea of a species
assembled from a YAML recipe at compile time. Rig supplies implementation;
Pi coding-agent supplies the functional reference. Model access is gateway first.
The latest priority is to run a real coding task early and remove unnecessary
platform mechanisms.

Repository: `https://github.com/mastwet/vivy-test`.
Current deliverable: documentation and a lean `AGENTS.md`. No agent, builder,
supported YAML schema, gateway setup or runtime smoke has been implemented.
Read [DESIGN.md](DESIGN.md) for decisions and [PI-BASELINE.md](PI-BASELINE.md) for
coverage. Those documents replace the need to reconstruct the chat history.

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

The initial documentation task does not clone references into this new repository;
the commands above are the receiving agent's setup. No upstream dependencies need
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

1. Confirm the available target/toolchain and the intended gateway without
   printing credentials. The inspected Rig revision requires Rust 1.97.1 or
   newer and documents Linux/macOS launcher support. Current development is on
   Windows. Identify whether a small native integration works; if choosing WSL
   or another target, state that limitation explicitly before claiming success.
2. Try the smallest runnable composition using Rig's existing harness, tools,
   and terminal where feasible. Compare only the concrete blockers against a
   `rig-core` + `rig-agent` host. Record the chosen route and reason in DESIGN;
   do not build both solutions or perform an unbounded architecture study.
3. Add a small YAML-to-static-assembly path for the selected route. Use existing
   Rig interfaces and ordinary Cargo dependencies. Prove a baseline recipe and
   one optional-component removal. Keep runtime gateway settings separate from
   compiled component selection.
4. Exercise a real streamed model/tool task in a disposable workspace, continue
   the conversation, and check failure/cancellation. Record exact commands,
   outcomes, limitations, and build timings alongside the implemented change.
5. Expand the Pi baseline in useful slices after the first proof. Prioritize
   observed daily-use gaps over a large public plugin framework.

The owner has not specified a gateway endpoint, credentials, model, or a final
platform choice in this brief. Use existing authorized configuration if present;
otherwise request only the missing information needed for a real call. Offline
assembly and focused tests can proceed independently. Do not guess secrets or
report a mocked endpoint as a successful real-gateway smoke.

## Research directions and traps

- **Measure iteration cost.** More upstream reuse can mean more dependencies but
  less owned product code. Check cold/warm/plugin-edit builds and debugging cost
  before deciding that a smaller-looking crate is the faster foundation.
- **Prove real optionality.** Inspect the resolved dependency graph for each
  generated agent. Shared upstream crates can contain several tools/providers;
  omitting a wrapper does not prove all their source was excluded.
- **Keep feedback useful.** Tool errors, run results and bounded diagnostics can
  guide the model's next action. Add those where needed before inventing an
  autonomous self-repair service. Prime Agent's separate incident CLI and harness
  refinement are background inspiration, not required infrastructure or an
  additional reference clone for this task.
- **Preserve semantics as coverage expands.** Branch-aware state, cancellation,
  extension interception, and tool-result pairing matter more than matching
  command names. Use representative behavior checks.
- **Keep source assertions modest.** Rig library availability does not prove
  integration with this gateway, terminal, platform, or recipe. New APIs at this
  pre-release revision may differ from older online examples.

## Evidence at handoff

Prior research inspected the pinned Pi and Rig source and the local Vivy assembly
implementation. Rig workspace metadata resolved with
`cargo +1.98.0 metadata --no-deps --offline --locked --format-version 1` in the
previous reference checkout; this was metadata inspection only. No Rig or Pi
build, integration test, or live model comparison established runtime parity.

The repository bootstrap is documentation-only. Appropriate verification is a
diff/content/link review and `git diff --check`; Rust CI and a product smoke do
not exist yet. Success for this deliverable means a successor can understand the
decisions, obtain the correct reference sources, and start the first proof without
cloning or consulting the original Vivy project.
