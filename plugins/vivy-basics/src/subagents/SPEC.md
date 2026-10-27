---
id: subagents
type: submodule-design
status: draft
title: Native delegation (`plugins/vivy-basics/src/subagents`)
parent: basics
---

## Responsibility

Vivy's delegation surface over Rig's native subagent lifecycle — Rig keeps the
execution, reports, usage and child sessions; this module adds the shape Pi
examples ask for.

`task`, `message`, `wait` are Rig's. `workflows` adds two finite forms over the
same `SpawnSubagent` effect: `task_parallel` (1–8 tasks, at most four children
running per session including follow-ups) and `task_chain` (each step's bounded
answer substituted at `{previous}`, stopping on the first unsuccessful step). A
workflow report is claimed by the parent before the parent's inbox accepts
anything, so the parent receives exactly one aggregate; full child transcripts stay
in Rig and inline answers are capped with a reference to the child's own view.

`roles` snapshots a role at spawn from the built-in scout/planner/reviewer/worker
set, overridden by home `agents/*.md` and then trusted project `.rig/agents/*.md`;
a tool selection can never exceed the parent's access or the selected role, and a
child's `cwd` is canonicalized inside the project and applied through WorkDir
without changing the process directory. An invalid override disables that role
rather than degrading to another. `admission` bounds capacity: at capacity a root
request queues, while a nested spawn is refused outright, because queueing it is
what deadlocks a parent waiting on its own child. `ui` renders role, progress,
readable answers, child views and usage in the terminal.

## Boundary

Sits inside `basics` and depends on Rig's `subagents` module for every execution
primitive; it adds policy, roles, bounded workflows and presentation, nothing that
runs an agent itself.

What it deliberately is not: a worker process, an OS sandbox, a permission
platform, a cross-project context reload, or an automatic Git worktree. Children
share the host process, project instructions, skills and trust.

Invariants: cancellation stops running children and cancels pending launches and
follow-ups together; interrupted work is not replayed on resume; a restored child
keeps the role prompt it was spawned with.
