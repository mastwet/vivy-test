---
id: tui
type: submodule-design
status: draft
title: Compact terminal presentation (`plugins/vivy-basics/src/tui`)
parent: basics
---

## Responsibility

The recipe's optional terminal front, composed from Rig's public widget API. It
composes the screen and changes its looks; it does not re-implement a widget.

The footer stays visible while typing and carries activity, the Plan marker,
model, reasoning, usage, and the stop/input hints. Its right side alternates
between telemetry and the Python/memory page every five seconds, with a narrow
screen prioritizing activity and Plan, then Python, and dropping memory before
Python. Preparation and infrastructure errors pin the workbench page; a recovery
stays readable for five seconds before rotation resumes. `looks` supplies the
welcome banner and a readable presentation for the calls that have no upstream
look of their own (local shell, search, list) while the upstream renderers keep
output and diffs.

## Boundary

Rig owns the editor, completion, scrolling, questions and session lifecycle; this
module owns only appearance and the footer composition. The footer *projects*
status items — it never overwrites the producers' items, so a value updated while
its page is hidden reappears with its latest value.

Its status input is the `basics` `workbench_status` contract, and only that: the
terminal never starts a Python worker, installs packages, queries the memory
database or performs a model call to obtain a page, and one clock alarm tied to
the footer is the only timer. A recipe without the Python component has no
workbench page and no rotation timer at all.

Invariants: `rig-tui` exists in the build only when this component is selected;
activity, Plan and stop hints survive every layout; the default identity also
works in print/JSON mode, where this module is absent.
