---
id: basics
type: module-design
status: draft
title: Pi-shaped coding plugins (`plugins/vivy-basics`)
parent: architecture
depends-on: [process]
---

## Responsibility

The product surface the recipe composes, as **separately selectable plugin
types** on Rig — one recipe component per concern, so leaving one out removes it
from the build rather than hiding it at runtime.

The Pi-shaped workflow parts translate Pi behavior onto Rig mechanisms instead of
re-implementing them: the coding identity (`prompt`); project trust plus the
skills discovery it gates (`trust`, `skills`); `AGENTS.md`/`CLAUDE.md` and
`SYSTEM.md`/`APPEND_SYSTEM.md` reading (`context`); prompt templates as commands
(`templates`); Pi's session and model command names over Rig's sessions and
`/effort` (`session`, `model`); a branch summary for what a rewind leaves behind
(`branch`); collaborative Plan mode with a user-reviewed exit (`plan`); the
`search`/`list` tools (`search`); and a Git Bash shell with owned process trees on
Windows, Rig's shell elsewhere (`shell`). The terminal is [`tui`](src/tui/SPEC.md)
and delegation is [`subagents`](src/subagents/SPEC.md).

`workbench_status` is a contract, not a feature: the per-agent display snapshot
(`PythonState`, `MemoryState`, `WorkbenchStatus`) that the optional Python
component publishes and the footer renders. It lives here because the display side
owns what is displayed — the terminal never starts a worker or probes
dependencies to learn a status, and a recipe without Python publishes nothing.

## Boundary

Depends on Rig (`rig-harness`, `rig-basics`, and `rig-tui` behind the `tui`
feature) at the recipe's pinned revision, plus `process` for process-tree
ownership. The Rig pin is not a preference: an agent holds one copy of each Rig
crate, so a second revision would duplicate types.

Owns the plugins and the status contract. It does not own the agent loop, tool
execution, session storage, compaction, the editor or the terminal's native
widgets; it does not add a second discovery catalog (skills, templates and context
files keep one loader each, the Rig/Pi path); and Plan guidance never changes the
tool catalog or permissions.

Invariants: trust gates project-owned resources only — context files load
regardless, as in Pi; an interrupted workflow is not replayed on resume; and the
`python` component is the one consumer that depends on this crate for the status
contract, never the reverse.
