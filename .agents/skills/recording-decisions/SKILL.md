---
name: recording-decisions
description: Use when a change alters what this project does or why — a feature, a behavior change, a bug fix, or a choice a later agent would otherwise have to reconstruct from the code. Requires the owning spec to be updated in the same change, so the recorded decisions stay equal to the code.
---

# Recording decisions

The spec graph — `goal-and-requirements.md`, `docs/DESIGN.md`, and a `SPEC.md`
beside each module — is what the next agent reads instead of re-deriving intent.
It is only worth reading while it is true: **a drifted spec is worse than no spec**,
because it is believed and acted on.

## The rule

1. **Find the owner first.** Start at `goal-and-requirements.md` (root), then
   `docs/DESIGN.md`, then the nearest `SPEC.md` to the code you are touching; grep
   the area if it is unclear (`rg -l '^id:' -g 'SPEC.md' -g 'docs/DESIGN.md'` lists
   every recorded decision). If the change establishes a boundary no spec records,
   create one beside the code it describes.
2. **Update it in the same change.** The decision, boundary, invariant, limitation
   and the *why* go in the spec — not in a code comment (at most a one-line pointer
   where mis-editing would silently break something), not in a commit message, not
   in a PR body. Work is not done because the code changed; it is done when the
   record matches the code.
3. **Surface contradictions instead of diverging.** When a change contradicts a
   recorded decision, say so and get it settled. Amend the record in place, marked
   as an amendment with its reason — never rewrite history to look consistent.
4. **Never leave a spec looking true when it is not.** If behavior changed without
   its spec — another tool, another agent, a hotfix — correct that spec or set its
   `status: stale`. Do not read past the gap and act on a stale premise.
5. **Input is not requirement.** Daily logs, research notes, plans and handoffs are
   evidence, not rules; durable decisions go into the spec tree in place, and
   history stays where it is. A fact lives in exactly one spec — anywhere else,
   link to its `id` instead of restating it.

## Inferred is not decided

Whatever the files imply rather than a decision owner confirmed stays `status:
draft` and is marked unconfirmed where it stands. Only whoever owns the decision
moves it out of `draft`; the agent that drafted the inference never promotes its
own guess, however plausible it looks.

## Done means

- [ ] The owning spec reflects the new behavior, or the change provably touches no
      documented decision
- [ ] Nothing is now stated in two places
- [ ] Every `status` is honest, and anything still inferred is marked as such
