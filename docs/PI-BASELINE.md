# Pi coding-agent functional baseline

Source baseline: [earendil-works/pi][pi], revision
`f321e5bdd26b3624bfb2ab182a334d0a5edb698a`, coding-agent package `1.4.0`, inspected
2026-10-24. Rig comparison revision:
`55cd481a2421fd5da468b5bd6afc6ac876883599` (`0.46.0-rc.1`).

The user requested the complete coding-agent capability range, then prioritized
a minimal working experiment with gateway access and no standalone SDK or species
hash system. Keep the wider range visible without making it a first-run gate.
Rust functional equivalents are accepted; existing TypeScript extensions and
Pi's exact wire/source API compatibility are not requirements.

**No row has reached Pi parity.** The first slice (see DESIGN, "First slice")
compiles Rig's harness plugins into the recipe-built agent; rows marked
*Slice 1* name what was exercised there, offline against a scripted gateway
unless stated. Other Rig entries describe source-level reuse opportunities, not
verified equivalence. Pi document paths
are relative to `.workspace/pi/packages/coding-agent/` unless stated otherwise.

| Area | Pi behavior to retain in the coverage map | Rig direction / important gap | Pi source entry |
| --- | --- | --- | --- |
| Agent execution | Streaming text/thinking, tool calls, cancellation, retries, usage, sequential/parallel tools, queued steering and follow-ups | Reuse an existing Rig runtime; verify event ordering, queue behavior and failure handling. *Slice 1:* Rig harness loop; one streamed tool round trip, a gateway 401, and SIGINT stopping a running tool | `../agent/src/agent-loop.ts`, `docs/how-pi-works.md` |
| Coding tools | Read including images; edit/write; shell; optional grep/find/ls and native Windows PowerShell; truncation and full-output spill | `rig-tools` and basics coding plugins; exact defaults, limits and PowerShell are gaps to check. *Slice 1:* read/edit/write/shell compiled in; `shell` wrote a file and does not see `RIG_SECRET_*` *Slice 2:* Rig `search` and `list` registered (`vivy-basics`) as grep/find/ls equivalents; no context lines or literal flag. | `src/core/tools/`, `docs/cli.md` |
| Terminal interaction | Streaming transcript, tool/diff views, multiline editor, history, completion, image input, model/reasoning picker, status and usage | Evaluate `rig-tui` before writing a terminal; compare actual interactions. *Slice 1:* `rig-tui` compiled in as the `terminal` component and seen starting; not used interactively | `docs/usage.md`, `docs/tui.md` |
| Terminal customization | Themes, keybindings, custom rendering, widgets, dialogs, overlays, header/footer/editor hooks | Native Rust plugin equivalents; upstream UI extension points need mapping | `docs/extensions.md`, `docs/themes.md`, `docs/keybindings.md` |
| Sessions | Persistent JSONL tree, resume/list/name/delete, ephemeral sessions, branch switching, fork and clone | Rig memory/session primitives exist; branch semantics and persistence are not automatically Pi-equivalent. *Slice 1:* a session resumed in a new process with `RIG_SESSION`; branches/fork not exercised | `docs/sessions.md`, `docs/session-format.md` |
| Context management | Manual/automatic/overflow compaction, branch summaries, context edits, preserved tool-call/result pairing | Reuse Rig compaction where it fits; verify retained state and branch projection | `docs/compaction.md`, `docs/session-format.md` |
| Execution modes | Interactive, print, JSON event output, long-lived RPC control, embedding | Rig has terminal/print/JSON surfaces; its protocol is smaller than Pi's. Embedding can use normal Rust APIs. *Slice 1:* `-p` and `-p --json` exercised | `docs/cli.md`, `docs/json.md`, `docs/rpc.md`, `docs/sdk.md` |
| Configuration and trust | User/project settings, precedence, trusted project resources, CLI overrides | Reuse settings infrastructure; project trust is not a shell sandbox | `docs/configuration.md`, `docs/security.md` |
| Project context | AGENTS/override/context files, system prompt overrides and additions | Rig's AGENTS plugin is a starting point; compare discovery and precedence | `docs/configuration.md`, `src/core/resource-loader.ts` |
| Skills | Metadata discovery, lazy body loading, supporting files, explicit skill invocation | Rig skills plugin exists; verify discovery and model exposure. Pi's documented experimental `allowed-tools` is not an enforced sandbox | `docs/skills.md` |
| Prompt templates | File templates, metadata, arguments, expansion and invocation | Rig corpus examples are reference code, not a shipped default product feature *Slice 2:* `vivy-basics` templates with Pi's substitutions, from `.rig/prompts` and `RIG_HOME/prompts`; no trust step, no packages or `/reload`. | `docs/prompt-templates.md` |
| Extension lifecycle | Input/prompt/context hooks; model/tool events; session, compaction and trust hooks; reload behavior | Preserve useful behavior through Rust hooks. Native code changes rebuild/restart; no TS loader | `docs/extensions.md` |
| Extension contributions | Tools, commands, shortcuts, flags, custom messages, events, dynamic tool exposure, branch-aware state | Avoid a giant plugin API; implement typed contributions as real consumers appear | `docs/extensions.md` |
| Extension distribution | Install/update/remove/list reusable packages and bundled resources | Later Rust/local/git/registry equivalents; package manager and marketplace are not needed for the first recipe | `docs/packages.md` |
| Provider/model access | API keys, provider protocols, subscription OAuth, custom providers, model catalogs and capabilities | Gateway first; broad direct-provider/OAuth parity explicitly deferred. *Slice 1:* `vivy-gateway` plugin (OpenAI-compatible Chat Completions); real gateway not yet called | `docs/providers.md`, `docs/models.md`, `docs/custom-provider.md` |
| Model behavior | Reasoning settings, usage/cost, prompt caching, model switching, virtual model routing | Establish the gateway's actual support. Reuse routing/capabilities instead of duplicating a catalog | `docs/models.md`, `docs/virtual-models.md` |
| Specialized model services | Classification and image generation/editing through appropriate services | Later capability, gateway/provider dependent; not implied by chat completion support | `docs/codemode.md`, `docs/custom-provider.md` |
| Local model workflow | llama.cpp discovery and model management | Later; not a requirement to create a second direct-provider stack initially | `docs/llama-cpp.md` |
| MCP connections | stdio and Streamable HTTP, configuration, connection lifecycle, OAuth, tool changes/progress | `rig-rmcp` supplies primitives; configuration/auth/exposure UX still needs product work | `docs/mcp.md` |
| MCP tool/resource use | Namespaces, direct/deferred/Code Mode/hidden tool exposure, resource lists/templates/read, search | Preserve discovery and exposure semantics; remote tools sit behind a compiled MCP component | `docs/mcp.md`, `docs/codemode.md` |
| JavaScript Code Mode | Model-facing JS tool orchestration, tool discovery, bounded output, cancellation, branch-aware store/load | Separate from TS extension compatibility. Rig Steel is Scheme, so it does not supply JS equivalence | `docs/codemode.md` |
| Session utilities | Export/import, HTML output, copy/share, diagnostic/bug-report workflows | Add useful equivalents later; keep sensitive transcript sharing explicit *Slice 2:* `/export` (HTML, JSONL), `/copy`, `/session`; no import, share or bug report. | `docs/cli.md`, `docs/slash-commands.md` |
| Platforms | Linux/macOS/Windows behavior, shell handling and terminal integration; documented Termux usage | Current host is Windows; inspected Rig launcher supports Linux/macOS. Do not claim portability without a run | `docs/windows.md`, `docs/termux.md`, `docs/terminal-setup.md` |

## Findings that prevent misleading scope claims

- This Pi revision includes MCP and JavaScript Code Mode. Older descriptions of
  Pi as having no built-in MCP no longer describe this source baseline.
- Pi's branch tree affects extension state, compaction, Code Mode storage and
  resumed context; implementing a flat message log alone does not match it.
- Pi's CLI and its bare embedding factory do not automatically initialize the
  same built-in extensions. Test the surface being claimed.
- Subagents, plan mode, permission systems, OS sandboxes and git checkpoints can
  be extensions/examples. Pi's README does not claim subagents or plan mode as
  built-in defaults. The entire community extension ecosystem is not a finite
  parity requirement.
- `packages/durable` is a separate experimental Pi library. Do not quietly add
  its durable scheduler/replay contracts to ordinary coding-agent parity or
  assume the coding-agent already uses them.
- Rig's `corpus/` contains useful examples. An example's existence does not mean
  the default harness provides that feature. Classic `rig-agent` memory saves
  successful completed runs; failure/cancellation durability needs deliberate
  handling if required by the selected runtime.

Update this same table with implementation evidence and explicit decisions as
work progresses. A postponed feature is not a completed one. Do not create a
second feature inventory or estimate the expanded scope from earlier small-spike
estimates.

[pi]: https://github.com/earendil-works/pi/tree/f321e5bdd26b3624bfb2ab182a334d0a5edb698a/packages/coding-agent
